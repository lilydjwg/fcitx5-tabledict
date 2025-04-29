use std::os::raw::c_char;
use std::ffi::{c_void, CString, NulError};
use std::path::Path;
use std::os::unix::ffi::OsStrExt;
use std::io;
use std::fmt;

enum TableDictCxx { }

unsafe extern "C" {
  fn new_tabledict() -> *mut TableDictCxx;
  fn free_table_dict(td: *mut TableDictCxx);

  fn load_main(td: *mut TableDictCxx, filename: *const c_char, err: *mut c_void);
  fn load_user(td: *mut TableDictCxx, filename: *const c_char, err: *mut c_void);
  fn save_user(td: *mut TableDictCxx, filename: *const c_char, err: *mut c_void);

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

  fn reverse_lookup(
    td: *const TableDictCxx,
    word: *const u8, size: usize,
    flag: PhraseFlag,
    result: *mut c_void,
    err: *mut c_void,
  );

  fn insert(
    td: *mut TableDictCxx,
      code: *const u8,
      code_len: usize,
      word: *const u8,
      word_len: usize,
      flag: PhraseFlag,
  ) -> bool;

  fn delete_entry(
    td: *mut TableDictCxx,
      code: *const u8,
      code_len: usize,
      word: *const u8,
      word_len: usize,
  ) -> bool;

  fn statistic(td: *const TableDictCxx);
}

pub struct TableDict {
  ptr: *mut TableDictCxx,
  user_dict_path: Option<CString>,
}

#[derive(Debug)]
pub struct WordEntry {
  pub code: String,
  pub word: String,
  pub index: u32,
  pub flag: PhraseFlag,
}

impl fmt::Display for WordEntry {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
    write!(f, "{} {} {} {:?}", self.code, self.word, self.index, self.flag)
  }
}

/// Note: returned value should be used only within the callback
unsafe fn char_ptr_to_str(ptr: *const u8, len: usize) -> &'static str {
  unsafe {
    let slice = std::slice::from_raw_parts(ptr, len);
    std::str::from_utf8_unchecked(slice)
  }
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

    fn handle_err(msg: &str, which: &str) -> io::Result<()> {
      if msg.is_empty() {
        Ok(())
      } else {
        Err(io::Error::other(format!("failed to load {}: {}", which, msg)))
      }
    }

    let mut err = String::new();
    if let Some(f) = main_dict {
      let cpath = path_to_cstring(f.as_ref())?;
      unsafe { load_main(td, cpath.as_ptr(), &mut err as *mut _ as _) };
      handle_err(&err, "main dict")?;
    }
    let user_dict_path;
    if let Some(f) = user_dict {
      let cpath = path_to_cstring(f.as_ref())?;
      unsafe { load_user(td, cpath.as_ptr(), &mut err as *mut _ as _) };
      handle_err(&err, "user dict")?;
      user_dict_path = Some(cpath);
    } else {
      user_dict_path = None;
    }
    Ok(Self {
      ptr: td,
      user_dict_path,
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
      unsafe {
        (*(vec as *mut Vec<WordEntry>)).push(WordEntry {
          code: char_ptr_to_str(code, code_len).to_owned(),
          word: char_ptr_to_str(word, word_len).to_owned(),
          index,
          flag,
        })
      }
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

  pub fn reverse_lookup(&self, word: &str, flag: PhraseFlag) -> Result<String, String> {
    let mut ret = String::new();
    let mut err = String::new();

    unsafe {
      reverse_lookup(
        self.ptr,
        word.as_ptr(),
        word.len(),
        flag,
        &mut ret as *mut _ as _,
        &mut err as *mut _ as _,
      );
    }

    if err.is_empty() {
      Ok(ret)
    } else {
      Err(err)
    }
  }

  pub fn insert(&mut self, code: &str, word: &str) -> bool {
    unsafe {
      insert(
        self.ptr,
        code.as_ptr(),
        code.len(),
        word.as_ptr(),
        word.len(),
        PhraseFlag::User,
      )
    }
  }
  
  pub fn delete(&mut self, code: &str, word: &str) -> bool {
    unsafe {
      delete_entry(
        self.ptr,
        code.as_ptr(),
        code.len(),
        word.as_ptr(),
        word.len(),
      )
    }
  }
  
  pub fn save(&self) -> io::Result<()> {
    if let Some(path) = &self.user_dict_path {
      let mut err = String::new();
      unsafe {
        save_user(self.ptr, path.as_ptr(), &mut err as *mut _ as _)
      }
      if err.is_empty() {
        Ok(())
      } else {
        Err(io::Error::other(err))
      }
    } else {
      Err(io::Error::new(
          io::ErrorKind::InvalidInput,
          "cannot save dict: user dict path not supplied on construction",
      ))
    }
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

#[unsafe(no_mangle)]
unsafe extern "C" fn put_string(
  s: *mut c_void,
  v: *const u8,
  len: usize,
) {
  unsafe {
    (*(s as *mut String)).push_str(char_ptr_to_str(v, len));
  }
}
