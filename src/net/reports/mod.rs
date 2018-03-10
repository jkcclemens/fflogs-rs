use vec_map::VecMap;

pub mod fights;

#[derive(Debug, Default)]
pub struct QueryReports(pub(crate) VecMap<&'static str, String>);

impl QueryReports {
  pub fn start(mut self, start: i64) -> Self {
    self.0.insert("start", start.to_string());

    self
  }

  pub fn end(mut self, end: i64) -> Self {
    self.0.insert("end", end.to_string());

    self
  }
}
