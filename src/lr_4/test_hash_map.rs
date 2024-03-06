use super::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_map () {
        let mut map: HashMap<&str, &str> = HashMap::new();
        map.insert("key1", "value1");
        map.insert("key2", "value2");
        map.insert("key3", "value3");

        println!("{:?}", map.get(&"key1")); // Some("value1")
        println!("{:?}", map.get(&"key2")); // Some("value2")
        println!("{:?}", map.get(&"key3")); // Some("value3")

        map.remove(&"key2");
        println!("{:?}", map.get(&"key2")); // None
    }
}