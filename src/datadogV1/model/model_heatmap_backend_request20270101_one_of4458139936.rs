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
pub struct HeatmapBackendRequest20270101OneOf4458139936 {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "process_query")]
    pub process_query: crate::datadogV1::model::ProcessQueryDefinition,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "style")]
    pub style: Option<crate::datadogV1::model::RequestStyle>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl HeatmapBackendRequest20270101OneOf4458139936 {
    pub fn new(process_query: crate::datadogV1::model::ProcessQueryDefinition) -> HeatmapBackendRequest20270101OneOf4458139936 {
        HeatmapBackendRequest20270101OneOf4458139936 {
            process_query,
            style: None,
            _unparsed: false,
        }
    }
    
    pub fn style(mut self, value: crate::datadogV1::model::RequestStyle) -> Self {
        self.style = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for HeatmapBackendRequest20270101OneOf4458139936 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HeatmapBackendRequest20270101OneOf4458139936Visitor;
        impl<'a> Visitor<'a> for HeatmapBackendRequest20270101OneOf4458139936Visitor {
            type Value = HeatmapBackendRequest20270101OneOf4458139936;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut process_query: Option<crate::datadogV1::model::ProcessQueryDefinition> = None;
                let mut style: Option<crate::datadogV1::model::RequestStyle> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "process_query" => {
                            process_query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "style" => {
                            if v.is_null() {
                                continue;
                            }
                            style = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let process_query = process_query.ok_or_else(|| M::Error::missing_field("process_query"))?;

                let content = HeatmapBackendRequest20270101OneOf4458139936 {
                    process_query,
                    style,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(HeatmapBackendRequest20270101OneOf4458139936Visitor)
    }
}
