use clap::Parser;

#[derive(Parser, Debug)]
pub struct Config {
  pub query: String,
  pub file_path: String,
  #[arg(short, long)]
  pub ignore_case: bool,
}
