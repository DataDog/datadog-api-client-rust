// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CustomRuleRevisionTest {
    /// Expected violation count
    #[serde(rename = "annotation_count")]
    pub annotation_count: i64,
    /// Test code
    #[serde(rename = "code")]
    pub code: String,
    /// Test filename
    #[serde(rename = "filename")]
    pub filename: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CustomRuleRevisionTest {
    pub fn new(annotation_count: i64, code: String, filename: String) -> CustomRuleRevisionTest {
        CustomRuleRevisionTest {
            annotation_count,
            code,
            filename,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for CustomRuleRevisionTest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CustomRuleRevisionTestVisitor;
        impl<'a> Visitor<'a> for CustomRuleRevisionTestVisitor {
            type Value = CustomRuleRevisionTest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut annotation_count: Option<i64> = None;
                let mut code: Option<String> = None;
                let mut filename: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "annotation_count" => {
                            annotation_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "code" => {
                            code = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "filename" => {
                            filename = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let annotation_count =
                    annotation_count.ok_or_else(|| M::Error::missing_field("annotation_count"))?;
                let code = code.ok_or_else(|| M::Error::missing_field("code"))?;
                let filename = filename.ok_or_else(|| M::Error::missing_field("filename"))?;

                let content = CustomRuleRevisionTest {
                    annotation_count,
                    code,
                    filename,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CustomRuleRevisionTestVisitor)
    }
}
