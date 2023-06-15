//! This is my attempt at creating a single producer multiple consumer channel with broadcasting.
//! Every consumer will receive every message.
//!
//! At the time of writing this there are no good synchronous broadcasting channel crates available.
//! There is a bus crate from Jon Gjengset but that is pratically abandoned and has cpu usage
//! problems.
//! There is a mpmc channel from crossbeam_channel but that does not support broadcasting.
//! There are a number of async broadcasting channels however i dont really need them to be
//! ansynchronous and including an async runtime just for broadcasting channels is not great either.
//!
//! This implementation tries to stay close to the ``mpsc::channel` from the standard library.
//!
//! Every broadcasting sender has a list of sender and receiver pairs. One pari for each
//! broadcasting receiver. Sending a message simply sends a message on each sender receiver pair.
//! The list is implemented as simple vector where each element has a unique id. This is the simplest
//! solution right now. A linked list would be better to get better insert/remove performance.
//!

use std::{
    sync::{
        mpsc::{self, Iter, RecvError, RecvTimeoutError, TryIter, TryRecvError},
        Arc, Mutex,
    },
    time::Duration,
};

/// Creates a new broadcasting sender receiver pair.
pub fn channel<T: Clone>() -> (Sender<T>, Receiver<T>) {
    let mut sender_list = SenderList::new();
    let (index, receiver) = sender_list.add();
    let sender_list = Arc::new(Mutex::new(sender_list));

    (
        Sender {
            sender_list: sender_list.clone(),
        },
        Receiver {
            index,
            receiver,
            sender_list,
        },
    )
}

/// A broadcasting sender.
#[derive(Clone)]
pub struct Sender<T: Clone> {
    sender_list: Arc<Mutex<SenderList<T>>>,
}

/// A broadcasting receiver.
pub struct Receiver<T> {
    index: i32,
    receiver: mpsc::Receiver<T>,
    sender_list: Arc<Mutex<SenderList<T>>>,
}

struct SenderList<T> {
    list: Vec<(i32, mpsc::Sender<T>)>,
    index: i32,
}

impl<T> SenderList<T> {
    fn new() -> Self {
        Self {
            list: Vec::new(),
            index: 0,
        }
    }

    fn add(&mut self) -> (i32, mpsc::Receiver<T>) {
        let (tx, rx) = mpsc::channel();
        self.index += 1;
        self.list.push((self.index, tx));
        (self.index, rx)
    }

    fn remove(&mut self, index: i32) {
        self.list.retain(|(i, _)| i != &index);
    }
}

impl<T: Clone> Sender<T> {
    /// Send a value on this channel.
    ///
    /// Different from the `mpsc::Sender` this sender will never fail. If there are no
    /// receivers to receive the value then the value is lost.
    /// This sender might briefly block the current thread if a receiver is cloned at the same time.
    /// Such a block should only be for a very brief while. However it should be avoided to clone
    /// receivers very rapidly to avoid any blocking.
    pub fn send(&self, value: T) {
        let sender_list = self
            .sender_list
            .lock()
            .expect("This mutex should never be poisoned");
        for (_, sender) in sender_list.list.iter() {
            sender
                .send(value.clone())
                .expect("The receiver should be alive");
        }
    }
}

