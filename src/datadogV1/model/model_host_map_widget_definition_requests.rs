// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// List of definitions.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct HostMapWidgetDefinitionRequests {
    /// Updated host map.
    #[serde(rename = "fill")]
    pub fill: Option<crate::datadogV1::model::HostMapRequest>,
    /// Updated host map.
    #[serde(rename = "size")]
    pub size: Option<crate::datadogV1::model::HostMapRequest>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl HostMapWidgetDefinitionRequests {
    pub fn new() -> HostMapWidgetDefinitionRequests {
        HostMapWidgetDefinitionRequests {
            fill: None,
            size: None,
            _unparsed: false,
        }
    }

    pub fn fill(mut self, value: crate::datadogV1::model::HostMapRequest) -> Self {
        self.fill = Some(value);
        self
    }

    pub fn size(mut self, value: crate::datadogV1::model::HostMapRequest) -> Self {
        self.size = Some(value);
        self
    }
}

impl Default for HostMapWidgetDefinitionRequests {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for HostMapWidgetDefinitionRequests {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HostMapWidgetDefinitionRequestsVisitor;
        impl<'a> Visitor<'a> for HostMapWidgetDefinitionRequestsVisitor {
            type Value = HostMapWidgetDefinitionRequests;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut fill: Option<crate::datadogV1::model::HostMapRequest> = None;
                let mut size: Option<crate::datadogV1::model::HostMapRequest> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "fill" => {
                            if v.is_null() {
                                continue;
                            }
                            fill = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "size" => {
                            if v.is_null() {
                                continue;
                            }
                            size = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = HostMapWidgetDefinitionRequests {
                    fill,
                    size,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(HostMapWidgetDefinitionRequestsVisitor)
    }
}
