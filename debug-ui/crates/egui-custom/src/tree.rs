use std::collections::{
    hash_map::{Iter, IterMut},
    HashMap,
};
use std::hash::Hash;

/// A node in the tree.
/// Each node has its associated value `V`, its optional parent node
/// and a list of child nodes.
pub struct Node<K, V> {
    pub value: V,
    pub parent: Option<K>,
    pub children: Vec<K>,
}
/// A tree like data structure that uses a hash map internaly to store a set of nodes.
/// A node is identified by their node id `K`
pub struct Tree<K, V> {
    map: HashMap<K, Node<K, V>>,
}

#[allow(dead_code)]
impl<K, V> Tree<K, V>
where
    K: Eq + Hash + Copy + Clone,
{
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, id: K, value: V) {
        self.map.insert(
            id,
            Node {
                value,
                parent: None,
                children: Vec::new(),
            },
        );
    }

    pub fn remove(&mut self, id: K) {
        if let Some(node) = self.map.remove(&id) {
            // remove node from parent
            if let Some(parent_id) = node.parent {
                if let Some(parent) = self.map.get_mut(&parent_id) {
                    parent.children.retain(|node_id| *node_id != id);
                }
            }
            // Remove all children from map
            for child_id in node.children.iter() {
                self.map.remove(child_id);
            }
        }
    }

    pub fn add_child_to_parent(&mut self, child_id: K, parent_id: K) {
        if let Some(parent) = self.map.get_mut(&parent_id) {
            parent.children.push(child_id);
        }
        if let Some(child) = self.map.get_mut(&child_id) {
            child.parent = Some(parent_id);
        }
    }

    /// An iterator visiting all nodes in the tree.
    pub fn nodes(&self) -> Iter<K, Node<K, V>> {
        self.map.iter()
    }
    /// An iterator visiting all nodes in the tree with a mutable reference.
    pub fn nodes_mut(&mut self) -> IterMut<K, Node<K, V>> {
        self.map.iter_mut()
    }

    // An iterator visiting all values stored in the tree.
    pub fn values(&self) -> impl Iterator<Item = &V> + '_ {
        self.map.values().map(|node| &node.value)
    }
    // An iterator visiting all values stored in the tree with a mutable reference.
    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut V> + '_ {
        self.map.values_mut().map(|node| &mut node.value)
    }

    /// Return a reference to the value for a given node id.
    pub fn get(&self, id: &K) -> Option<&V> {
        self.map.get(id).map(|node| &node.value)
    }
    /// Return a mutable reference to the value for a given node id.s
    pub fn get_mut(&mut self, id: &K) -> Option<&mut V> {
        self.map.get_mut(id).map(|node| &mut node.value)
    }

    /// Return a reference to the node for the given node id.
    pub fn get_node(&self, id: &K) -> Option<&Node<K, V>> {
        self.map.get(id)
    }
    /// Return a mutable reference to the node for the given node id.
    pub fn get_node_mut(&mut self, id: &K) -> Option<&mut Node<K, V>> {
        self.map.get_mut(id)
    }

    /// Return `true` if there are no nodes in the tree.
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}
