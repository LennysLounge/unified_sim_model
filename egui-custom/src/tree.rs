use std::collections::{
    hash_map::{Iter, IterMut, Values, ValuesMut},
    HashMap,
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct NodeId(usize);

impl NodeId {
    #[allow(dead_code)]
    pub fn add<T>(&self, other: NodeId, tree: &mut Tree<T>) {
        if let Some(me) = tree.map.get_mut(self) {
            me.children.push(other);
        }
        if let Some(other) = tree.map.get_mut(&other) {
            other.parent = Some(*self);
        }
    }
}

pub struct Node<T> {
    pub value: T,
    pub parent: Option<NodeId>,
    pub children: Vec<NodeId>,
}

pub struct Tree<T> {
    map: HashMap<NodeId, Node<T>>,
    id_count: usize,
}

#[allow(dead_code)]
impl<T> Tree<T> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            id_count: 0,
        }
    }
    pub fn new_node(&mut self, value: T) -> NodeId {
        let id = NodeId(self.id_count);
        self.id_count += 1;
        self.map.insert(
            id,
            Node {
                value,
                parent: None,
                children: Vec::new(),
            },
        );
        id
    }

    pub fn remove(&mut self, id: NodeId) {
        if let Some(node) = self.map.remove(&id) {
            // remove node from parent
            if let Some(parent_id) = node.parent {
                if let Some(parent) = self.map.get_mut(&parent_id) {
                    parent.children.retain(|node_id| *node_id != id);
                }
            }
            // Remove all children from map
            for child_id in node.children.iter() {
                self.map.remove(&child_id);
            }
        }
    }

    pub fn iter(&self) -> Iter<NodeId, Node<T>> {
        self.map.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<NodeId, Node<T>> {
        self.map.iter_mut()
    }

    pub fn values(&self) -> Values<NodeId, Node<T>> {
        self.map.values()
    }
    pub fn values_mut(&mut self) -> ValuesMut<NodeId, Node<T>> {
        self.map.values_mut()
    }

    pub fn get(&self, id: &NodeId) -> Option<&Node<T>> {
        self.map.get(id)
    }
    pub fn get_mut(&mut self, id: &NodeId) -> Option<&mut Node<T>> {
        self.map.get_mut(id)
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}
