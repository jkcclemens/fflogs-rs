use errors::*;

use std::str::FromStr;

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

#[derive(Debug, ToString, AsRefStr, Clone, Copy)]
pub enum Server {
  Adamantoise,
  Aegis,
  Alexander,
  Anima,
  Asura,
  Atomos,
  Bahamut,
  Balmung,
  Behemoth,
  Belias,
  Brynhildr,
  Cactuar,
  Carbuncle,
  Cerberus,
  Chocobo,
  Coeurl,
  Diabolos,
  Durandal,
  Excalibur,
  Exodus,
  Faerie,
  Famfrit,
  Fenrir,
  Garuda,
  Gilgamesh,
  Goblin,
  Gungnir,
  Hades,
  Hyperion,
  Ifrit,
  Ixion,
  Jenova,
  Kujata,
  Lamia,
  Leviathan,
  Lich,
  Louisoix,
  Malboro,
  Mandragora,
  Masamune,
  Mateus,
  Midgardsormr,
  Moogle,
  Odin,
  Omega,
  Pandaemonium,
  Phoenix,
  Ragnarok,
  Ramuh,
  Ridill,
  Sargatanas,
  Shinryu,
  Shiva,
  Siren,
  Tiamat,
  Titan,
  Tonberry,
  Typhon,
  Ultima,
  Ultros,
  Unicorn,
  Valefor,
  Yojimbo,
  Zalera,
  Zeromus,
  Zodiark,
  // String isn't Copy
  // Other {
  //   name: String,
  //   region: ServerRegion
  // },
}

impl Server {
  pub fn region(&self) -> ServerRegion {
    match *self {
      Server::Adamantoise | Server::Balmung | Server::Cactuar | Server::Coeurl | Server::Faerie
      | Server::Gilgamesh | Server::Goblin | Server::Jenova | Server::Mateus | Server::Midgardsormr
      | Server::Sargatanas | Server::Siren | Server::Zalera | Server::Behemoth | Server::Brynhildr
      | Server::Diabolos | Server::Excalibur | Server::Exodus | Server::Famfrit | Server::Hyperion
      | Server::Lamia | Server::Leviathan | Server::Malboro | Server::Ultros
        => ServerRegion::NorthAmerica,
      Server::Cerberus | Server::Lich | Server::Louisoix | Server::Moogle | Server::Odin
      | Server::Omega | Server::Phoenix | Server::Ragnarok | Server::Shiva | Server::Zodiark
        => ServerRegion::Europe,
      Server::Aegis | Server::Atomos | Server::Carbuncle | Server::Garuda | Server::Gungnir
      | Server::Kujata | Server::Ramuh | Server::Tonberry | Server::Typhon | Server::Unicorn
      | Server::Alexander | Server::Bahamut | Server::Durandal | Server::Fenrir | Server::Ifrit
      | Server::Ridill | Server::Tiamat | Server::Ultima | Server::Valefor | Server::Yojimbo
      | Server::Zeromus | Server::Anima | Server::Asura | Server::Belias | Server::Chocobo
      | Server::Hades | Server::Ixion | Server::Mandragora | Server::Masamune | Server::Pandaemonium
      | Server::Shinryu | Server::Titan
        => ServerRegion::Japan,
      // Server::Other { name, region } => region
    }
  }
}

impl FromStr for Server {
  type Err = ();

  fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
    let server = match s.to_lowercase().as_str() {
      "adamantoise" => Server::Adamantoise,
      "aegis" => Server::Aegis,
      "alexander" => Server::Alexander,
      "anima" => Server::Anima,
      "asura" => Server::Asura,
      "atomos" => Server::Atomos,
      "bahamut" => Server::Bahamut,
      "balmung" => Server::Balmung,
      "behemoth" => Server::Behemoth,
      "belias" => Server::Belias,
      "brynhildr" => Server::Brynhildr,
      "cactuar" => Server::Cactuar,
      "carbuncle" => Server::Carbuncle,
      "cerberus" => Server::Cerberus,
      "chocobo" => Server::Chocobo,
      "coeurl" => Server::Coeurl,
      "diabolos" => Server::Diabolos,
      "durandal" => Server::Durandal,
      "excalibur" => Server::Excalibur,
      "exodus" => Server::Exodus,
      "faerie" => Server::Faerie,
      "famfrit" => Server::Famfrit,
      "fenrir" => Server::Fenrir,
      "garuda" => Server::Garuda,
      "gilgamesh" => Server::Gilgamesh,
      "goblin" => Server::Goblin,
      "gungnir" => Server::Gungnir,
      "hades" => Server::Hades,
      "hyperion" => Server::Hyperion,
      "ifrit" => Server::Ifrit,
      "ixion" => Server::Ixion,
      "jenova" => Server::Jenova,
      "kujata" => Server::Kujata,
      "lamia" => Server::Lamia,
      "leviathan" => Server::Leviathan,
      "lich" => Server::Lich,
      "louisoix" => Server::Louisoix,
      "malboro" => Server::Malboro,
      "mandragora" => Server::Mandragora,
      "masamune" => Server::Masamune,
      "mateus" => Server::Mateus,
      "midgardsormr" => Server::Midgardsormr,
      "moogle" => Server::Moogle,
      "odin" => Server::Odin,
      "omega" => Server::Omega,
      "pandaemonium" => Server::Pandaemonium,
      "phoenix" => Server::Phoenix,
      "ragnarok" => Server::Ragnarok,
      "ramuh" => Server::Ramuh,
      "ridill" => Server::Ridill,
      "sargatanas" => Server::Sargatanas,
      "shinryu" => Server::Shinryu,
      "shiva" => Server::Shiva,
      "siren" => Server::Siren,
      "tiamat" => Server::Tiamat,
      "titan" => Server::Titan,
      "tonberry" => Server::Tonberry,
      "typhon" => Server::Typhon,
      "ultima" => Server::Ultima,
      "ultros" => Server::Ultros,
      "unicorn" => Server::Unicorn,
      "valefor" => Server::Valefor,
      "yojimbo" => Server::Yojimbo,
      "zalera" => Server::Zalera,
      "zeromus" => Server::Zeromus,
      "zodiark" => Server::Zodiark,
      _ => return Err(())
    };
    Ok(server)
  }
}

#[derive(Debug, ToString, AsRefStr, Clone, Copy)]
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
