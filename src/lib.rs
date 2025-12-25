mod test;
mod preloader;

#[ctor::ctor]
fn safe_setup() {
    std::panic::set_hook(Box::new(move |_panic_info| {}));
    main();
}

fn main() {
    let _ = test::write_uuid_to_external_dir();
}
