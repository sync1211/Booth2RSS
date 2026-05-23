use std::collections::HashMap;
use std::time::{SystemTime, Duration};

pub struct CachedValue {
    value: String,
    timestamp: SystemTime
}


pub struct ResponseCache {
    cache_dict: HashMap<String, CachedValue>,
    max_age: Duration,
    max_size: usize
}

impl ResponseCache {
    pub fn with_defaults() -> ResponseCache {
        return ResponseCache::new(50, Duration::from_mins(15));
    }

    pub fn new(max_size: usize, max_age: Duration) -> ResponseCache {
        return ResponseCache {
            cache_dict: HashMap::with_capacity(max_size),
            max_age,
            max_size
        }
    }

    pub fn add_item(&mut self, key: String, value: String) {
        let entry = CachedValue {
            value,
            timestamp: SystemTime::now()
        };

        self.ensure_size();

        self.cache_dict.insert(key, entry);
    }

    fn ensure_size(&mut self){
        //NOTE: Ensures that there is space for at least one more item
        if self.cache_dict.len() < self.max_size {
            return;
        }

        self.prune();

        while self.cache_dict.len() >= self.max_size {
            self.remove_oldest(); //TODO: calculate the number of entries to remove
        }
    }

    pub fn get_item(&self, key: &String) -> Option<&CachedValue> {
        return self.cache_dict.get(key);
    }

    pub fn get_active_value(&self, key: &String) -> Option<&String> {
        let entry = self.get_item(key)?;
        
        return match &self.is_expired(entry) {
            false => Some(&entry.value),
            true => None
        };
    }

    fn is_expired(&self, entry: &CachedValue) -> bool {
        let timediff = match SystemTime::now().duration_since(entry.timestamp) {
            Ok(duration) => duration,
            Err(e) => {
                println!("Error determining time difference: {}. Assuming outdated!", e);
                return true;
            }
        };

        return timediff > self.max_age;
    }

    pub fn prune(&mut self) -> i32 {
        let mut keys_to_delete: Vec<String> = Vec::new();
        let mut count = 0;

        for (key, entry) in self.cache_dict.iter() {
            if !self.is_expired(entry) {
                continue;
            }
            
            keys_to_delete.push(key.to_owned());
        }

        for key in keys_to_delete.iter() {
            self.cache_dict.remove(key);
            count += 1;
        }

        return count;
    }

    fn remove_oldest(&mut self) {
        let mut oldest_key: Option<&String> = None;
        let mut oldest_time = SystemTime::now();
        
        for (key, entry) in self.cache_dict.iter() {
            if entry.timestamp >= oldest_time {
                continue;
            }

            oldest_time = entry.timestamp;
            oldest_key = Some(key); //TODO: improve
        }
   
        if let Some(key) = oldest_key {
            self.cache_dict.remove(&key.to_owned());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_get_active_value() {
        let key = "Testo".to_string();
        let value = "Telesto".to_string();
        
        let mut cache = ResponseCache::with_defaults();
        
        cache.add_item(key.clone(), value.clone());

        let result = cache.get_active_value(&key).unwrap();

        assert_eq!(result, &value);
    }

    #[test]
    fn test_active_value_multiple() {
        let key1 = "Testo".to_string();
        let key2 = "Telesto".to_string();
        let value1 = "foo".to_string();
        let value2 = "bar".to_string();
        
        let mut cache = ResponseCache::with_defaults();
        
        cache.add_item(key1.clone(), value1.clone());
        cache.add_item(key2.clone(), value2.clone());

        let result1 = cache.get_active_value(&key1).unwrap();
        let result2 = cache.get_active_value(&key2).unwrap();

        assert_eq!(result1, &value1);
        assert_eq!(result2, &value2);
    }


    #[test]
    fn test_get_active_value_none() {
        let key = "Testo".to_string();
        let value = "Telesto".to_string();
        
        let mut cache = ResponseCache::with_defaults();
        
        cache.add_item(key, value);

        let result = cache.get_active_value(&"nonexistium".to_string());

        assert_eq!(result, None);
    }

    #[test]
    fn test_get_active_value_expired() {
        let key = "Testo".to_string();
        let value = "Telesto".to_string();
        
        let mut cache = ResponseCache::new(50, Duration::from_millis(200));
        
        cache.add_item(key, value);
        thread::sleep(Duration::from_millis(200));

        let result = cache.get_active_value(&"nonexistium".to_string());

        assert_eq!(result, None);
    }

    #[test]
    fn test_remove_oldest() {
        let key1 = "Testo".to_string();
        let key2 = "Telesto".to_string();
        
        let mut cache = ResponseCache::with_defaults();
        
        cache.add_item(key1.clone(), key1.clone());
        thread::sleep(Duration::from_millis(100));

        cache.add_item(key2.clone(), key2.clone());
        
        cache.remove_oldest();

        let result1 = cache.get_item(&key1);
        let result2 = cache.get_item(&key2);
        
        match result2 {
            Some(cv) => assert_eq!(cv.value, key2),
            None => assert!(false)
        };

        match result1 {
            Some(_) => assert!(false),
            None => assert!(true)
        };
    }

    #[test]
    fn test_ensure_size() {
        let key1 = "Testo".to_string();
        let key2 = "Telesto".to_string();
        let value1 = "foo".to_string();
        let value2 = "bar".to_string();
        
        let mut cache = ResponseCache::new(3, Duration::from_secs(10));
        
        cache.add_item(key1.clone(), value1.clone());
        thread::sleep(Duration::from_millis(100));
        cache.add_item(key2.clone(), value2.clone());

        cache.max_size = 2;
        cache.ensure_size();

        let result1 = cache.get_item(&key1);
        let result2 = cache.get_item(&key2);
        
        match result2 {
            Some(cv) => assert_eq!(cv.value, value2),
            None => assert!(false)
        };

        match result1 {
            Some(_) => assert!(false),
            None => assert!(true)
        };
    }

    #[test]
    fn test_prune() {
        let key1 = "Testo".to_string();
        let key2 = "Telesto".to_string();
        let value1 = "foo".to_string();
        let value2 = "bar".to_string();
        
        let mut cache = ResponseCache::new(50, Duration::from_millis(200));
        
        cache.add_item(key1.clone(), value1.clone());
        thread::sleep(Duration::from_millis(200));
        cache.add_item(key2.clone(), value2.clone());

        cache.prune();

        let result1 = cache.get_item(&key1);
        let result2 = cache.get_item(&key2);
        

        match result2 {
            Some(cv) => assert_eq!(cv.value, value2),
            None => assert!(false)
        };

        match result1 {
            Some(_) => assert!(false),
            None => assert!(true)
        };
    }

    #[test]
    fn test_is_expired() {
        let cache = ResponseCache::new(50, Duration::from_millis(200));
        
        let item = CachedValue {
            timestamp: SystemTime::now(),
            value: "Test".to_string()
        };
        
        assert!(!cache.is_expired(&item));

        thread::sleep(Duration::from_millis(200));
        
        assert!(cache.is_expired(&item));
    }
}
