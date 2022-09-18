use static_mutex::{IDs, StaticMutex, ID};

extern crate static_mutex;

fn main() {
    let ids = IDs::empty();
    let a1 = StaticMutex::<_, { ID::new() }>::new(1);
    let a2 = StaticMutex::<_, { ID::new() }>::new(2);

    let (ids, v1) = a1.lock(ids).unwrap();
    let (ids, _v2) = a2.lock(ids).unwrap();
    //let (ids, v1) = a1.lock(ids).unwrap(); // Double-Locks!

    let ids = StaticMutex::unlock(ids, v1);
    let (_ids, _v1) = a1.lock(ids).unwrap();
}
