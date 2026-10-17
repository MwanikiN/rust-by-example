// TODO: Define a public generic struct KeyValue<K, V>
// with two private fields: key of type K and value of type V
pub struct KeyValue<K, V> {
    key: K,
    value: V
}

// TODO: Implement methods for KeyValue<K, V>:
// - new: associated function that creates a KeyValue from a key and value
// - key: method that returns a reference to the key
// - value: method that returns a reference to the value
// - set_value: method that replaces the current value with a new one

impl<K, V> KeyValue<K, V> {
    pub fn new(key: K, value: V) -> KeyValue<K, V> {
        KeyValue{
            key,
            value
        }
    }
    pub fn key(&self) -> &K {
        &self.key
    }
    pub fn value(&self) -> &V {
        &self.value
    }
    pub fn set_value(&mut self, value: V) {
        self.value = value;
    }
}
