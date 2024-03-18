// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Response containing the indexed logs usage broken down by retention period for an organization during a given hour.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UsageLogsByRetentionResponse {
    /// Get hourly usage for indexed logs by retention period.
    #[serde(rename = "usage")]
    pub usage: Option<Vec<crate::datadogV1::model::UsageLogsByRetentionHour>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UsageLogsByRetentionResponse {
    pub fn new() -> UsageLogsByRetentionResponse {
        UsageLogsByRetentionResponse {
            usage: None,
            _unparsed: false,
        }
    }

    pub fn usage(mut self, value: Vec<crate::datadogV1::model::UsageLogsByRetentionHour>) -> Self {
        self.usage = Some(value);
        self
    }
}

impl Default for UsageLogsByRetentionResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UsageLogsByRetentionResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UsageLogsByRetentionResponseVisitor;
        impl<'a> Visitor<'a> for UsageLogsByRetentionResponseVisitor {
            type Value = UsageLogsByRetentionResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut usage: Option<Vec<crate::datadogV1::model::UsageLogsByRetentionHour>> =
                    None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "usage" => {
                            if v.is_null() {
                                continue;
                            }
                            usage = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = UsageLogsByRetentionResponse { usage, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UsageLogsByRetentionResponseVisitor)
    }
}
