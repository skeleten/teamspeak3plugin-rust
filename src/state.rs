use std::sync::{Arc,Mutex,Once,ONCE_INIT};
use std::{mem,thread};
use functions::TS3Functions;
use interface::Plugin;

#[derive(Clone)]
pub struct SingletonReader {
    // Since we will be used in many threads, we need to protect
    // concurrent access
    pub functions:      Arc<Mutex<Option<TS3Functions>>>,
    pub plugin:         Arc<Mutex<Option<Box<Plugin>>>>,
}

pub fn singleton() -> SingletonReader {
    // Initialize it to a null value
    static mut SINGLETON: *const SingletonReader = 0 as *const SingletonReader;
    static ONCE: Once = ONCE_INIT;

    unsafe {
        ONCE.call_once(|| {
            // Make it
            let singleton = SingletonReader {
                functions:      Arc::new(Mutex::new(None)),
                plugin:         Arc::new(Mutex::new(None)),
            };

            // Put it in the heap so it can outlive this call
            SINGLETON = mem::transmute(Box::new(singleton));

            // Make sure to free heap memory at exit
            /* This doesn't exist in stable 1.0, so we will just leak it!
            rt::at_exit(|| {
                let singleton: Box<SingletonReader> = mem::transmute(SINGLETON);

                // Let's explictly free the memory for this example
                drop(singleton);

                // Set it to null again. I hope only one thread can call `at_exit`!
                SINGLETON = 0 as *const _;
            });
            */
        });

        // Now we give out a copy of the data that is safe to use concurrently.
        (*SINGLETON).clone()
    }
}

pub fn set_functions(funcs: TS3Functions) {
    singleton().functions = Arc::new(Mutex::new(Some(funcs)));
}

pub fn set_implementation(imp: Box<Plugin>) {
    singleton().plugin = Arc::new(Mutex::new(Some(imp)));
}