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
pub struct DistributionBackendRequest20270101OneOf4455770976 {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "style")]
    pub style: Option<crate::datadogV1::model::RequestStyle>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "synthetics_batches")]
    pub synthetics_batches: crate::datadogV1::model::LogQueryDefinition,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl DistributionBackendRequest20270101OneOf4455770976 {
    pub fn new(synthetics_batches: crate::datadogV1::model::LogQueryDefinition) -> DistributionBackendRequest20270101OneOf4455770976 {
        DistributionBackendRequest20270101OneOf4455770976 {
            style: None,
            synthetics_batches,
            _unparsed: false,
        }
    }
    
    pub fn style(mut self, value: crate::datadogV1::model::RequestStyle) -> Self {
        self.style = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for DistributionBackendRequest20270101OneOf4455770976 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DistributionBackendRequest20270101OneOf4455770976Visitor;
        impl<'a> Visitor<'a> for DistributionBackendRequest20270101OneOf4455770976Visitor {
            type Value = DistributionBackendRequest20270101OneOf4455770976;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut style: Option<crate::datadogV1::model::RequestStyle> = None;
                let mut synthetics_batches: Option<crate::datadogV1::model::LogQueryDefinition> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "style" => {
                            if v.is_null() {
                                continue;
                            }
                            style = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "synthetics_batches" => {
                            synthetics_batches = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let synthetics_batches = synthetics_batches.ok_or_else(|| M::Error::missing_field("synthetics_batches"))?;

                let content = DistributionBackendRequest20270101OneOf4455770976 {
                    style,
                    synthetics_batches,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DistributionBackendRequest20270101OneOf4455770976Visitor)
    }
}
