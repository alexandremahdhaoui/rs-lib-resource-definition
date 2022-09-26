/// TODO: Collection should be able to get "api" & "kind" arbitrary out of a json string
///  - This will enable `Collector::de` to use these functions & only pass `json: String` as
///    argument.
///  - Should parse json into token & only check root fields.
///    e.g. the parser works with only an AST of depths 1 (not really 1 though).
///  - Parser will continue parsing the true trying to go back to the root of the tree (level "1").
///    However it will completely ignore the content it traverses.
///
/// TODO: Otherwise, just use serde & try to see if both `api` & `kind` exist in a HashMap
///  - Can use a struct with only `api: String` & `kind: String` as field & check there content.
///
/// TODO: Try to keep the code Open.

/// TODO: split `def_generator` into:
///  - `definition`
///  - `collection`
#[macro_export]
macro_rules! def_generator {
    ($kind: literal{
        $(
            $api:ident { $api_spec: ident {
                $($field_name:ident : $field_type:ty),+
            }}
        )+
    }) => {
        /// TODO: Move  the `use` in the module.
        ///  then add `use super::*` in the tests.
        use crate::{resource_definition::{ResourceDefinition, Definition}, collection::Collector};
        use serde::{Serialize, Deserialize};
        $(
            #[allow(non_camel_case_types)]
            #[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
            pub struct $api_spec { $($field_name: $field_type),+ }
            impl $api_spec {
                fn new($($field_name: $field_type),+) -> Self {
                    Self {$($field_name),+}
                }
            }
        )+
        #[derive(Debug, PartialEq)]
        pub enum Collection {
            $(
                #[allow(non_camel_case_types)]
                $api(ResourceDefinition<$api_spec>),
            )+
        }
        impl Collector for Collection{
            fn key(&self) -> String {
                match self {
                    $(
                    Collection::$api(def) => def.key(),
                    )+
                }
            }
            fn ser(&self) -> Option<String> {
                match self {
                    $(
                    Collection::$api(def) => def.ser(),
                    )+
                }
            }
            fn de(key: String, input: String) -> Option<Self> {
                $(
                if key == stringify!($api) {
                    return match ResourceDefinition::<$api_spec>::de(input) {
                        Some(obj) => Some(Collection::$api(obj)),
                        _ => None
                    }
                }
                )+
                None
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::metadata::Metadata;
    use pretty_assertions::assert_eq;
    use serde_json;

    def_generator! {
        "Distributed" {
            v1_alpha {
                v1_alpha_spec {
                    section: String,
                    sub_sections: Vec<String>,
                    title: String,
                    order: u8
                }
            }
            test {
                test_spec {
                    yolo: String
                }
            }
        }
    }

    fn get_collection() -> Collection {
        Collection::v1_alpha(get_obj())
    }

    fn get_key() -> String {
        "distributed_v1_alpha".to_string()
    }

    fn get_obj() -> ResourceDefinition<v1_alpha_spec> {
        ResourceDefinition::new(
            "v1_alpha",
            "Distributed",
            Metadata::new("yolo_testing"),
            v1_alpha_spec::new(
                "section".to_string(),
                vec!["sub_section".to_string()],
                "A Title".to_string(),
                0)
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
    fn generator_ser() {
        let input = get_obj();
        let expected = get_str();
        let output = input.ser().unwrap();
        dbg!(&input);
        assert_eq!(output, expected);
    }

    #[test]
    fn generator_de() {
        let input = get_str();
        let expected = get_obj();
        let output: ResourceDefinition<v1_alpha_spec> = serde_json::from_str(input.as_str()).unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn collection_ser() {
        let input = get_collection();
        let expected = get_str();
        let output = input.ser().unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn collection_de() {
        let input= get_str();
        let expected= get_collection();
        let output = Collection::de("v1_alpha".to_string(), input).unwrap();
        assert_eq!(output, expected);
    }
    #[test]
    fn generator_key() {
        let input =  get_obj();
        let expected = get_key();
        let output = input.key();
        assert_eq!(output, expected);
    }

    #[test]
    fn collection_key() {
        let input =  get_collection();
        let expected = get_key();
        let output = input.key();
        assert_eq!(output, expected);
    }
}