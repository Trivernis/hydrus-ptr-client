use hydrus_ptr_client::Client;
use std::env;
use std::sync::{Arc, Mutex, MutexGuard};

fn setup() {
    lazy_static::lazy_static! { static ref SETUP_DONE: Arc<Mutex<bool>> = Arc::new(Mutex::new(false)); }
    let mut setup_done: MutexGuard<bool> = SETUP_DONE.lock().unwrap();

    if !*setup_done {
        dotenv::dotenv().expect("failed to initialize dotenv");
        tracing_subscriber::fmt::init();
        *setup_done = true;
    }
}

pub fn get_client() -> Client {
    setup();

    Client::builder()
        .endpoint(env::var("PTR_URL").unwrap())
        .access_key(env::var("PTR_ACCESS_KEY").unwrap())
        .accept_invalid_certs(true)
        .build()
        .unwrap()
}
