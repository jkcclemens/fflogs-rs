#![feature(macro_at_most_once_rep)]

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate reqwest;
extern crate url;
extern crate strum;
#[macro_use]
extern crate strum_macros;

pub mod errors;
pub mod fflogs;
pub mod models;
pub mod net;

mod vec_map;

pub use fflogs::FfLogs;
