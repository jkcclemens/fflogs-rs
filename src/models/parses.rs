use serde_json;

pub type Parses = Vec<Parse>;

#[derive(Debug, Deserialize)]
pub struct Parse {
  pub difficulty: i64,
  pub size: i64,
  pub kill: i64,
  pub name: String,
  pub specs: Vec<Spec>,
  pub variable: bool,
  pub partition: i64,
}

#[derive(Debug, Deserialize)]
pub struct Spec {
    pub class: String,
    pub spec: String,
    pub combined: bool,
    pub data: Vec<Datum>,
    pub best_persecondamount: f64,
    pub best_historical_percent: f64,
    pub best_historical_locked: bool,
    pub best_duration: i64,
    pub best_allstar_points: f64,
    pub best_combined_allstar_points: i64,
    pub possible_allstar_points: i64,
    pub historical_locked: bool,
    pub historical_total: i64,
    pub historical_median: f64,
    pub historical_avg: f64,
}

#[derive(Debug, Deserialize)]
pub struct Datum {
    pub character_id: i64,
    pub character_name: String,
    pub persecondamount: f64,
    pub keystone_adjust: i64,
    pub ilvl: f64,
    pub affixes: Option<serde_json::Value>,
    pub duration: i64,
    pub start_time: i64,
    pub report_code: String,
    pub report_fight: i64,
    pub ranking_id: i64,
    pub guild: Option<String>,
    pub total: i64,
    pub rank: Rank,
    pub percent: f64,
    pub exploit: i64,
    pub banned: bool,
    pub historical_locked: bool,
    pub historical_count: i64,
    pub historical_percent: f64,
}

#[derive(Debug, Deserialize)]
pub enum Rank {
  #[serde(rename = "-")]
  Empty,
}
