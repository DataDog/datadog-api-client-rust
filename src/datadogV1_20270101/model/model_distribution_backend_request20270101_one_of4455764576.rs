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
pub struct DistributionBackendRequest20270101OneOf4455764576 {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "network_query")]
    pub network_query: crate::datadogV1_20270101::model::LogQueryDefinition,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "style")]
    pub style: Option<crate::datadogV1_20270101::model::RequestStyle>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl DistributionBackendRequest20270101OneOf4455764576 {
    pub fn new(network_query: crate::datadogV1_20270101::model::LogQueryDefinition) -> DistributionBackendRequest20270101OneOf4455764576 {
        DistributionBackendRequest20270101OneOf4455764576 {
            network_query,
            style: None,
            _unparsed: false,
        }
    }
    
    pub fn style(mut self, value: crate::datadogV1_20270101::model::RequestStyle) -> Self {
        self.style = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for DistributionBackendRequest20270101OneOf4455764576 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DistributionBackendRequest20270101OneOf4455764576Visitor;
        impl<'a> Visitor<'a> for DistributionBackendRequest20270101OneOf4455764576Visitor {
            type Value = DistributionBackendRequest20270101OneOf4455764576;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut network_query: Option<crate::datadogV1_20270101::model::LogQueryDefinition> = None;
                let mut style: Option<crate::datadogV1_20270101::model::RequestStyle> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "network_query" => {
                            network_query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let network_query = network_query.ok_or_else(|| M::Error::missing_field("network_query"))?;

                let content = DistributionBackendRequest20270101OneOf4455764576 {
                    network_query,
                    style,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DistributionBackendRequest20270101OneOf4455764576Visitor)
    }
}
