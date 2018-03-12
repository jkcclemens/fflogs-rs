use errors::*;
use models::classes::Classes;
use models::parses::Parses;
use models::rankings::Rankings;
use models::reports::Reports;
use models::reports::fights::Fights;
use models::zones::Zones;
use net::*;
use net::parses::*;
use net::rankings::*;
use net::reports::*;
use net::reports::fights::*;
use vec_map::VecMap;

use reqwest::Client;
use serde::de::DeserializeOwned;
use serde_json;
use url::Url;

pub struct FfLogs {
  client: Client,
  api_key: String
}

impl FfLogs {
  // no trailing slash
  const API_BASE: &'static str = "https://www.fflogs.com/v1";

  fn make_api_url<P, S>(&self, path: P, q: Option<VecMap<&'static str, String>>) -> Result<Url>
    where P: IntoIterator<Item=S>,
          S: AsRef<str>
  {
    let mut url = Url::parse(FfLogs::API_BASE).map_err(Error::Url)?;
    {
      let mut segments = url.path_segments_mut().map_err(|_| Error::UrlNotBase)?;
      for segment in path.into_iter() {
        segments.push(segment.as_ref());
      }
    }
    if let Some(q) = q {
      for (key, value) in q {
        url.query_pairs_mut().append_pair(key, &value);
      }
    }
    url.query_pairs_mut().append_pair("api_key", &self.api_key);
    Ok(url)
  }

  fn download<T: DeserializeOwned>(&self, url: Url) -> Result<T> {
    let response = self.client.get(url).send().map_err(Error::Reqwest)?;
    let result: FfLogsResult<T> = serde_json::from_reader(response).map_err(Error::Json)?;
    result.into_result()
  }

  pub fn new(api_key: &str) -> FfLogs {
    FfLogs {
      client: Client::new(),
      api_key: api_key.to_string()
    }
  }

  pub fn zones(&self) -> Result<Zones> {
    self.download(self.make_api_url(&["zones"], None)?)
  }

  pub fn classes(&self) -> Result<Classes> {
    self.download(self.make_api_url(&["classes"], None)?)
  }

  pub fn rankings_character<C>(&self, name: &str, server: &str, region: ServerRegion, config: C) -> Result<Rankings>
    where C: FnOnce(QueryCharacter) -> QueryCharacter
  {
    let url = self.make_api_url(
      &["rankings", "character", name, server, &region.to_string()],
      Some(config(QueryCharacter::default()).0)
    )?;
    self.download(url)
  }

  pub fn rankings_encounter<C>(&self, id: &str, config: C) -> Result<Rankings>
    where C: FnOnce(QueryEncounter) -> QueryEncounter
  {
    let url = self.make_api_url(
      &["rankings", "encounter", id],
      Some(config(QueryEncounter::default()).0)
    )?;
    self.download(url)
  }

  pub fn parses<C>(&self, name: &str, server: &str, region: ServerRegion, config: C) -> Result<Parses>
    where C: FnOnce(QueryParses) -> QueryParses
  {
    let url = self.make_api_url(
      &["parses", "character", name, server, &region.to_string()],
      Some(config(QueryParses::default()).0)
    )?;
    self.download(url)
  }

  pub fn reports_guild<C>(&self, name: &str, server: &str, region: ServerRegion, config: C) -> Result<Reports>
    where C: FnOnce(QueryReports) -> QueryReports
  {
    let url = self.make_api_url(
      &["reports", "guild", name, server, &region.to_string()],
      Some(config(QueryReports::default()).0)
    )?;
    self.download(url)
  }

  pub fn reports_user<C>(&self, name: &str, config: C) -> Result<Reports>
    where C: FnOnce(QueryReports) -> QueryReports
  {
    let url = self.make_api_url(
      &["reports", "user", name],
      Some(config(QueryReports::default()).0)
    )?;
    self.download(url)
  }

  pub fn report_fights<C>(&self, code: &str, config: C) -> Result<Fights>
    where C: FnOnce(QueryFight) -> QueryFight
  {
    let url = self.make_api_url(
      &["report", "fight", code],
      Some(config(QueryFight::default()).0)
    )?;
    self.download(url)
  }
}
