extern crate fflogs;
extern crate dotenv;

use fflogs::FfLogs;
use fflogs::net::Server;

use std::env;

fn main() {
  dotenv::dotenv().ok();

  let api_key = env::var("FFLOGS_API_KEY").expect("FFLOGS_API_KEY in env");
  let fflogs = FfLogs::new(&api_key);

  let ranks = fflogs.rankings_character("Duvivi Duvi", Server::Adamantoise, |x| x);

  println!("{:#?}", ranks);
}
