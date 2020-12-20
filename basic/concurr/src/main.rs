/// * Mutax(mutual exclusion)
/// - You must acquire a lock before using the data.
///     - and after use, unlock 
// Arc(Artomic Rerference Count) = 성능손실이 조금 있기때문에 다중 스레드에서 Rc 사용시에만 사용
use std::sync::{Mutex, Arc};

fn main() {
    // for Multi(OW) 다중소유권
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = std::thread::spawn(move || {
            // acquire a lock
            // return LockResult as MutextGuard Type as Smart Pointer
            // and this pointer is implemented Deref trait
            let mut num = counter.lock().unwrap();
            
            // dereference(역참조) 여러 스레드가 공유가능한 데이터를 락을통해
            // 얻어왔다면 사용 가능하다.
            *num += 1;            

            // 범위를 벗어나면 unlock
        });
        handles.push(handle);
    }
    for handle in handles {
        // 모든 자식 스레드가 종료되기를 기다린다.
        handle.join().unwrap();
    }

    println!("Result : {}", *counter.lock().unwrap());
}
