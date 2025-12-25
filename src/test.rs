use std::fs::{OpenOptions, create_dir_all};
use std::io::Write;
use std::path::PathBuf;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::ffi::CString;

fn android_log(msg: &str) {
    unsafe {
        let tag = CString::new("RustMod").unwrap();
        let msg = CString::new(msg).unwrap();
        libc::__android_log_print(
            libc::ANDROID_LOG_INFO,
            tag.as_ptr(),
            msg.as_ptr(),
        );
    }
}

pub fn write_test_to_external_dir() {
    unsafe {
        let dir_ptr = crate::preloader::pl_get_externalFiles_dir();
        if dir_ptr.is_null() {
            android_log("externalFilesDir is NULL");
            return;
        }

        let dir = match CStr::from_ptr(dir_ptr as *const c_char).to_str() {
            Ok(v) => v,
            Err(_) => {
                android_log("Failed to convert externalFilesDir to str");
                return;
            }
        };

        let base = PathBuf::from(dir);

        // Pastikan direktori ada
        if let Err(e) = create_dir_all(&base) {
            android_log(&format!("Failed to create dir: {:?}", e));
            return;
        }

        let mut path = base;
        path.push("mod_test.txt");

        let content = "Test berhasil dari: mod BetterViewDistance\n";

        match OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&path)
        {
            Ok(mut file) => {
                let _ = file.write_all(content.as_bytes());
                android_log("File berhasil ditulis: mod_test.txt");
            }
            Err(e) => {
                android_log(&format!("Gagal membuka file: {:?}", e));
            }
        }
    }
}
