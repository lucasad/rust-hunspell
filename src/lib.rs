#![feature(libc)]
#![feature(std_misc)]

//! This crate provides bindings to hunspell from Rust. 
//!
extern crate libc;

use std::ffi::CString;
use std::ptr;

mod ffi; 


/// # Examples
///
/// ```
/// # let aff_path = "/usr/share/myspell/en_US.aff";
/// # let dic_path = "/usr/share/myspell/en_US.dic";
/// let dic = hunspell::Hunspell::new(aff_path, dic_path);
///
/// assert!(dic.spell("word"));
/// assert_eq!(dic.spell("notaword"), false);

///
pub struct Hunspell {
    _handle: *mut ffi::Hunhandle
}

impl Hunspell {
    /// - `affpath` The affix file
    pub fn new(affpath: &str, dpath: &str) -> Hunspell {
        let affpath = CString::new(affpath).unwrap();
        let dpath = CString::new(dpath).unwrap();

        Hunspell {
            _handle: unsafe { ffi::Hunspell_create(affpath.as_ptr(), dpath.as_ptr()) },
        }
    }

    pub fn spell(&self, word: &str) -> bool {
        let word = CString::new(word).unwrap();
        match unsafe { ffi::Hunspell_spell(self._handle, word.as_ptr()) } {
            0 => false,
            _ => true,
        }
    }
}

impl Drop for Hunspell {
    fn drop(&mut self) {
        unsafe {
            ffi::Hunspell_destroy(self._handle);
        }
    }
}
