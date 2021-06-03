use std::sync::atomic::{AtomicUsize, Ordering};

use std::sync::Arc;

static GLOBAL_ID: AtomicUsize = AtomicUsize::new(0);

pub struct MyGameEntityId {
    id: usize,
}

impl MyGameEntityId {
    pub fn new() -> MyGameEntityId {
        MyGameEntityId {
            id: Self::make_new_id(),
        }
    }

    fn make_new_id() -> usize {
        let previous_id = GLOBAL_ID.fetch_add(1, Ordering::SeqCst);
        previous_id + 1
    }
}
