#[macro_export]
macro_rules! setter {
  ($name: ident, $type: ty) => {
    pub fn $name(mut self, value: $type) -> Self {
      self.$name = value;
      return self;
    }
  };
}
