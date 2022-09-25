use std::collections::HashMap;
use std::fmt::{Debug};
use serde::{Deserialize, Serialize};
use serde;
use serde_json;
use serde_json::Error;

fn default_hashmap() -> HashMap<String, String> {
    HashMap::new()
}

//: Serialize + for <'de> Deserialize<'de> + Sized
pub trait SerDe {
    fn api(&self) -> String;
    fn kind(&self) -> String;
    fn ser_json(&self) -> Result<String, Error>;
    fn de_json(s: String) -> serde_json::Result<Self> where Self: Sized;
}

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Metadata {
    name: String,
    #[serde(default = "default_hashmap")]
    labels: HashMap<String, String>,
    #[serde(default = "default_hashmap")]
    annotations: HashMap<String, String>
}

impl Metadata {
    pub fn new(name: &str) -> Metadata {
        Metadata{
            name: name.to_string(),
            labels: Default::default(),
            annotations: Default::default()
        }
    }
}

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ResourceDefinition<T> {
    api: String,
    kind: String,
    metadata: Metadata,
    spec: T
}

impl<T> ResourceDefinition<T> {
    pub fn new(api: &str, kind: &str, metadata: Metadata, spec: T) -> Self {
        Self {
            api: api.to_string(),
            kind: kind.to_string(),
            metadata,
            spec
        }
    }
}

impl<T: Serialize + for<'de> Deserialize<'de> + Default> SerDe for ResourceDefinition<T> {
    fn api(&self) -> String {
        self.api.to_string()
    }
    fn kind(&self) -> String {
        self.kind.to_string()
    }

    fn ser_json(&self) -> Result<String, Error> {
        serde_json::to_string(&self)
    }

    fn de_json(s: String) -> serde_json::Result<Self> {
        serde_json::from_str(s.as_str())
    }
}

pub fn new<T>() -> fn(&str, &str, Metadata, T) -> ResourceDefinition<T> {
    ResourceDefinition::new
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
    struct V1AlphaSpec {
        section: String,
        sub_sections: Vec<String>,
        title: String,
        order: u8
    }

    impl V1AlphaSpec {
        fn new(section: String, sub_sections: Vec<String>, title: String, order: u8) -> V1AlphaSpec {
            V1AlphaSpec {
                section,
                sub_sections,
                title,
                order
            }
        }
    }


    fn get_obj() -> ResourceDefinition<V1AlphaSpec> {
        ResourceDefinition::new(
            "v1_alpha",
            "Distributed",
            Metadata::new("yolo_testing"),
            V1AlphaSpec::new(
                "section".to_string(),
                vec!["sub_section".to_string()],
                "A Title".to_string(),
                0
            )
        )
    }

    fn get_str() -> String {
        "{\
         \"api\":\"v1_alpha\",\
         \"kind\":\"Distributed\",\
         \"metadata\":\
            {\
                \"name\":\"yolo_testing\",\
                \"labels\":{},\
                \"annotations\":{}\
            },\
         \"spec\":\
            {\
                \"section\":\"section\",\
                \"sub_sections\":[\"sub_section\"],\
                \"title\":\"A Title\",\
                \"order\":0\
            }\
         }".to_string()
    }

    #[test]
    fn test_ser_rd() {
        let input = get_obj();
        let expected = get_str();
        let output = serde_json::to_string(&input).unwrap();

        dbg!(&output);
        assert_eq!(output, expected);
    }

    #[test]
    fn test_de_rd() {
        let input = get_str();
        let expected = get_obj();
        let output: ResourceDefinition<V1AlphaSpec> = serde_json::from_str(input.as_str()).unwrap();

        dbg!(&output);
        assert_eq!(output, expected);
    }

    #[test]
    fn test_ser_serde_trait() {
        let input = get_obj();
        let expected = get_str();
        let output = input.ser_json().unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_de_serde_trait() {
        let input = get_str();
        let expected = get_obj();
        let output = ResourceDefinition
            ::<V1AlphaSpec>
            ::de_json(input)
            .unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_api() {
        let obj = get_obj();
        let expected = "v1_alpha".to_string();

        assert_eq!(obj.api(), expected);
    }
    #[test]
    fn test_kind() {
        let obj = get_obj();
        let expected = "Distributed".to_string();

        assert_eq!(obj.kind(), expected);
    }
}
