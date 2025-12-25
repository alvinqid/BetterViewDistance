use std::fs::{OpenOptions, create_dir_all};
use std::io::Write;
use std::path::PathBuf;
use std::ffi::CStr;
use std::os::raw::c_char;

pub fn write_test_to_external_dir() {
    unsafe {
        let dir_ptr = crate::preloader::pl_get_externalFiles_dir();
        if dir_ptr.is_null() {
            return;
        }

        let dir = match CStr::from_ptr(dir_ptr as *const c_char).to_str() {
            Ok(v) => v,
            Err(_) => return,
        };

        let base = PathBuf::from(dir);

        // pastikan folder ada
        let _ = create_dir_all(&base);

        let mut path = base;
        path.push("mod_test.txt");

        let text = "Test berhasil dari: mod BetterViewDistance\n";

        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)
        {
            let _ = file.write_all(text.as_bytes());
        }
    }
}
