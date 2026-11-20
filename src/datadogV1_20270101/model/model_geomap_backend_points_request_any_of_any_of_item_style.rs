// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GeomapBackendPointsRequestAnyOfAnyOfItemStyle {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "palette")]
    pub palette: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl GeomapBackendPointsRequestAnyOfAnyOfItemStyle {
    pub fn new() -> GeomapBackendPointsRequestAnyOfAnyOfItemStyle {
        GeomapBackendPointsRequestAnyOfAnyOfItemStyle {
            palette: None,
            _unparsed: false,
        }
    }
    
    pub fn palette(mut self, value: String) -> Self {
        self.palette = Some(value);
        self
    }
    
}


impl Default for GeomapBackendPointsRequestAnyOfAnyOfItemStyle {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for GeomapBackendPointsRequestAnyOfAnyOfItemStyle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GeomapBackendPointsRequestAnyOfAnyOfItemStyleVisitor;
        impl<'a> Visitor<'a> for GeomapBackendPointsRequestAnyOfAnyOfItemStyleVisitor {
            type Value = GeomapBackendPointsRequestAnyOfAnyOfItemStyle;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut palette: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "palette" => {
                            if v.is_null() {
                                continue;
                            }
                            palette = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }

                let content = GeomapBackendPointsRequestAnyOfAnyOfItemStyle {
                    palette,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GeomapBackendPointsRequestAnyOfAnyOfItemStyleVisitor)
    }
}
