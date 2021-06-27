mod readline;
mod tabledict;

use readline::{readline, prompt};

fn main() {
  let dict = tabledict::TableDict::from_dict_file(
    Some("/home/lilydjwg/.local/share/fcitx5/table/lilywb.main.dict"),
    Some("/home/lilydjwg/.local/share/fcitx5/table/lilywb.user.dict"),
  ).unwrap();
  let p = prompt(">> ");
  while let Ok(Some(s)) = readline(&p) {
    if !s.is_empty() {
      println!("read: {}", s);
      let words = dict.match_words(&s, tabledict::TableMatchMode::Exact);
      println!("Matches: {:?}", words);
    }
  }
  println!();
  dict.stat();
}
