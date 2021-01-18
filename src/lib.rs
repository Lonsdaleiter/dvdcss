use bitflags::_core::ffi::c_void;
use std::ffi::CStr;

mod ffi;

#[repr(i32)]
/// Describes whether to recalculate keys.
pub enum SeekOptions {
    None = 0,
    /// At the discretion of libdvdcss, checks the title key and allows for it to be recalculated.
    Mpeg = 1 << 0,
    /// Checks the title key and allows for it to be recalculated.
    Key = 1 << 1,
}

/// Information may only be loaded out of a DVD in 2048 byte increments.
pub type Block = [u8; 2048];

#[repr(transparent)]
/// An instance of dvdcss. Represents a DVD which has been loaded into a drive.
///
/// Must have its destructor run.
pub struct DVD {
    ptr: ffi::dvdcss_t,
}

impl DVD {
    /// Opens the DVD at the device location specified in the string (eg /dev/sr0).
    ///
    /// Returns `None` if there is no DVD in the drive.
    pub fn new(location: &CStr) -> Option<Self> {
        let ptr = unsafe { ffi::dvdcss_open(location.as_ptr()) };

        match !ptr.is_null() {
            true => Some(Self { ptr }),
            false => None,
        }
    }

    /// Sets the absolute position of the needle of the DVD.
    ///
    /// The value `blocks` determines in intervals of 2048 (the size of a block) the absolute
    /// location to which the needle should be moved.
    ///
    /// The `flags` behavior is documented in `SeekOptions`.
    ///
    /// Returns `Ok` and the new position in blocks if the operation succeeded, and
    /// `Err` if it did not.
    pub fn seek(&self, blocks: u32, flags: SeekOptions) -> Result<u32, ()> {
        let val = unsafe { ffi::dvdcss_seek(self.ptr, blocks as i32, flags as i32) };
        match val >= 0 {
            true => Ok(val as u32),
            false => Err(()),
        }
    }

    /// Fills each `Block` of `buffer` with the raw contents of the DVD from the current
    /// position of the needle, which is modifiable with the `seek` function.
    ///
    /// The `decrypt` boolean should be enabled when the DVD is encrypted.
    ///
    /// Returns `Ok` and the accompanying number of blocks read if the operation
    /// succeeded, and `Err` if it did not.
    pub fn read(&self, buffer: &mut [Block], decrypt: bool) -> Result<u32, ()> {
        let val = unsafe {
            ffi::dvdcss_read(
                self.ptr,
                buffer.as_mut_ptr() as *mut c_void,
                buffer.len() as i32,
                // if decrypt == 0, uses empty flag 0, else uses set flag 1; this is valid
                decrypt as i32,
            )
        };
        match val >= 0 {
            true => Ok(val as u32),
            false => Err(()),
        }
    }

    /// Returns whether the DVD being read is scrambled.
    pub fn is_scrambled(&self) -> bool {
        match unsafe { ffi::dvdcss_is_scrambled(self.ptr) } {
            0 => false,
            1 => true,
            _ => unreachable!(),
        }
    }

    /// Returns text describing the latest error encountered by dvdcss.
    ///
    /// Output is undefined if no error was encountered, but should not crash to read.
    pub fn latest_error(&self) -> &CStr {
        unsafe { CStr::from_ptr(ffi::dvdcss_error(self.ptr)) }
    }
}

impl Drop for DVD {
    fn drop(&mut self) {
        let val = unsafe { ffi::dvdcss_close(self.ptr) };
        assert_eq!(val, 0);
    }
}
