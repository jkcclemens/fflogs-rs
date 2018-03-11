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
use url::Url;

pub struct FfLogs {
  client: Client,
  api_key: String
}

impl FfLogs {
  const API_BASE: &'static str = "https://www.fflogs.com/v1/";

  fn make_api_url(&self, path: &str, q: Option<VecMap<&'static str, String>>) -> Result<Url> {
    let mut url = Url::parse(FfLogs::API_BASE)
      .map_err(Error::Url)?
      .join(path)
      .map_err(Error::Url)?;
    if let Some(q) = q {
      for (key, value) in q {
        url.query_pairs_mut().append_pair(key, &value);
      }
    }
    url.query_pairs_mut().append_pair("api_key", &self.api_key);
    Ok(url)
  }

  fn download<T: DeserializeOwned>(&self, url: Url) -> Result<T> {
    let result: FfLogsResult<T> = self.client.get(url)
      .send()
      .map_err(Error::Reqwest)?
      .json()
      .map_err(Error::Reqwest)?;
    match result {
      FfLogsResult::Ok(t) => Ok(t),
      FfLogsResult::Err(e) => Err(Error::FfLogs(e))
    }
  }

  pub fn new(api_key: &str) -> FfLogs {
    FfLogs {
      client: Client::new(),
      api_key: api_key.to_string()
    }
  }

  pub fn zones(&self) -> Result<Zones> {
    self.download(self.make_api_url("zones", None)?)
  }

  pub fn classes(&self) -> Result<Classes> {
    self.download(self.make_api_url("classes", None)?)
  }

  pub fn rankings_character<C>(&self, name: &str, server: &str, region: ServerRegion, config: C) -> Result<Rankings>
    where C: FnOnce(QueryCharacter) -> QueryCharacter
  {
    let url_str = format!("rankings/character/{name}/{server}/{region}",
      name=name,
      server=server,
      region=region.to_string());
    let url = self.make_api_url(&url_str, Some(config(QueryCharacter::default()).0))?;
    self.download(url)
  }

  pub fn rankings_encounter<C>(&self, id: &str, config: C) -> Result<Rankings>
    where C: FnOnce(QueryEncounter) -> QueryEncounter
  {
    let url = self.make_api_url(
      &format!("rankings/encounter/{id}", id=id),
      Some(config(QueryEncounter::default()).0)
    )?;
    self.download(url)
  }

  pub fn parses<C>(&self, name: &str, server: &str, region: ServerRegion, config: C) -> Result<Parses>
    where C: FnOnce(QueryParses) -> QueryParses
  {
    let url_str = format!("parses/character/{name}/{server}/{region}",
      name=name,
      server=server,
      region=region.to_string());
    let url = self.make_api_url(&url_str, Some(config(QueryParses::default()).0))?;
    self.download(url)
  }

  pub fn reports_guild<C>(&self, name: &str, server: &str, region: ServerRegion, config: C) -> Result<Reports>
    where C: FnOnce(QueryReports) -> QueryReports
  {
    let url_str = format!("reports/guild/{name}/{server}/{region}",
      name=name,
      server=server,
      region=region.to_string());
    let url = self.make_api_url(&url_str, Some(config(QueryReports::default()).0))?;
    self.download(url)
  }

  pub fn reports_user<C>(&self, name: &str, config: C) -> Result<Reports>
    where C: FnOnce(QueryReports) -> QueryReports
  {
    let url = self.make_api_url(
      &format!("reports/user/{name}", name=name),
      Some(config(QueryReports::default()).0)
    )?;
    self.download(url)
  }

  pub fn report_fights<C>(&self, code: &str, config: C) -> Result<Fights>
    where C: FnOnce(QueryFight) -> QueryFight
  {
    let url = self.make_api_url(
      &format!("report/fights/{code}", code=code),
      Some(config(QueryFight::default()).0)
    )?;
    self.download(url)
  }
}
