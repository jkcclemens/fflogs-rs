pub mod fights;

pub type Reports = Vec<Report>;

#[derive(Debug, Deserialize)]
pub struct Report {
  pub id: String,
  pub title: String,
  pub owner: String,
  pub start: i64,
  pub end: i64,
  pub zone: i64,
}
