// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object containing the live span selection for the dashboard.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DashboardGlobalTime {
    /// Dashboard global time live_span selection
    #[serde(rename = "live_span")]
    pub live_span: Option<crate::datadogV1::model::DashboardGlobalTimeLiveSpan>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DashboardGlobalTime {
    pub fn new() -> DashboardGlobalTime {
        DashboardGlobalTime {
            live_span: None,
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
}

impl Default for DashboardGlobalTime {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for DashboardGlobalTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DashboardGlobalTimeVisitor;
        impl<'a> Visitor<'a> for DashboardGlobalTimeVisitor {
            type Value = DashboardGlobalTime;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut live_span: Option<crate::datadogV1::model::DashboardGlobalTimeLiveSpan> =
                    None;
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
                        &_ => {}
                    }
                }

                let content = DashboardGlobalTime {
                    live_span,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DashboardGlobalTimeVisitor)
    }
}
