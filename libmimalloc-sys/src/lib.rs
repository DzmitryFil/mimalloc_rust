// Copyright 2019 Octavian Oncescu

use core::ffi::c_void;

extern "C" {
    pub fn mi_zalloc(size: usize) -> *const c_void;
    pub fn mi_malloc(size: usize) -> *const c_void;
    pub fn mi_realloc(p: *const c_void, size: usize) -> *const c_void;
    pub fn mi_zalloc_aligned(size: usize, alignment: usize) -> *const c_void;
    pub fn mi_malloc_aligned(size: usize, alignment: usize) -> *const c_void;
    pub fn mi_realloc_aligned(p: *const c_void, size: usize, alignment: usize) -> *const c_void;
    pub fn mi_free(p: *const c_void) -> c_void;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_frees_memory_malloc() {
        let ptr = unsafe { mi_malloc_aligned(8, 8) } as *mut u8;
        unsafe { mi_free(ptr as *const c_void) };
    }

    #[test]
    fn it_frees_memory_zalloc() {
        let ptr = unsafe { mi_zalloc_aligned(8, 8) } as *mut u8;
        unsafe { mi_free(ptr as *const c_void) };
    }

    #[test]
    fn it_frees_memory_realloc() {
        let ptr = unsafe { mi_malloc_aligned(8, 8) } as *mut u8;
        let ptr = unsafe { mi_realloc_aligned(ptr as *const c_void, 8, 8) } as *mut u8;
        unsafe { mi_free(ptr as *const c_void) };
    }
}
