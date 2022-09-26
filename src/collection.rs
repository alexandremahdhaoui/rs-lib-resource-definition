use serde_json::Error;

pub trait Collector: Sized {
    fn api(&self) -> String ;
    fn kind(&self) -> String;
    fn ser_json(&self) -> Result<String, Error>;
    fn de_json(api: String, json: String) -> Option<Self>;
}
