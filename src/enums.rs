#![feature(trace_macros)]
#[cfg(test)]
mod tests {
    #![feature(trace_macros)]
    use super::*;
    use serde::{Deserialize, Serialize};
    use serde_json;
    use crate::def_generator;
    use pretty_assertions::assert_eq;
    use crate::ResourceDefinition;

    def_generator! {
        Test {
            yolo: String
        }
    }

    #[derive(Debug, Deserialize,PartialEq, Serialize)]
    #[serde(tag="kind")]
    enum V1Alpha {
        Test(ResourceDefinition<Test>)
    }

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[serde(tag="apiVersion", rename_all="lowercase")]
    enum ApiVersions {
        V1Alpha(V1Alpha),
    }

    #[test]
    fn testing() {
        let i1 = r#"{"apiVersion":"v1alpha","kind":"Test","metadata":{"name":"","labels":{},"annotations":{}},"spec":{"yolo":"test"}}"#;

        let i2 = ApiVersions::V1Alpha(
            V1Alpha::Test(
                ResourceDefinition::new(
                    "v1alpha",
                    "Test",
                    Default::default(),
                    Test{ yolo: "test".to_string() }
                )));

        let o1 = serde_json::to_string(&i2).unwrap();
        let o2 = serde_json::from_str::<ApiVersions>(&i1).unwrap();

        dbg!(&i1);
        dbg!(&i2);
        dbg!(&o1);
        dbg!(&o2);


        assert_eq!(i1, o1);
        assert_eq!(i2, o2);
    }
}