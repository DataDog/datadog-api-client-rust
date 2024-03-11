// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A response list of all service level objective deleted.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SLODeleteResponse {
    /// An array containing the ID of the deleted service level objective object.
    #[serde(rename = "data")]
    pub data: Option<Vec<String>>,
    /// An dictionary containing the ID of the SLO as key and a deletion error as value.
    #[serde(rename = "errors")]
    pub errors: Option<std::collections::BTreeMap<String, String>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SLODeleteResponse {
    pub fn new() -> SLODeleteResponse {
        SLODeleteResponse {
            data: None,
            errors: None,
            _unparsed: false,
        }
    }

    pub fn data(&mut self, value: Vec<String>) -> &mut Self {
        self.data = Some(value);
        self
    }

    pub fn errors(&mut self, value: std::collections::BTreeMap<String, String>) -> &mut Self {
        self.errors = Some(value);
        self
    }
}

impl Default for SLODeleteResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SLODeleteResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SLODeleteResponseVisitor;
        impl<'a> Visitor<'a> for SLODeleteResponseVisitor {
            type Value = SLODeleteResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data: Option<Vec<String>> = None;
                let mut errors: Option<std::collections::BTreeMap<String, String>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data" => {
                            if v.is_null() {
                                continue;
                            }
                            data = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "errors" => {
                            if v.is_null() {
                                continue;
                            }
                            errors = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SLODeleteResponse {
                    data,
                    errors,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SLODeleteResponseVisitor)
    }
}
