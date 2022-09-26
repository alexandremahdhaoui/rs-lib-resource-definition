
/// TODO: Try to create interfaces of implementations of serde, e.g. `serde_json`.
///  try:
///  - abstract factory
///  - bridge
pub trait Collector: Sized {
    fn key(&self) -> String;
    fn ser(&self) -> Option<String>;
    /// TODO: Replace `key`,`input` by `input: String` ?
    fn de(key: String, input: String) -> Option<Self>;
}