impl<T> Receiver<T> {
    /// Delegate to [`mpsc::Receiver`]
    ///
    /// Attempts to wait for a value on this receiver, returning an error if the
    /// corresponding channel has hung up.
    ///
    /// This function will always block the current thread if there is no data
    /// available and it's possible for more data to be sent (at least one sender
    /// still exists). Once a message is sent to the corresponding [`Sender`],
    /// this receiver will wake up and return that
    /// message.
    ///
    /// If the corresponding [`Sender`] has disconnected, or it disconnects while
    /// this call is blocking, this call will wake up and return [`Err`] to
    /// indicate that no more messages can ever be received on this channel.
    /// However, since channels are buffered, messages sent before the disconnect
    /// will still be properly received.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::mpsc;
    /// use std::thread;
    ///
    /// let (send, recv) = mpsc::channel();
    /// let handle = thread::spawn(move || {
    ///     send.send(1u8).unwrap();
    /// });
    ///
    /// handle.join().unwrap();
    ///
    /// assert_eq!(Ok(1), recv.recv());
    /// ```
    ///
    /// Buffering behavior:
    ///
    /// ```
    /// use std::sync::mpsc;
    /// use std::thread;
    /// use std::sync::mpsc::RecvError;
    ///
    /// let (send, recv) = mpsc::channel();
    /// let handle = thread::spawn(move || {
    ///     send.send(1u8).unwrap();
    ///     send.send(2).unwrap();
    ///     send.send(3).unwrap();
    ///     drop(send);
    /// });
    ///
    /// // wait for the thread to join so we ensure the sender is dropped
    /// handle.join().unwrap();
    ///
    /// assert_eq!(Ok(1), recv.recv());
    /// assert_eq!(Ok(2), recv.recv());
    /// assert_eq!(Ok(3), recv.recv());
    /// assert_eq!(Err(RecvError), recv.recv());
    /// ```
    pub fn recv(&self) -> Result<T, RecvError> {
        self.receiver.recv()
    }

    /// Delegate to [`mpsc::Receiver`]
    ///
    ///  Attempts to wait for a value on this receiver, returning an error if the
    /// corresponding channel has hung up, or if it waits more than `timeout`.
    ///
    /// This function will always block the current thread if there is no data
    /// available and it's possible for more data to be sent (at least one sender
    /// still exists). Once a message is sent to the corresponding [`Sender`]
    /// (or [`SyncSender`]), this receiver will wake up and return that
    /// message.
    ///
    /// If the corresponding [`Sender`] has disconnected, or it disconnects while
    /// this call is blocking, this call will wake up and return [`Err`] to
    /// indicate that no more messages can ever be received on this channel.
    /// However, since channels are buffered, messages sent before the disconnect
    /// will still be properly received.
    ///
    /// # Examples
    ///
    /// Successfully receiving value before encountering timeout:
    ///
    /// ```no_run
    /// use std::thread;
    /// use std::time::Duration;
    /// use std::sync::mpsc;
    ///
    /// let (send, recv) = mpsc::channel();
    ///
    /// thread::spawn(move || {
    ///     send.send('a').unwrap();
    /// });
    ///
    /// assert_eq!(
    ///     recv.recv_timeout(Duration::from_millis(400)),
    ///     Ok('a')
    /// );
    /// ```
    ///
    /// Receiving an error upon reaching timeout:
    ///
    /// ```no_run
    /// use std::thread;
    /// use std::time::Duration;
    /// use std::sync::mpsc;
    ///
    /// let (send, recv) = mpsc::channel();
    ///
    /// thread::spawn(move || {
    ///     thread::sleep(Duration::from_millis(800));
    ///     send.send('a').unwrap();
    /// });
    ///
    /// assert_eq!(
    ///     recv.recv_timeout(Duration::from_millis(400)),
    ///     Err(mpsc::RecvTimeoutError::Timeout)
    /// );
    /// ```
    pub fn recv_timeout(&self, timeout: Duration) -> Result<T, RecvTimeoutError> {
        self.receiver.recv_timeout(timeout)
    }

    /// Delegate to [`mpsc::Receiver`]
    ///
    /// Attempts to return a pending value on this receiver without blocking.
    ///
    /// This method will never block the caller in order to wait for data to
    /// become available. Instead, this will always return immediately with a
    /// possible option of pending data on the channel.
    ///
    /// This is useful for a flavor of "optimistic check" before deciding to
    /// block on a receiver.
    ///
    /// Compared with [`recv`], this function has two failure cases instead of one
    /// (one for disconnection, one for an empty buffer).
    ///
    /// [`recv`]: Self::recv
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::sync::mpsc::{Receiver, channel};
    ///
    /// let (_, receiver): (_, Receiver<i32>) = channel();
    ///
    /// assert!(receiver.try_recv().is_err());
    /// ```
    pub fn try_recv(&self) -> Result<T, TryRecvError> {
        self.receiver.try_recv()
    }

