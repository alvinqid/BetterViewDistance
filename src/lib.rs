use std::ffi::CStr;
use std::os::raw::c_char;
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
    std::thread::sleep(std::time::Duration::from_millis(1000));
    
    unsafe {
      let dir_ptr = crate::preloader::pl_get_minecraft_data_dir();
      if !dir_ptr.is_null() {
          let dir = CStr::from_ptr(dir_ptr as *const c_char).to_string_lossy();
          log::info!("Dir: {}", dir);
      }
    }
}
