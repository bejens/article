use std::sync::{Arc, Mutex};

use snowflake_rust::Snowflake;

pub fn id() -> Option<i64> {
    let instance = get_instance();
    let mut sf = instance.lock().unwrap();
    sf.generate()
}

// singleton
fn get_instance() -> Arc<Mutex<Snowflake>> {

    static mut SINGLETON: Option<Arc<Mutex<Snowflake>>> = None;

    unsafe {
        SINGLETON.get_or_insert_with( || {
            Arc::new(Mutex::new(Snowflake::kubernetes()))
        }).clone()
    }
}

