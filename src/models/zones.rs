pub type Zones = Vec<Zone>;

#[derive(Debug, Deserialize)]
pub struct Zone {
  pub id: i64,
  pub name: String,
  pub frozen: bool,
  pub encounters: Vec<Bracket>,
  pub brackets: Vec<Bracket>,
}

#[derive(Debug, Deserialize)]
pub struct Bracket {
  pub id: i64,
  pub name: String,
}
