pub trait Collector: Sized {
    fn key(&self) -> String;
    fn ser(&self) -> Option<String>;
    fn de(key: String, input: String) -> Option<Self>;
}

/// Limitations:
///  For now we can only creates collection of struct<T> or Collection of Collections.
///  We cannot create any mix of collections & structs.
#[macro_export]
macro_rules! collection_generator {
    ($name: ident { $class: ident { $( $spec:ident )+ }}) => {
        #[derive(Debug, PartialEq)]
        pub enum $name {
            $(
                #[allow(non_camel_case_types)]
                $spec($class<$spec>),
            )+
        }
        impl Collector for $name {
            fn key(&self) -> String {
                match self {
                    $(
                    $name::$spec(def) => def.key(),
                    )+
                }
            }
            fn ser(&self) -> Option<String> {
                match self {
                    $(
                    $name::$spec(def) => def.ser(),
                    )+
                }
            }
            fn de(key: String, input: String) -> Option<Self> {
                $(
                if key == stringify!($spec) {
                    return match $class::<$spec>::de(input) {
                        Some(obj) => Some($name::$spec(obj)),
                        _ => None
                    }
                }
                )+
                None
            }
        }
    };
    ($name: ident{$($item: ident)+}) => {
        #[derive(Debug, PartialEq)]
        pub enum $name {
            $(
                #[allow(non_camel_case_types)]
                $item($item),
            )+
        }
        impl Collector for $name {
            fn key(&self) -> String {
                match self {
                    $(
                    $name::$item(def) => def.key(),
                    )+
                }
            }
            fn ser(&self) -> Option<String> {
                match self {
                    $(
                    $name::$item(def) => def.ser(),
                    )+
                }
            }
            fn de(key: String, input: String) -> Option<Self> {
                $(
                if key == stringify!($item) {
                    return match $item::de(key, input) {
                        Some(obj) => Some($name::$item(obj)),
                        _ => None
                    }
                }
                )+
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        def_generator,
        metadata::Metadata,
        resource_definition::{ResourceDefinition, Definition}
    };
    use pretty_assertions::assert_eq;
    use serde::{Serialize, Deserialize};

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

    collection_generator!{
        Distributed {
            ResourceDefinition {
                v1alpha
                test
            }
        }
    }
    collection_generator!{
        AnotherCollection {
            ResourceDefinition {
                v1alpha
            }
        }
    }

    collection_generator!{
        SuperCollection {
            Distributed
            AnotherCollection
        }
    }

    fn get_collection() -> Distributed {
        Distributed::v1alpha(get_obj())
    }

    fn get_key() -> String {
        "v1alpha/Distributed".to_string()
    }

    fn get_obj() -> ResourceDefinition<v1alpha> {
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
        let output = Distributed::de("v1alpha".to_string(), input).unwrap();
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