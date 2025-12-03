use crate::cache::InMemoryCache;

mod cache;
fn main() {
    let cache = InMemoryCache::new();
    InMemoryCache::insert("Anol".to_string());
}
