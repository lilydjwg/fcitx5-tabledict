use std::os::raw::c_char;
use std::ffi::{c_void, CStr, CString, NulError};
use std::path::Path;
use std::os::unix::ffi::OsStrExt;
use std::io;

enum TableDictCxx { }

extern "C" {
  fn new_tabledict() -> *mut TableDictCxx;
  fn free_table_dict(td: *mut TableDictCxx);

  fn load_main(td: *mut TableDictCxx, filename: *const c_char) -> *mut c_char;
  fn load_user(td: *mut TableDictCxx, filename: *const c_char) -> *mut c_char;

  fn match_words(
    td: *const TableDictCxx,
    code: *const u8, size: usize,
    mode: TableMatchMode,
    callback: unsafe extern "C" fn(
      vec: *mut c_void,
      code: *const u8,
      code_len: usize,
      word: *const u8,
      word_len: usize,
      index: u32,
      flag: PhraseFlag),
    data: *mut c_void,
   ) -> bool;

  fn statistic(td: *const TableDictCxx);
  fn free(v: *mut c_void);
}

pub struct TableDict {
  ptr: *mut TableDictCxx,
}

#[derive(Debug)]
pub struct WordEntry {
  pub code: String,
  pub word: String,
  pub index: u32,
  pub flag: PhraseFlag,
}

unsafe fn char_ptr_to_string(ptr: *const u8, len: usize) -> String {
  let slice = std::slice::from_raw_parts(ptr, len);
  std::str::from_utf8(slice).unwrap().to_owned()
}

fn path_to_cstring(path: &Path) -> Result<CString, NulError> {
  CString::new(path.as_os_str().as_bytes())
}

impl TableDict {
  pub fn from_dict_file<P: AsRef<Path>>(
    main_dict: Option<P>,
    user_dict: Option<P>,
  ) -> io::Result<Self> {
    let td = unsafe { new_tabledict() };

    fn handle_err(msg: *mut c_char, which: &str) -> io::Result<()> {
      if msg.is_null() {
        Ok(())
      } else {
        let m = unsafe { CStr::from_ptr(msg) }.to_string_lossy();
        unsafe { free(msg as _); }

        Err(io::Error::new(
          io::ErrorKind::Other,
          format!("failed to load {}: {}", which, m),
        ))
      }
    }

    if let Some(f) = main_dict {
      let cpath = path_to_cstring(f.as_ref())?;
      let r = unsafe { load_main(td, cpath.as_ptr()) };
      handle_err(r, "main dict")?;
    }
    if let Some(f) = user_dict {
      let cpath = path_to_cstring(f.as_ref())?;
      let r = unsafe { load_user(td, cpath.as_ptr()) };
      handle_err(r, "user dict")?;
    }
    Ok(Self {
      ptr: td,
    })
  }

  pub fn match_words(&self, code: &str, mode: TableMatchMode) -> Vec<WordEntry> {
    let mut ret = vec![];

    unsafe extern "C" fn callback(
      vec: *mut c_void,
      code: *const u8,
      code_len: usize,
      word: *const u8,
      word_len: usize,
      index: u32,
      flag: PhraseFlag,
    ) {
      (*(vec as *mut Vec<WordEntry>)).push(WordEntry {
        code: char_ptr_to_string(code, code_len),
        word: char_ptr_to_string(word, word_len),
        index,
        flag,
      })
    }

    let _matched = unsafe {
      match_words(
        self.ptr,
        code.as_ptr(),
        code.len(),
        mode,
        callback,
        &mut ret as *mut _ as _,
      )
    };
    ret
  }

  pub fn stat(&self) {
    unsafe {
      statistic(self.ptr);
    }
  }
}

impl Drop for TableDict {
  fn drop(&mut self) {
    unsafe { free_table_dict(self.ptr) }
  }
}

#[repr(C)]
#[allow(unused)]
pub enum TableMatchMode { Exact, Prefix }

#[repr(C)]
#[derive(Debug)]
#[allow(unused)]
pub enum PhraseFlag {
  None = 1,
  Pinyin,
  Prompt,
  ConstructPhrase,
  User,
  Auto,
  Invalid,
}
