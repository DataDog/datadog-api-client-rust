// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Paging attributes.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringSignalsListResponseMetaPage {
    /// The cursor used to get the next results, if any. To make the next request, use the same
    /// parameters with the addition of the `page[cursor]`.
    #[serde(rename = "after")]
    pub after: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringSignalsListResponseMetaPage {
    pub fn new() -> SecurityMonitoringSignalsListResponseMetaPage {
        SecurityMonitoringSignalsListResponseMetaPage {
            after: None,
            _unparsed: false,
        }
    }

    pub fn after(mut self, value: String) -> Self {
        self.after = Some(value);
        self
    }
}

impl Default for SecurityMonitoringSignalsListResponseMetaPage {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SecurityMonitoringSignalsListResponseMetaPage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringSignalsListResponseMetaPageVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringSignalsListResponseMetaPageVisitor {
            type Value = SecurityMonitoringSignalsListResponseMetaPage;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut after: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "after" => {
                            if v.is_null() {
                                continue;
                            }
                            after = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SecurityMonitoringSignalsListResponseMetaPage { after, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringSignalsListResponseMetaPageVisitor)
    }
}
