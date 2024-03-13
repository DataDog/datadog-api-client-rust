// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object containing the options for a Synthetic test as a monitor
/// (for example, renotification).
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestOptionsMonitorOptions {
    /// Time interval before renotifying if the test is still failing
    /// (in minutes).
    #[serde(rename = "renotify_interval")]
    pub renotify_interval: Option<i64>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestOptionsMonitorOptions {
    pub fn new() -> SyntheticsTestOptionsMonitorOptions {
        SyntheticsTestOptionsMonitorOptions {
            renotify_interval: None,
            _unparsed: false,
        }
    }

    pub fn renotify_interval(mut self, value: i64) -> Self {
        self.renotify_interval = Some(value);
        self
    }
}

impl Default for SyntheticsTestOptionsMonitorOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestOptionsMonitorOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestOptionsMonitorOptionsVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestOptionsMonitorOptionsVisitor {
            type Value = SyntheticsTestOptionsMonitorOptions;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut renotify_interval: Option<i64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "renotify_interval" => {
                            if v.is_null() {
                                continue;
                            }
                            renotify_interval =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SyntheticsTestOptionsMonitorOptions {
                    renotify_interval,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestOptionsMonitorOptionsVisitor)
    }
}
