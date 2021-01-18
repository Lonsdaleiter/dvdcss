use dvdcss::DVD;
use std::ffi::CStr;

fn main() {
    let _ = DVD::new(CStr::from_bytes_with_nul(b"/dev/sr0\0").unwrap()).expect("No DVD was found.");
}
