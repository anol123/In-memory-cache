use std::time::Instant;

struct CacheEntry{
    value : Vec<u8>,
    expiry_at: Instant,
}