use ::std::sync::Arc;

pub trait ITag {
  /// Return the hint of class name of the structure.
  fn class_name(&self) -> Arc<String>;
  /// Return the hint of module name and class name in Rust.
  fn rs_module_name(&self) -> Arc<String>;
  /// Return the hint of module name in TypeScript.
  fn ts_module_name(&self) -> Arc<String>;
}
