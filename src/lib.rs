
#[cfg(target_arch = "wasm32")]
pub mod web;

pub type MyLockResult<'a, T> = Result<
    MyMutexGuard<'a, T>,
    std::sync::PoisonError<std::sync::MutexGuard<'a, T>>,
>;

pub struct MyMutex<T: ?Sized> {
    lock: std::sync::Mutex<T>,
}

pub struct MyMutexGuard<'a, T: ?Sized + 'a> {
    guard: std::sync::MutexGuard<'a, T>,
}

impl<T> MyMutex<T> {
    pub fn new(data: T) -> Self {
        MyMutex {
            lock: std::sync::Mutex::new(data),
        }
    }
}

impl<T: ?Sized> MyMutex<T> {
    pub fn lock(&self) -> MyLockResult<T> {
        match self.lock.lock() {
            Ok(guard) => Ok(MyMutexGuard { guard }),
            Err(err) => Err(err),
        }
    }
}

impl<T: ?Sized> std::ops::Deref for MyMutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        &*self.guard
    }
}

impl<T: ?Sized> std::ops::DerefMut for MyMutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut *self.guard
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contention() {
        let mutex: &'static MyMutex<u64> = Box::leak(Box::new(MyMutex::new(0)));
        let threads: Vec<_> = (0..20)
            .map(|_| {
                std::thread::spawn(move || {
                    for _ in 0..200_000 {
                        let mut guard = mutex.lock().unwrap();
                        *guard += 1;
                    }
                })
            })
            .collect();
        for thread in threads {
            thread.join().unwrap();
        }
        let guard = mutex.lock().unwrap();
        assert_eq!(*guard, 4_000_000);
    }

    #[test]
    fn test_both_sides_blocking() {
        let mutex: &'static MyMutex<()> = Box::leak(Box::new(MyMutex::new(())));
        // First, we take the mutex, and make the other side wait.
        let take_at = std::time::Instant::now();
        let guard = mutex.lock().unwrap();
        let take_duration = take_at.elapsed();
        // Assert that we got the lock quickly on the main thread.
        assert!(take_duration < std::time::Duration::from_millis(10));
        let thread = std::thread::spawn(move || {
            let take_at = std::time::Instant::now();
            let guard = mutex.lock().unwrap();
            let take_duration = take_at.elapsed();
            // Assert that we got the lock slowly on the other thread.
            assert!(take_duration > std::time::Duration::from_millis(90));
            std::thread::sleep(std::time::Duration::from_millis(100));
            drop(guard);
        });
        std::thread::sleep(std::time::Duration::from_millis(100));
        drop(guard);
        std::thread::sleep(std::time::Duration::from_millis(1));
        let take_at = std::time::Instant::now();
        let guard = mutex.lock().unwrap();
        let take_duration = take_at.elapsed();
        // Assert that we got the lock slowly on the main thread.
        assert!(take_duration > std::time::Duration::from_millis(90));
        drop(guard);
        thread.join().unwrap();
    }
}
