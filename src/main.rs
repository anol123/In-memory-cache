use std::time::Duration;

use crate::cache::InMemoryCache;

mod cache;
fn main() {
    let mut cache = InMemoryCache::new();
    cache.insert("Anol".to_string(),b"CSE".to_vec(), Duration::from_secs(5));

    println!("{:?}",cache.get("Anol"));
}
