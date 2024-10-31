use nogine2_core::{log::init_log, log_info, main_thread::set_main_thread};

pub fn init_nogine2() {
    init_log();
    set_main_thread();

    //window_subscribe_pre_tick(|_| log_warn!("Pre tick"));
    //window_subscribe_post_tick(|_| log_error!("Post tick"));

    log_info!("NOGINE2: Nogine initialized")
}
