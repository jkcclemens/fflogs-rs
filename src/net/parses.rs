use net::*;
use vec_map::VecMap;

#[derive(Debug, Default)]
pub struct QueryParses(pub(crate) VecMap<&'static str, String>);

impl QueryParses {
  pub fn encounter<S: Into<String>>(mut self, encounter: S) -> Self {
    self.0.insert("encounter", encounter.into());

    self
  }

  pub fn metric(mut self, metric: Metric) -> Self {
    self.0.insert("metric", metric.to_string());

    self
  }

  pub fn zone<S: Into<String>>(mut self, zone: S) -> Self {
    self.0.insert("zone", zone.into());

    self
  }

  pub fn bracket(mut self, bracket: usize) -> Self {
    self.0.insert("bracket", bracket.to_string());

    self
  }

  pub fn partition(mut self, partition: usize) -> Self {
    self.0.insert("partition", partition.to_string());

    self
  }

  pub fn compare(mut self, compare: usize) -> Self {
    self.0.insert("compare", compare.to_string());

    self
  }
}
