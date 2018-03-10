use serde_json;

#[derive(Debug, Deserialize)]
pub struct Fights {
  pub fights: Vec<FightsFight>,
  pub lang: String,
  pub friendlies: Vec<Friendly>,
  pub enemies: Vec<Enemy>,
  #[serde(rename = "friendlyPets")]
  pub friendly_pets: Vec<Enemy>,
  #[serde(rename = "enemyPets")]
  pub enemy_pets: Vec<serde_json::Value>,
  pub phases: Vec<serde_json::Value>,
  pub title: String,
  pub owner: String,
  pub start: i64,
  pub end: i64,
  pub zone: i64,
}

#[derive(Debug, Deserialize)]
pub struct Enemy {
  pub name: String,
  pub id: i64,
  pub guid: i64,
  #[serde(rename = "type")]
  pub enemy_type: String,
  pub fights: Vec<EnemyFight>,
  #[serde(rename = "petOwner")]
  pub pet_owner: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct EnemyFight {
  pub id: i64,
  pub instances: i64,
}

#[derive(Debug, Deserialize)]
pub struct FightsFight {
  pub id: i64,
  pub start_time: i64,
  pub end_time: i64,
  pub boss: i64,
  pub size: i64,
  pub difficulty: i64,
  pub kill: bool,
  pub partial: i64,
  #[serde(rename = "standardComposition")]
  pub standard_composition: bool,
  #[serde(rename = "bossPercentage")]
  pub boss_percentage: i64,
  #[serde(rename = "fightPercentage")]
  pub fight_percentage: i64,
  #[serde(rename = "lastPhaseForPercentageDisplay")]
  pub last_phase_for_percentage_display: i64,
  pub name: String,
  #[serde(rename = "zoneID")]
  pub zone_id: i64,
  #[serde(rename = "zoneName")]
  pub zone_name: String,
}

#[derive(Debug, Deserialize)]
pub struct Friendly {
  pub name: String,
  pub id: i64,
  pub guid: i64,
  #[serde(rename = "type")]
  pub friendly_type: String,
  pub fights: Vec<FriendlyFight>,
}

#[derive(Debug, Deserialize)]
pub struct FriendlyFight {
  pub id: i64,
}
