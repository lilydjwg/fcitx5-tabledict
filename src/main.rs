#![feature(iter_intersperse)]

mod readline;
mod tabledict;
mod cmd;

fn main() {
  let dict = tabledict::TableDict::from_dict_file(
    Some("/home/lilydjwg/.local/share/fcitx5/table/lilywb.main.dict"),
    Some("/home/lilydjwg/.local/share/fcitx5/table/lilywb.user.dict"),
  ).unwrap();

  let mut c = cmd::Commander::new(dict);
  c.start_loop();
}
