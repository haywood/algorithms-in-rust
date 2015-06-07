use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug)]
pub struct Trie<K: Eq + Hash + Clone, V> {
    children: HashMap<K, Trie<K, V>>,
    payload: Option<V>
}

impl <K: Eq + Hash + Clone, V> Trie<K, V> {
    pub fn new() -> Trie<K, V> {
        Trie {
            children: HashMap::new(),
            payload: None
        }
    }

    pub fn find(&self, key: &[K]) -> Option<&V> {
        let mut curr = self;
        for k in key {
            match curr.children.get(k) {
                None => return None,
                Some(child) => curr = child
            }
        }
        match curr.payload {
            None => None,
            Some(ref p) => Some(p)
        }
    }

    pub fn contains(&self, key: &[K]) -> bool {
        self.find(key).is_some()
    }

    pub fn insert(&mut self, mut key: &[K], value: V) {
        let mut curr = self;
        for k in key {
            if !curr.children.contains_key(k) {
                curr.children.insert(k.clone(), Trie::new());
            }
            curr = curr.children.get_mut(k).unwrap();
        }
        curr.payload = Some(value);
    }

    pub fn remove(&mut self, key: &[K]) -> Option<V> {
        panic!("TODO")
    }
}

#[test]
fn test_trie() {
    let key = b"Odobenus rosmarus";
    let value = "Walrus";
    let mut trie = Trie::new();
    trie.insert(key, value);
    assert_eq!(trie.find(key), Some(&value));
    assert_eq!(trie.remove(key), Some(value));
    assert_eq!(trie.find(key), None);
    assert_eq!(trie.remove(key), None);
    trie.insert(key, value);
    assert_eq!(trie.find(key), Some(&value));
}
