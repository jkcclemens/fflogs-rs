use net::*;
use vec_map::VecMap;

#[derive(Debug, Default)]
pub struct QueryEncounter(pub(crate) VecMap<&'static str, String>);

impl QueryEncounter {
  pub fn metric(mut self, metric: Metric) -> Self {
    self.0.insert("metric", metric.to_string());

    self
  }

  pub fn size(mut self, size: usize) -> Self {
    self.0.insert("size", size.to_string());

    self
  }

  pub fn difficulty(mut self, difficulty: usize) -> Self {
    self.0.insert("difficulty", difficulty.to_string());

    self
  }

  pub fn partition(mut self, partition: usize) -> Self {
    self.0.insert("partition", partition.to_string());

    self
  }

  pub fn class(mut self, class: usize) -> Self {
    self.0.insert("class", class.to_string());

    self
  }

  pub fn spec(mut self, spec: usize) -> Self {
    self.0.insert("spec", spec.to_string());

    self
  }

  /// Sets the job for this request.
  ///
  /// This is a convenience method for calling `class(1)` and `spec(job.as_usize())`.
  pub fn job(self, job: Job) -> Self {
    self.class(1).spec(job.as_usize())
  }

  pub fn bracket(mut self, bracket: usize) -> Self {
    self.0.insert("bracket", bracket.to_string());

    self
  }

  pub fn limit(mut self, limit: usize) -> Self {
    self.0.insert("limit", limit.to_string());

    self
  }

  pub fn page(mut self, page: usize) -> Self {
    self.0.insert("page", page.to_string());

    self
  }

  pub fn guild<S: Into<String>>(mut self, guild: S) -> Self {
    self.0.insert("guild", guild.into());

    self
  }

  pub fn server<S: Into<String>>(mut self, server: S) -> Self {
    self.0.insert("server", server.into());

    self
  }

  pub fn filter<S: Into<String>>(mut self, filter: S) -> Self {
    self.0.insert("filter", filter.into());

    self
  }

  pub fn region(mut self, region: ServerRegion) -> Self {
    self.0.insert("region", region.to_string());

    self
  }
}

#[derive(Debug, Default)]
pub struct QueryCharacter(pub(crate) VecMap<&'static str, String>);

impl QueryCharacter {
  pub fn metric(mut self, metric: Metric) -> Self {
    self.0.insert("metric", metric.to_string());

    self
  }

  pub fn partition(mut self, partition: usize) -> Self {
    self.0.insert("partition", partition.to_string());

    self
  }

  pub fn bracket(mut self, bracket: usize) -> Self {
    self.0.insert("bracket", bracket.to_string());

    self
  }

  pub fn encounter<S: Into<String>>(mut self, encounter: S) -> Self {
    self.0.insert("encounter", encounter.into());

    self
  }

  pub fn zone<S: Into<String>>(mut self, zone: S) -> Self {
    self.0.insert("zone", zone.into());

    self
  }
}
