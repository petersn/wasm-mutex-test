
use super::MyMutex;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub fn thread_entry_point(ptr_string: String) {
    let ptr: usize = ptr_string.parse().unwrap();
    // Unsafely cast the integer back into a reference.
    let mutex: &MyMutex<u64> = unsafe { &*(ptr as *const MyMutex<u64>) };
    for _ in 0..200_000 {
        let mut guard = mutex.lock().unwrap();
        *guard += 1;
    }
}

static mut MAIN_WORKER: *const web_sys::Worker = std::ptr::null();

#[wasm_bindgen]
pub fn wasm_mutex_test_init() {
    log("=== wasm_mutex_test_init ===");
    let worker = web_sys::Worker::new("./worker.js").unwrap();
    // We send this worker our module and memory, so we can share memory.
    let array = js_sys::Array::new();
    array.push(&wasm_bindgen::module());
    array.push(&wasm_bindgen::memory());
    worker.post_message(&array).unwrap();
    unsafe {
        MAIN_WORKER = Box::into_raw(Box::new(worker));
    }
}

fn send_message_to_main_worker(mutex: &MyMutex<u64>) {
    let ptr = mutex as *const MyMutex<u64> as usize;
    let ptr_string = ptr.to_string();
    unsafe {
        let main_worker = &*MAIN_WORKER;
        main_worker.post_message(&JsValue::from_str(&ptr_string)).unwrap();
    }
}

#[wasm_bindgen]
pub fn main_test() {
    log("=== main_test ===");
    let mutex: &'static MyMutex<u64> = Box::leak(Box::new(MyMutex::new(0)));
    send_message_to_main_worker(mutex);
    loop {
        let mutex_value = *mutex.lock().unwrap();
        log(&format!("mutex_value: {}", mutex_value));
        if mutex_value >= 200_000 {
            break;
        }
    }
    log("=== main_test done ===");
}
