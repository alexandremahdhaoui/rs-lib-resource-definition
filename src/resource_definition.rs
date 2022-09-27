use serde::{Deserialize, Serialize};
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
    fn ser(&self) -> Option<String>;
    fn de(input: String) -> Option<Self> where Self: Sized;
}


#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all="camelCase")]
pub struct ResourceDefinition<T> {
    #[serde(skip)]
    // #[serde(skip_serializing)]
    api_version: String,
    #[serde(skip)]
    // #[serde(skip_serializing)]
    kind: String,
    #[serde(default)]
    metadata: Metadata,
    spec: T
}

impl<T> ResourceDefinition<T> {
    pub fn new(api_version: &str, kind: &str, metadata: Metadata, spec: T) -> Self {
        Self {
            api_version: api_version.to_string(),
            kind: kind.to_string(),
            metadata,
            spec
        }
    }
}

impl<T: Serialize + for<'de> Deserialize<'de> + Default> Definition for ResourceDefinition<T> {
    fn key(&self) -> String {
        format!("{}/{}", self.api_version, self.kind)
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

/// TODO: Try to keep the code Open.
/// TODO: split `def_generator` into:
///  - `definition`
///  - `collection`

/// def_generator requires importing:
///  - serde::{Serialize, Deserialize}
///  - rs_lib_resource_definition::resource_definition::{ResourceDefinition, Definition}
///
#[macro_export]
macro_rules! def_generator {
    ($($spec:ident {$($field_name:ident : $field_type:ty),+})+) => {
        $(
            #[allow(non_camel_case_types)]
            #[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
            #[serde(rename_all="camelCase")]
            pub struct $spec { $($field_name: $field_type),+ }
            impl $spec {
                fn new($($field_name: $field_type),+) -> Self {
                    Self {$($field_name),+}
                }
            }
        )+
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
    #[serde(rename_all="camelCase")]
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
        "v1alpha/Distributed".to_string()
    }

    fn get_obj() -> ResourceDefinition<V1AlphaSpec> {
        ResourceDefinition::new(
            "v1alpha",
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
         \"apiVersion\":\"v1alpha\",\
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
                \"subSections\":[\"sub_section\"],\
                \"title\":\"A Title\",\
                \"order\":0\
            }\
         }".to_string()
    }

    #[test]
    fn ser_rd() {
        let input = get_obj();
        let expected = get_str();
        let output = serde_json::to_string(&input).unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn de_rd() {
        let input = get_str();
        let expected = get_obj();
        let output: ResourceDefinition<V1AlphaSpec> = serde_json::from_str(input.as_str()).unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn ser_serde_trait() {
        let input = get_obj();
        let expected = get_str();
        let output = input.ser().unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn de_serde_trait() {
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
    fn key() {
        let obj = get_obj();
        let expected = get_key();
        assert_eq!(obj.key(), expected);
    }

    def_generator! {
        v1alpha {
            section: String,
            sub_sections: Vec<String>,
            title: String,
            order: u8
        }
        test {
            yolo: String
        }
    }

    fn get_gen_obj() -> ResourceDefinition<v1alpha> {
        ResourceDefinition::new(
            "v1alpha",
            "Distributed",
            Metadata::new("yolo_testing"),
            v1alpha::new(
                "section".to_string(),
                vec!["sub_section".to_string()],
                "A Title".to_string(),
                0)
        )
    }

    #[test]
    fn def_generator_ser() {
        let input = get_gen_obj();
        let expected = get_str();
        let output = input.ser().unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn def_generator_de() {
        let input = get_str();
        let expected = get_gen_obj();
        let output: ResourceDefinition<v1alpha> = serde_json::from_str(input.as_str()).unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn def_generator_key() {
        let input =  get_gen_obj();
        let expected = get_key();
        let output = input.key();
        assert_eq!(output, expected);
    }
}
