use vec_map::VecMap;

#[derive(Debug, Default)]
pub struct QueryFight(pub(crate) VecMap<&'static str, String>);

impl QueryFight {
  pub fn translate(mut self, translate: bool) -> Self {
    self.0.insert("translate", translate.to_string());

    self
  }
}
