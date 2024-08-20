// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Timeframe setting for the shared dashboard.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SharedDashboardUpdateRequestGlobalTime {
    /// Dashboard global time live_span selection
    #[serde(rename = "live_span")]
    pub live_span: Option<crate::datadogV1::model::DashboardGlobalTimeLiveSpan>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SharedDashboardUpdateRequestGlobalTime {
    pub fn new() -> SharedDashboardUpdateRequestGlobalTime {
        SharedDashboardUpdateRequestGlobalTime {
            live_span: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn live_span(
        mut self,
        value: crate::datadogV1::model::DashboardGlobalTimeLiveSpan,
    ) -> Self {
        self.live_span = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl Default for SharedDashboardUpdateRequestGlobalTime {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SharedDashboardUpdateRequestGlobalTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SharedDashboardUpdateRequestGlobalTimeVisitor;
        impl<'a> Visitor<'a> for SharedDashboardUpdateRequestGlobalTimeVisitor {
            type Value = SharedDashboardUpdateRequestGlobalTime;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut live_span: Option<crate::datadogV1::model::DashboardGlobalTimeLiveSpan> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "live_span" => {
                            if v.is_null() {
                                continue;
                            }
                            live_span = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _live_span) = live_span {
                                match _live_span {
                                    crate::datadogV1::model::DashboardGlobalTimeLiveSpan::UnparsedObject(_live_span) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SharedDashboardUpdateRequestGlobalTime {
                    live_span,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SharedDashboardUpdateRequestGlobalTimeVisitor)
    }
}
