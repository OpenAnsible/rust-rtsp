
use std::string::ToString;
use std::collections::BTreeMap;

pub type Key   = String;
pub type Value = String;

#[derive(Debug)]
pub struct Headers {
    headers: BTreeMap<Key, Value>,
}

impl Headers{
    pub fn new () -> Headers {
        Headers { headers: BTreeMap::new() }
    }
    pub fn get(&self, key: Key) -> Option<&Value> {
        self.headers.get(key.as_str())
    }
    pub fn insert(&mut self, _key: Key, value: Value) -> Option<Value> {
        // If the map did not have this key present, None is returned.
        // If the map did have this key present, the value is updated, and the old value is returned.
        let key = _key.to_lowercase();
        if self.headers.contains_key(&key) {
            self.remove(&key);
        }
        self.headers.insert(key, value)
    }
    pub fn remove(&mut self, key: &Key) -> Option<Value> {
        // If the map did have this key present, returning the value at the key if the key was previously in the map
        // If the map did not have this key present, return None.
        self.headers.remove(key)
    }
    pub fn keys(&self) -> Vec<Key> {
        self.headers.keys().cloned().collect::<Vec<Key>>()
    }
    pub fn values(&self) -> Vec<Value> {
        self.headers.values().cloned().collect::<Vec<Value>>()
    }
    pub fn len(&self) -> usize {
        self.headers.len()
    }
    pub fn clear(&mut self) {
        self.headers.clear();
    }
}

impl ToString for Headers {
    fn to_string(&self) -> String {
        let mut s = String::new();
        for (key, value) in &self.headers {
            let mut key_chars = key.chars();
            let mut uppercase_key = String::new();
            uppercase_key.push_str(key_chars.nth(0).unwrap().to_string().to_uppercase().as_ref());
            uppercase_key.push_str(key_chars.as_str());
            let line = uppercase_key + ": " + value.as_ref() + "\r\n";
            s.push_str(line.as_ref())
        }
        s
    }
}

// impl Copy for Headers {}



#[test]
fn test() {
    let mut headers = Headers::new();
    headers.insert("content-type".to_string(), "video/mp4".to_string());
    headers.insert("content-length".to_string(), "512".to_string());

    assert_eq!(headers.keys(), vec!["content-length", "content-type"] );
    assert_eq!(headers.values(), vec!["512", "video/mp4"] );

    assert_eq!(headers.get("content-type".to_string()), Some(&"video/mp4".to_string()) );
    assert_eq!(headers.get("content-length".to_string()), Some(&"512".to_string()) );
    assert_eq!(headers.get("None".to_string()), None );

    assert_eq!(headers.to_string(), "Content-length: 512\r\nContent-type: video/mp4\r\n");

    headers.remove(&"content-type".to_string());
    assert_eq!(headers.get("content-type".to_string()), None );

    headers.clear();
    assert_eq!(headers.get("content-length".to_string()), None );
    assert_eq!(headers.len(), 0);
}