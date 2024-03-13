// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object containing a summary of indexed logs usage by retention period for a single month.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogsByRetentionMonthlyUsage {
    /// The month for the usage.
    #[serde(rename = "date")]
    pub date: Option<String>,
    /// Indexed logs usage for each active retention for the month.
    #[serde(rename = "usage")]
    pub usage: Option<Vec<crate::datadogV1::model::LogsRetentionSumUsage>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogsByRetentionMonthlyUsage {
    pub fn new() -> LogsByRetentionMonthlyUsage {
        LogsByRetentionMonthlyUsage {
            date: None,
            usage: None,
            _unparsed: false,
        }
    }

    pub fn date(mut self, value: String) -> Self {
        self.date = Some(value);
        self
    }

    pub fn usage(mut self, value: Vec<crate::datadogV1::model::LogsRetentionSumUsage>) -> Self {
        self.usage = Some(value);
        self
    }
}

impl Default for LogsByRetentionMonthlyUsage {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for LogsByRetentionMonthlyUsage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsByRetentionMonthlyUsageVisitor;
        impl<'a> Visitor<'a> for LogsByRetentionMonthlyUsageVisitor {
            type Value = LogsByRetentionMonthlyUsage;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut date: Option<String> = None;
                let mut usage: Option<Vec<crate::datadogV1::model::LogsRetentionSumUsage>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "date" => {
                            if v.is_null() {
                                continue;
                            }
                            date = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "usage" => {
                            if v.is_null() {
                                continue;
                            }
                            usage = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = LogsByRetentionMonthlyUsage {
                    date,
                    usage,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsByRetentionMonthlyUsageVisitor)
    }
}
