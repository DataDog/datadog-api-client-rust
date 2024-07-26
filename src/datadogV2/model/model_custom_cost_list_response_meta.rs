// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Meta for the response from the List Custom Costs endpoints.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CustomCostListResponseMeta {
    /// Number of Custom Costs files returned by the List Custom Costs endpoint
    #[serde(rename = "total_filtered_count")]
    pub total_filtered_count: Option<i64>,
    /// Version of Custom Costs file
    #[serde(rename = "version")]
    pub version: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CustomCostListResponseMeta {
    pub fn new() -> CustomCostListResponseMeta {
        CustomCostListResponseMeta {
            total_filtered_count: None,
            version: None,
            _unparsed: false,
        }
    }

    pub fn total_filtered_count(mut self, value: i64) -> Self {
        self.total_filtered_count = Some(value);
        self
    }

    pub fn version(mut self, value: String) -> Self {
        self.version = Some(value);
        self
    }
}

impl Default for CustomCostListResponseMeta {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CustomCostListResponseMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CustomCostListResponseMetaVisitor;
        impl<'a> Visitor<'a> for CustomCostListResponseMetaVisitor {
            type Value = CustomCostListResponseMeta;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut total_filtered_count: Option<i64> = None;
                let mut version: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "total_filtered_count" => {
                            if v.is_null() {
                                continue;
                            }
                            total_filtered_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version" => {
                            if v.is_null() {
                                continue;
                            }
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = CustomCostListResponseMeta {
                    total_filtered_count,
                    version,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CustomCostListResponseMetaVisitor)
    }
}
