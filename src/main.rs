use rust_study::basic::index::{hs_index, hs_closures, hs_closures1, hs_thread, hs_thread1, hs_thread2, hs_thread3, hs_thread4, hs_thread5, hs_thread6, hs_smart_pointer, hs_smart_pointer2, hs_smart_pointer3, hs_smart_pointer5};

#[tokio::main]
async fn main() {
    // hs_index();
    // hs_thread();
    // hs_thread1();
    // hs_thread2();
    // hs_thread6();
    // hs_closures();
    // hs_closures1();
    // println!("{}", "hihi!");
    // hs_smart_pointer();
    // hs_smart_pointer2();
    // hs_smart_pointer3();
    hs_smart_pointer5();
    // worker().await;
    // tokio::task::spawn(worker());
    // println!("hhhhhhhhhhhhhhhhhhhhhhhhhhhh");
    // for i in (1..40) {
    //     println!("{}", i);
    // }
}

async fn worker() {
    println!("hihi!!")
}

