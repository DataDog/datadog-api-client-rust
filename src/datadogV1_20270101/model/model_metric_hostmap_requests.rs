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
pub struct MetricHostmapRequests {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "fill")]
    pub fill: Option<crate::datadogV1_20270101::model::HostmapTileDefFillRequest>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "size")]
    pub size: Option<crate::datadogV1_20270101::model::HostmapTileDefSizeRequest>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl MetricHostmapRequests {
    pub fn new() -> MetricHostmapRequests {
        MetricHostmapRequests {
            fill: None,
            size: None,
            _unparsed: false,
        }
    }
    
    pub fn fill(mut self, value: crate::datadogV1_20270101::model::HostmapTileDefFillRequest) -> Self {
        self.fill = Some(value);
        self
    }
    
    pub fn size(mut self, value: crate::datadogV1_20270101::model::HostmapTileDefSizeRequest) -> Self {
        self.size = Some(value);
        self
    }
    
}


impl Default for MetricHostmapRequests {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for MetricHostmapRequests {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MetricHostmapRequestsVisitor;
        impl<'a> Visitor<'a> for MetricHostmapRequestsVisitor {
            type Value = MetricHostmapRequests;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut fill: Option<crate::datadogV1_20270101::model::HostmapTileDefFillRequest> = None;
                let mut size: Option<crate::datadogV1_20270101::model::HostmapTileDefSizeRequest> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "fill" => {
                            if v.is_null() {
                                continue;
                            }
                            fill = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "size" => {
                            if v.is_null() {
                                continue;
                            }
                            size = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }

                let content = MetricHostmapRequests {
                    fill,
                    size,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MetricHostmapRequestsVisitor)
    }
}
