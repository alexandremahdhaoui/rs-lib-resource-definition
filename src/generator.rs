#[macro_export]
macro_rules! kind_generator {
    ($kind: literal{
        $(
            $api:ident { $api_spec: ident {
                $($field_name:ident : $field_type:ty),+
            }}
        )+
    }) => {
        use crate::resource_definition::{ResourceDefinition, SerDe};
        use serde::{Serialize, Deserialize};
        use serde_json;
        use serde_json::Error;
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
        pub enum Collection {
            $(
                #[allow(non_camel_case_types)]
                $api(ResourceDefinition<$api_spec>),
            )+
        }
        impl Collection {
            fn api(&self) -> String {
                match self {
                    $(
                    Collection::$api(el) => el.api(),
                    )+
                }
            }
            fn kind(&self) -> String {
                match self {
                    $(
                    Collection::$api(el) => el.kind(),
                    )+
                }
            }
            fn ser_json(&self) -> Result<String, Error> {
                match self {
                    $(
                    Collection::$api(el) => el.ser_json(),
                    )+
                }
            }
            fn de_json(api: String, json: String) -> Option<Self> {
                $(
                if api == stringify!($api) {
                    return Some(Collection::$api(serde_json::from_str(json.as_str()).unwrap()))
                }
                )+
                None
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::resource_definition::Metadata;
    use pretty_assertions::assert_eq;

    kind_generator! {
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
    fn test_generator_ser() {
        let input = get_obj();
        let expected = get_str();
        let output = input.ser_json().unwrap();
        dbg!(&input);
        dbg!(&expected);
        dbg!(&output);
        assert_eq!(output, expected);
    }

    #[test]
    fn test_generator_de() {
        let input = get_str();
        let expected = get_obj();
        let output: ResourceDefinition<v1_alpha_spec> = serde_json::from_str(input.as_str()).unwrap();
        assert_eq!(output, expected);
    }
    //
    // #[test]
    // fn test_collection_ser() {
    //     let input = get_obj();
    //     let expected = get_str();
    //     assert_eq!(output, expected);
    // }
    //
    // #[test]
    // fn test_collection_de() {
    //     let input= get_str();
    //     let expected= get_obj()
    //     assert_eq!(output, expected);;
    // }
}