#![feature(iter_intersperse)]

use clap::Parser;

mod readline;
mod tabledict;
mod cmd;

#[derive(Parser)]
struct Args {
  #[arg(default_value="/home/lilydjwg/.local/share/fcitx5/table/lilywb.main.dict")]
  main_dict: String,
  #[arg(default_value="/home/lilydjwg/.local/share/fcitx5/table/lilywb.user.dict")]
  user_dict: String,
}

fn main() {
  let args = Args::parse();

  let dict = tabledict::TableDict::from_dict_file(
    Some(&args.main_dict),
    Some(&args.user_dict),
  ).unwrap();

  let mut c = cmd::Commander::new(dict);
  c.start_loop();
}
