use std::collections::{
    hash_map::{Iter, IterMut, Values, ValuesMut},
    HashMap,
};
use std::hash::Hash;

pub struct Node<K, V> {
    pub value: V,
    pub parent: Option<K>,
    pub children: Vec<K>,
}

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
                self.map.remove(&child_id);
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

    pub fn iter(&self) -> Iter<K, Node<K, V>> {
        self.map.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<K, Node<K, V>> {
        self.map.iter_mut()
    }

    pub fn values(&self) -> Values<K, Node<K, V>> {
        self.map.values()
    }
    pub fn values_mut(&mut self) -> ValuesMut<K, Node<K, V>> {
        self.map.values_mut()
    }

    pub fn get(&self, id: &K) -> Option<&V> {
        self.map.get(id).map(|node| &node.value)
    }
    pub fn get_mut(&mut self, id: &K) -> Option<&mut V> {
        self.map.get_mut(id).map(|node| &mut node.value)
    }

    pub fn get_node(&self, id: &K) -> Option<&Node<K, V>> {
        self.map.get(id)
    }
    pub fn get_node_mut(&mut self, id: &K) -> Option<&mut Node<K, V>> {
        self.map.get_mut(id)
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}
