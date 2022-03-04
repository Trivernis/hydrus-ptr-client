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
    let ptr_url_env = env::var("PTR_URL").ok();
    let ptr_key_env = env::var("PTR_ACCESS_KEY").ok();

    let mut builder = Client::builder().accept_invalid_certs(true);
    if let Some(url) = ptr_url_env {
        builder = builder.endpoint(url);
    }
    if let Some(key) = ptr_key_env {
        builder = builder.access_key(key);
    }
    builder.build().unwrap()
}
