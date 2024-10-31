use std::{sync::RwLock, thread::ThreadId};

use crate::{assert_expr, crash};

static MAIN_THREAD: RwLock<Option<ThreadId>> = RwLock::new(None);

pub fn set_main_thread() {
    let Ok(mut main_thread) = MAIN_THREAD.write() else { crash!("Couldn't access Main Thread singleton!") };
    *main_thread = Some(std::thread::current().id());
}

pub fn test_main_thread() {
    let Ok(main_thread) = MAIN_THREAD.read() else { crash!("Couldn't access Main Thread singleton!") };
    assert_expr!(Some(std::thread::current().id()) == *main_thread, "This function may only be called from the main thread!");

}