    /// Delegate to [`mpsc::Receiver`]
    ///
    /// Returns an iterator that will block waiting for messages, but never
    /// [`panic!`]. It will return [`None`] when the channel has hung up.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::sync::mpsc::channel;
    /// use std::thread;
    ///
    /// let (send, recv) = channel();
    ///
    /// thread::spawn(move || {
    ///     send.send(1).unwrap();
    ///     send.send(2).unwrap();
    ///     send.send(3).unwrap();
    /// });
    ///
    /// let mut iter = recv.iter();
    /// assert_eq!(iter.next(), Some(1));
    /// assert_eq!(iter.next(), Some(2));
    /// assert_eq!(iter.next(), Some(3));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter(&self) -> Iter<'_, T> {
        self.receiver.iter()
    }

    /// Delegate to [`mpsc::Receiver`]
    ///
    /// Returns an iterator that will attempt to yield all pending values.
    /// It will return `None` if there are no more pending values or if the
    /// channel has hung up. The iterator will never [`panic!`] or block the
    /// user by waiting for values.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::mpsc::channel;
    /// use std::thread;
    /// use std::time::Duration;
    ///
    /// let (sender, receiver) = channel();
    ///
    /// // nothing is in the buffer yet
    /// assert!(receiver.try_iter().next().is_none());
    ///
    /// thread::spawn(move || {
    ///     thread::sleep(Duration::from_secs(1));
    ///     sender.send(1).unwrap();
    ///     sender.send(2).unwrap();
    ///     sender.send(3).unwrap();
    /// });
    ///
    /// // nothing is in the buffer yet
    /// assert!(receiver.try_iter().next().is_none());
    ///
    /// // block for two seconds
    /// thread::sleep(Duration::from_secs(2));
    ///
    /// let mut iter = receiver.try_iter();
    /// assert_eq!(iter.next(), Some(1));
    /// assert_eq!(iter.next(), Some(2));
    /// assert_eq!(iter.next(), Some(3));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn try_iter(&self) -> TryIter<'_, T> {
        self.receiver.try_iter()
    }
}

impl<T> Clone for Receiver<T> {
    fn clone(&self) -> Self {
        let (index, receiver) = self
            .sender_list
            .lock()
            .expect("This mutex should never be poisoned")
            .add();
        Self {
            index,
            receiver,
            sender_list: self.sender_list.clone(),
        }
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        self.sender_list
            .lock()
            .expect("Should be able to lock")
            .remove(self.index);
    }
}

#[cfg(test)]
mod test {
    use crate::broadcast::*;

    #[test]
    fn send_to_single_receiver() {
        let (sender, receiver) = channel();
        sender.send(12);

        assert_eq!(receiver.recv().unwrap(), 12);
    }

    #[test]
    fn every_receiver_receives_the_message() {
        let (sender, receiver1) = channel();
        let receiver2 = receiver1.clone();

        sender.send(12);

        assert_eq!(receiver1.recv().unwrap(), 12);
        assert_eq!(receiver2.recv().unwrap(), 12);
    }

    #[test]
    fn dropping_a_receiver_removes_it_from_the_list() {
        let (sender, receiver): (Sender<()>, Receiver<()>) = channel();
        std::mem::drop(receiver);

        assert_eq!(sender.sender_list.lock().unwrap().list.len(), 0);
    }

    #[test]
    fn sender_with_no_receivers_does_not_panic_or_block() {
        let (sender, receiver): (Sender<()>, Receiver<()>) = channel();
        std::mem::drop(receiver);

        sender.send(());
    }

    #[test]
    fn messages_are_retained_per_receiver() {
        let (sender, receiver_1) = channel();
        let receiver_2 = receiver_1.clone();

        sender.send(1);
        sender.send(2);
        sender.send(3);

        assert_eq!(receiver_1.recv().unwrap(), 1);
        assert_eq!(receiver_1.recv().unwrap(), 2);
        assert_eq!(receiver_1.recv().unwrap(), 3);
        assert_eq!(receiver_2.recv().unwrap(), 1);
        assert_eq!(receiver_2.recv().unwrap(), 2);
        assert_eq!(receiver_2.recv().unwrap(), 3);
    }
}
