use ::std::sync::Arc;

pub trait ITag {
  /// Return the hint of class name of the structure.
  fn class_name(&self) -> Arc<String>;
  // fn rs_module_name(&self) -> &str;
  // fn ts_module_name(&self) -> &str;
}
