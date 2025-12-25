use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use std::ffi::CStr;

fn uuid_v4() -> String {
    let t = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    let mut b = [0u8; 16];
    for i in 0..16 {
        b[i] = ((t >> (i * 8)) & 0xff) as u8;
    }

    b[6] = (b[6] & 0x0f) | 0x40; // version 4
    b[8] = (b[8] & 0x3f) | 0x80; // variant

    format!(
        "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        b[0], b[1], b[2], b[3],
        b[4], b[5],
        b[6], b[7],
        b[8], b[9],
        b[10], b[11], b[12], b[13], b[14], b[15]
    )
}

pub fn write_uuid_to_external_dir() {
    unsafe {
        let dir_ptr = crate::preloader::pl_get_externalFiles_dir();
        if dir_ptr.is_null() {
            return;
        }

        let dir = match CStr::from_ptr(dir_ptr).to_str() {
            Ok(v) => v,
            Err(_) => return,
        };

        let mut path = PathBuf::from(dir);
        path.push("mod_uuid.txt");

        // Kalau sudah ada → jangan overwrite
        if path.exists() {
            return;
        }

        let uuid = uuid_v4();

        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .write(true)
            .open(path)
        {
            let _ = file.write_all(uuid.as_bytes());
        }
    }
}
