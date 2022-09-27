#[macro_export]
macro_rules! apis {
    ($($api_name:ident : $api_version:literal {$($spec_type:ident),+})+) => {
        $(
            #[derive(Debug, Deserialize, PartialEq, Serialize)]
            #[serde(tag="kind")]
            pub enum $api_name {
                $($spec_type{metadata : $crate::Metadata, spec : $spec_type}),+
            }
        )+
        #[derive(Debug, Deserialize, PartialEq, Serialize)]
        #[serde(tag="apiVersion")]
        pub enum APIs {
            $(
            #[serde(rename=$api_version, alias=$api_version)]
            $api_name($api_name),
            )+
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::specs;
    use pretty_assertions::assert_eq;
    use serde::{self, Serialize, Deserialize};
    use serde_json;

    specs!{
        Test {
            string: String
        }
    }

    apis!{
        V1Alpha: "definitions.alexandre.mahdhaoui.com/v1alpha" {
            Test
        }
        V1Beta1: "definitions.alexandre.mahdhaoui.com/v1beta1" {
            Test
        }
    }

    fn obj() -> APIs {
        APIs::V1Alpha(V1Alpha::Test {
            metadata: Default::default(),
            spec: Test{string: "test".to_string()}
        })
    }

    fn string() -> String {
        r#"{"apiVersion":"definitions.alexandre.mahdhaoui.com/v1alpha","kind":"Test","metadata":{"name":"","labels":{},"annotations":{}},"spec":{"string":"test"}}"#.to_string()
    }

    #[test]
    fn apis_de() {
        let i = string();
        let e = obj();
        let o: APIs = serde_json::from_str(&i).unwrap();
        assert_eq!(e, o);
    }

    #[test]
    fn apis_ser() {
        let i = obj();
        let e = string();
        let o = serde_json::to_string(&i).unwrap();
        assert_eq!(e, o);
    }
}