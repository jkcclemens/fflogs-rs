extern crate fflogs;
extern crate dotenv;

use fflogs::FfLogs;

use std::env;

fn main() {
  dotenv::dotenv().ok();

  let api_key = env::var("FFLOGS_API_KEY").expect("FFLOGS_API_KEY in env");
  let fflogs = FfLogs::new(&api_key);

  println!("{:#?}", fflogs.zones());
}
