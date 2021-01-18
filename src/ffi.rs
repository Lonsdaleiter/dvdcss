use std::os::raw::{c_char, c_void};

#[allow(non_camel_case_types)]
pub(crate) type dvdcss_t = *mut c_void;

#[link(name = "dvdcss")]
extern "C" {
    pub(crate) fn dvdcss_open(psz_target: *const c_char) -> dvdcss_t;
    pub(crate) fn dvdcss_close(instance: dvdcss_t) -> i32;
    pub(crate) fn dvdcss_seek(instance: dvdcss_t, i_blocks: i32, i_flags: i32) -> i32;
    pub(crate) fn dvdcss_read(
        instance: dvdcss_t,
        p_buffer: *mut c_void,
        i_blocks: i32,
        i_flags: i32,
    ) -> i32;
    // pub(crate) fn dvdcss_readv(
    //     instance: dvdcss_t,
    //     p_iovec: *mut c_void,
    //     i_blocks: i32,
    //     i_flags: i32,
    // ) -> i32;
    pub(crate) fn dvdcss_error(instance: dvdcss_t) -> *const c_char;
    pub(crate) fn dvdcss_is_scrambled(instance: dvdcss_t) -> i32;
}
