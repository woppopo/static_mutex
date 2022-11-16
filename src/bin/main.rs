use static_mutex::{Id, Log, StaticMutex};

extern crate static_mutex;

fn main() {
    let log = Log::new();
    let a1 = StaticMutex::<{ Id::new() }, _>::new(1);
    let a2 = StaticMutex::<{ Id::new() }, _>::new(2);

    let (log, v1) = a1.lock(log).unwrap();
    let (log, _v2) = a2.lock(log).unwrap();
    //let (log, v1) = a1.lock(log).unwrap(); // Double-Locks!

    let log = StaticMutex::unlock(log, v1);
    let (_log, _v1) = a1.lock(log).unwrap();
}
