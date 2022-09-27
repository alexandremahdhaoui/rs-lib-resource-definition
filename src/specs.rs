/// TODO: Try to keep the code Open.
/// TODO: split `def_generator` into:
///  - `definition`
///  - `collection`

/// `specs` requires importing:
/// => `serde::{self, Serialize, Deserialize}`
#[macro_export]
macro_rules! specs {
    ($($spec:ident {$($field_name:ident : $field_type:ty),+})+) => {
        $(

            #[allow(non_camel_case_types)]
            #[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
            #[serde(rename_all="camelCase")]
            pub struct $spec { $($field_name: $field_type),+ }
            #[allow(dead_code)]
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
    use pretty_assertions::assert_eq;
    use serde::{self, Deserialize, Serialize};
    use serde_json;
    use std::collections::HashMap;

    #[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
    struct ComplexChild {
        #[serde(default = "crate::metadata::default_hashmap")]
        nested: HashMap<String, String>
    }

    specs!{
        Test {
            string: String
        }
        Complex {
            name: String,
            complex_child: ComplexChild
        }
    }

    fn json() -> String {
        r#"{"name":"test","complexChild":{"nested":{}}}"#.to_string()
    }

    fn obj() -> Complex {
        Complex { name: "test".to_string(), complex_child: Default::default() }
    }

    #[test]
    fn specs_new() {
        let i = "string";
        let e = Test { string: i.to_string() };
        let o = Test::new(i.to_string());
        assert_eq!(e, o);
    }

    #[test]
    fn specs_de() {
        let i = json();
        let e = obj();
        let o: Complex = serde_json::from_str(&i).unwrap();
        assert_eq!(e, o);
    }

    #[test]
    fn specs_ser() {
        let i = obj();
        let e = json();
        let o = serde_json::to_string(&i).unwrap();
        assert_eq!(e, o);
    }

}

