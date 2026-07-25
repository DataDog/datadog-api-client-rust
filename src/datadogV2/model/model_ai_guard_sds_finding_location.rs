// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The location of a sensitive data match within the evaluated request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AIGuardSdsFindingLocation {
    /// The end character index (exclusive) of the sensitive data match.
    #[serde(rename = "end_index_exclusive")]
    pub end_index_exclusive: i64,
    /// The JSON path to the field containing the sensitive data.
    #[serde(rename = "path")]
    pub path: String,
    /// The start character index of the sensitive data match.
    #[serde(rename = "start_index")]
    pub start_index: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AIGuardSdsFindingLocation {
    pub fn new(
        end_index_exclusive: i64,
        path: String,
        start_index: i64,
    ) -> AIGuardSdsFindingLocation {
        AIGuardSdsFindingLocation {
            end_index_exclusive,
            path,
            start_index,
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

impl<'de> Deserialize<'de> for AIGuardSdsFindingLocation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AIGuardSdsFindingLocationVisitor;
        impl<'a> Visitor<'a> for AIGuardSdsFindingLocationVisitor {
            type Value = AIGuardSdsFindingLocation;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut end_index_exclusive: Option<i64> = None;
                let mut path: Option<String> = None;
                let mut start_index: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "end_index_exclusive" => {
                            end_index_exclusive =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "path" => {
                            path = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "start_index" => {
                            start_index =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let end_index_exclusive = end_index_exclusive
                    .ok_or_else(|| M::Error::missing_field("end_index_exclusive"))?;
                let path = path.ok_or_else(|| M::Error::missing_field("path"))?;
                let start_index =
                    start_index.ok_or_else(|| M::Error::missing_field("start_index"))?;

                let content = AIGuardSdsFindingLocation {
                    end_index_exclusive,
                    path,
                    start_index,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AIGuardSdsFindingLocationVisitor)
    }
}
