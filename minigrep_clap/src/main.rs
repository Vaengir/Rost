use clap::Parser;
use minigrep_clap::Config;

fn main() {
  let config = Config::parse();
  dbg!(config);
}
