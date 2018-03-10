pub type Classes = Vec<Class>;

#[derive(Debug, Deserialize)]
pub struct Class {
  pub id: i64,
  pub name: String,
  pub specs: Vec<Spec>,
}

#[derive(Debug, Deserialize)]
pub struct Spec {
  pub id: i64,
  pub name: String,
}
