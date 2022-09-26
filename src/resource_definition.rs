use serde::{self, Deserialize, Serialize};
use serde_json;
use crate::metadata::Metadata;

/// TODO: Rename `ser_json` & `de_json` to `ser` & `de` to keep the code Open to anything
/// TODO: Try to create an interface for implementation of serde, e.g. `serde_json`.
///  try:
///  - abstract factory
///  - bridge
///
/// Add `set_strategy`?
pub trait Definition {
    fn key(&self) -> String;
    /// TODO: Delete `api` & only use the `json: String`
    fn ser(&self) -> Option<String>;
    fn de(input: String) -> Option<Self> where Self: Sized;
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

impl<T: Serialize + for<'de> Deserialize<'de> + Default> Definition for ResourceDefinition<T> {
    fn key(&self) -> String {
        format!("{}_{}", self.kind.to_lowercase(), self.api.to_lowercase())
    }

    fn ser(&self) -> Option<String>{
        return match serde_json::to_string(&self) {
            Ok(s) => Some(s),
            Err(_) => None
        }
    }

    fn de(s: String) -> Option<Self> {
        match serde_json::from_str(s.as_str()) {
            Ok(obj) =>  Some(obj),
            Err(_) => None
        }
    }
}

// pub fn new<T>() -> fn(&str, &str, Metadata, T) -> ResourceDefinition<T> {
//     ResourceDefinition::new
// }

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

    fn get_key() -> String {
        "distributed_v1_alpha".to_string()
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
        let output = input.ser().unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_de_serde_trait() {
        let input = get_str();
        let expected = get_obj();
        let output =
            ResourceDefinition
                ::<V1AlphaSpec>
                ::de(input)
                .unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_key() {
        let obj = get_obj();
        let expected = get_key();
        assert_eq!(obj.key(), expected);
    }

}
