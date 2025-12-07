use std::time::Duration;

use crate::cache::InMemoryCache;

mod cache;
fn main() {
    let mut cache = InMemoryCache::new();
    cache.insert("Anol".to_string(),b"CSE".to_vec(), Duration::from_secs(5));
    cache.insert("abc".to_string(),b"NOC".to_vec(), Duration::from_secs(6));
    cache.insert("xyz".to_string(),b"we".to_vec(), Duration::from_secs(5));
    cache.insert("pqr".to_string(),b"vbvb".to_vec(), Duration::from_secs(15));

    println!("{:?}",cache.get("Anol"));
}
