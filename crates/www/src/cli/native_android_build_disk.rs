use std::path::Path;

#[cfg(windows)]
pub(super) fn available_free_bytes(path: &Path) -> Option<u64> {
    use std::os::windows::ffi::OsStrExt;

    let mut directory = path;
    while !directory.exists() {
        directory = directory.parent()?;
    }
    let wide_path = directory
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect::<Vec<_>>();
    let mut free_bytes_available = 0_u64;
    let mut total_bytes = 0_u64;
    let mut total_free_bytes = 0_u64;

    // SAFETY: `wide_path` is a null-terminated UTF-16 path buffer that lives for the call.
    let ok = unsafe {
        GetDiskFreeSpaceExW(
            wide_path.as_ptr(),
            &mut free_bytes_available,
            &mut total_bytes,
            &mut total_free_bytes,
        )
    };
    if ok == 0 {
        None
    } else {
        Some(free_bytes_available)
    }
}

#[cfg(not(windows))]
pub(super) fn available_free_bytes(_path: &Path) -> Option<u64> {
    None
}

#[cfg(windows)]
#[link(name = "kernel32")]
unsafe extern "system" {
    fn GetDiskFreeSpaceExW(
        lp_directory_name: *const u16,
        lp_free_bytes_available_to_caller: *mut u64,
        lp_total_number_of_bytes: *mut u64,
        lp_total_number_of_free_bytes: *mut u64,
    ) -> i32;
}
