mod test;
mod preloader;

pub fn setup_logging() {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Trace),
    );
}

#[ctor::ctor]
fn safe_setup() {
    setup_logging();
    std::panic::set_hook(Box::new(move |panic_info| {
        log::error!("Thread crashed: {}", panic_info);
    }));
    main();
}

fn main() {
    log::info!("the main function of the BetterViewDistance mod is called!");
    let dir_ptr = crate::preloader::pl_get_externalFiles_dir();
    log::info!("ExternalDir: {}", dir_ptr);
}


