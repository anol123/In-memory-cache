use std::{collections::HashMap, sync::{Arc, Mutex}, time::{Duration, Instant}};

pub struct CacheEntry{
    value : Vec<u8>,
    expiry_at: Instant,
}
pub struct InMemoryCache{
    store: Arc<Mutex<HashMap<String, CacheEntry>>>,
}
impl InMemoryCache{
    pub fn new()->Self{
        Self { store: Arc::new(Mutex::new(HashMap::new())) }
    }
    pub fn insert(&mut self, key:String, value: Vec<u8>, ttl: Duration){
        //let expires_at = Instant::now() + ttl;
        let entry = CacheEntry{
            value, 
            expiry_at: Instant::now() + ttl };
        self.store.lock().unwrap().insert(key,entry);
    }

    pub fn get(&self, key:&str)-> Option<Vec<u8>>{
        self.store.lock().unwrap().get(key).and_then(|entry|{
            if Instant::now()< entry.expiry_at{
                Some(entry.value.clone())
            }
            else{
                None
            }
        })
    }

    pub fn remove(&mut self, key: &str){
        self.store.lock().unwrap().remove(key);
    }
}
