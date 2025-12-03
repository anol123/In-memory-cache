use std::{collections::HashMap, sync::{Arc, Mutex}, time::Instant};

pub struct CacheEntry{
    value : Vec<u8>,
    expiry_at: Instant,
}
pub struct InMemoryCache{
    store: Arc<Mutex<HashMap<String, CacheEntry>>>,
}
impl InMemoryCache{
    pub fn new(){
        
    }
    pub fn insert(key:String){
        println!("Insert called with key {}",key);
    }
}
