use std::ffi::CString;

use super::tabledict::{TableDict, self};
use super::readline;

enum NextOp {
  Continue,
  Exit,
}

type Commands = &'static [(
  &'static str,
  fn(&mut Commander, Args) -> NextOp,
  &'static str,
)];

type Args<'a> = &'a [&'a str];

pub struct Commander {
  dict: TableDict,
  prompt: CString,
  commands: Commands,
}

const COMMANDS: Commands = &[
  ("find", Commander::do_find, "show code for word; only one is shown"),
  ("match", Commander::do_match, "show entries that match each code"),

  ("insert", Commander::do_insert, "insert a pair of (code, word)"),
  ("delete", Commander::do_delete, "delete a pair of (code, word)"),
  ("save", Commander::do_save, "save the user dict"),

  ("stats", Commander::do_stats, "show statistics on the tabledict"),
  ("help", Commander::do_help, "show help"),
  ("quit", Commander::do_quit, "quit this program"),
];

impl Commander {
  pub fn new(dict: TableDict) -> Self {
    Self {
      dict, commands: COMMANDS,
      prompt: readline::prompt("\x1b[38;5;135m>>\x1b[0m "),
    }
  }

  fn do_help(&mut self, _args: Args) -> NextOp {
    for (c, _, h) in self.commands {
      println!("{:10} {}", c, h);
    }
    NextOp::Continue
  }

  fn do_stats(&mut self, _args: Args) -> NextOp {
    self.dict.stat();
    NextOp::Continue
  }

  fn do_match(&mut self, args: Args) -> NextOp {
    if args.is_empty() {
      println!("match: arguments needed");
    } else {
      for w in args {
        for we in self.dict.match_words(w, tabledict::TableMatchMode::Exact) {
          println!("{}", we);
        }
      }
    }
    NextOp::Continue
  }

  fn do_find(&mut self, args: Args) -> NextOp {
    if args.is_empty() {
      println!("find: arguments needed");
    } else {
      for w in args {
        match self.dict.reverse_lookup(w, tabledict::PhraseFlag::None) {
          Ok(code) if !code.is_empty() => println!("{} {}", code, w),
          Ok(_) => println!("{} not found in main dict", w),
          Err(e) => println!("Error: {}", e),
        }
      }
    }
    NextOp::Continue
  }

  fn do_insert(&mut self, args: Args) -> NextOp {
    match args {
      [code, word] => {
        self.dict.insert(code, word);
      },
      _ => { println!("insert takes two arguments: code and word"); },
    }
    NextOp::Continue
  }

  fn do_delete(&mut self, args: Args) -> NextOp {
    match args {
      [code, word] => {
        self.dict.delete(code, word);
      },
      _ => { println!("delete takes two arguments: code and word"); },
    }
    NextOp::Continue
  }

  fn do_save(&mut self, _args: Args) -> NextOp {
    if let Err(e) = self.dict.save() {
      println!("save failed: {}", e.to_string());
    }
    NextOp::Continue
  }

  fn do_quit(&mut self, _args: Args) -> NextOp {
    NextOp::Exit
  }

  pub fn start_loop(&mut self) {
    let mut need_newline = true;

    while let Ok(Some(s)) = readline::readline(&self.prompt) {
      if s.is_empty() {
        continue;
      }
      let words: Vec<_> = s.split_whitespace().collect();
      let cmd = words[0];
      let matches: Vec<_> = self.commands.iter().filter(|(c, _, _)| c.starts_with(cmd)).collect();
      match matches.len() {
        0 => { println!("Unknown command: {}", cmd) },
        1 => {
          match matches[0].1(self, &words[1..]) {
            NextOp::Exit => { need_newline = false; break },
            NextOp::Continue => { },
          }
        },
        _ => {
          print!("Ambiguous command, could be: ");
          matches.iter().map(|(c, _, _)| c).intersperse(&", ").for_each(
            |s| print!("{}", s));
          println!();
        }
      };
    }

    if need_newline {
      println!();
    }
  }

}
