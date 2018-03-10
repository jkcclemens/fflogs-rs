pub type Rankings = Vec<Ranking>;

#[derive(Debug, Deserialize)]
pub struct Ranking {
  pub encounter: i64,
  pub class: i64,
  pub spec: i64,
  pub guild: String,
  pub rank: i64,
  #[serde(rename = "outOf")]
  pub out_of: i64,
  pub duration: i64,
  #[serde(rename = "startTime")]
  pub start_time: i64,
  #[serde(rename = "reportID")]
  pub report_id: String,
  #[serde(rename = "fightID")]
  pub fight_id: i64,
  pub difficulty: i64,
  pub size: i64,
  #[serde(rename = "itemLevel")]
  pub item_level: f64,
  pub total: f64,
  pub estimated: Option<bool>,
}
