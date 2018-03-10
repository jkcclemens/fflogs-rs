# fflogs_rs

Use the [FF Logs API](https://www.fflogs.com/v1/docs/) in Rust.

```rust
extern crate fflogs;

use fflogs::FfLogs;
use fflogs::net::{ServerRegion, Metric};

fn main() {
  let fflogs = FfLogs::new("my api key");

  let rankings = fflogs.rankings_character("Some Dude", "Adamantoise", ServerRegion::NorthAmerica, |c| c.metric(Metric::Dps));

  println!("{:#?}", rankings);
}
```
