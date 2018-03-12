use errors::*;

pub mod parses;
pub mod rankings;
pub mod reports;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum FfLogsResult<T> {
  Ok(T),
  Err(FfLogsError)
}

impl<T> FfLogsResult<T> {
  pub fn into_result(self) -> Result<T> {
    match self {
      FfLogsResult::Ok(t) => Ok(t),
      FfLogsResult::Err(e) => Err(Error::FfLogs(e))
    }
  }
}

#[derive(Debug, Deserialize)]
pub struct FfLogsError {
  pub status: i64,
  pub error: String
}

#[derive(Debug, ToString, Clone, Copy)]
pub enum ServerRegion {
  #[strum(serialize = "NA")]
  NorthAmerica,
  #[strum(serialize = "EU")]
  Europe,
  #[strum(serialize = "KR")]
  Korea,
  #[strum(serialize = "TW")]
  Taiwan,
  #[strum(serialize = "CN")]
  China,
  #[strum(serialize = "JP")]
  Japan
}

#[derive(Debug, ToString, Clone, Copy)]
pub enum ReportView {
  #[strum(serialize = "damage-done")]
  DamageDone,
  #[strum(serialize = "damage-taken")]
  DamageTaken,
  #[strum(serialize = "healing")]
  Healing,
  #[strum(serialize = "casts")]
  Casts,
  #[strum(serialize = "summons")]
  Summons,
  #[strum(serialize = "buffs")]
  Buffs,
  #[strum(serialize = "debuffs")]
  Debuffs,
  #[strum(serialize = "deaths")]
  Deaths,
  #[strum(serialize = "survivability")]
  Survivability,
  #[strum(serialize = "resources")]
  Resources,
  #[strum(serialize = "resources-gains")]
  ResourcesGains
}

#[derive(Debug, ToString, Clone, Copy)]
pub enum Metric {
  // Fight metrics
  #[strum(serialize = "speed")]
  Speed,
  #[strum(serialize = "execution")]
  Execution,
  #[strum(serialize = "feats")]
  Feats,
  // Character metrics
  #[strum(serialize = "dps")]
  Dps,
  #[strum(serialize = "hps")]
  Hps,
  #[strum(serialize = "bossdps")]
  BossDps,
  #[strum(serialize = "tankhps")]
  TankHps,
  #[strum(serialize = "playerspeed")]
  PlayerSpeed
}
