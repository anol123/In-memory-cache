use std::{thread, time::Duration};

use crate::cache::InMemoryCache;

mod cache;
fn main() {
    println!("<-----Wrong code------>")
    let cache = InMemoryCache::new(Duration::from_secs(1));
    //cache.insert("Anol".to_string(),b"CSE".to_vec(), Duration::from_secs(5));
    // cache.insert("abc".to_string(),b"NOC".to_vec(), Duration::from_secs(6));
    // cache.insert("xyz".to_string(),b"we".to_vec(), Duration::from_secs(5));
    //cache.insert("pqr".to_string(),b"vbvb".to_vec(), Duration::from_secs(15));
    //cache.insert("Anol".to_string(),b"CSE".to_vec(), Duration::from_secs(5));

    // println!("{:?}",cache.get("Anol"));
    // println!("{:?}",cache.get("pqr"));


    let c1 = cache.clone();
    let t1 = thread::spawn(move||{
        c1.insert("Anol", b"CSE".to_vec(), Duration::from_secs(5));
    });

    let c2 = cache.clone();
    let t2 = thread::spawn(move||{
        let msg = c2.get("Anol");
        println!("Thread2 get : {:?}",msg);
    });
    
    let c3 = cache.clone();
    let t2 = thread::spawn(move||{
        c3.insert("msg", b"apicall".to_vec(), Duration::from_secs(15));
    });

    let c4 = cache.clone();
    let c4 = thread::spawn(move||{
        //let data = c4.get("msg");
        println!("Thread4 get : {:?}",c4.get("msg"));
    });

    t1.join().unwrap();
    t2.join().unwrap();
}