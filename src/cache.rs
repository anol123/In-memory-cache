use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

#[derive(Clone)]
pub struct CacheEntry {
    value: Vec<u8>,
    expiry_at: Instant,
}

pub struct InMemoryCache {
    store: Arc<Mutex<HashMap<String, CacheEntry>>>,
    shutdown: Arc<Mutex<bool>>,
    cleanup_thread: Option<std::thread::JoinHandle<()>>,
}

impl Clone for InMemoryCache {
    fn clone(&self) -> Self {
        Self {
            store: Arc::clone(&self.store),
            shutdown: Arc::clone(&self.shutdown),
            cleanup_thread: None, // ❗ DO NOT CLONE THREAD
        }
    }
}

impl InMemoryCache {
    pub fn new(cleanup_interval: Duration) -> Self {
        let store = Arc::new(Mutex::new(HashMap::<String, CacheEntry>::new()));
        let shutdown = Arc::new(Mutex::new(false));

        let store_clone = Arc::clone(&store);
        let shutdown_clone = Arc::clone(&shutdown);

        let handle = std::thread::spawn(move || {
            loop {
                std::thread::sleep(cleanup_interval);

                // Check if main cache signaled shutdown
                if *shutdown_clone.lock().unwrap() {
                    break; // <-- Exit the cleanup thread
                }

                // Remove expired entries
                let mut map = store_clone.lock().unwrap();
                let now = Instant::now();
                map.retain(|_, entry| entry.expiry_at > now);
            }
        });

        Self {
            store,
            shutdown,
            cleanup_thread: Some(handle),
        }
    }

    //Notice: insert() method signature no longer needs &mut self because Mutex handles interior mutability.
    pub fn insert(&self, key: &str, value: Vec<u8>, ttl: Duration) {
        //let expires_at = Instant::now() + ttl;
        let entry = CacheEntry {
            value,
            expiry_at: Instant::now() + ttl,
        };
        self.store.lock().unwrap().insert(key.to_string(), entry);
        println!("Insertion done successfully!!!")
    }

    pub fn get(&self, key: &str) -> Option<Vec<u8>> {
        self.store.lock().unwrap().get(key).and_then(|entry| {
            if Instant::now() < entry.expiry_at {
                Some(entry.value.clone())
            } else {
                None
            }
        })
    }

    pub fn remove(&mut self, key: &str) {
        self.store.lock().unwrap().remove(key);
    }
}