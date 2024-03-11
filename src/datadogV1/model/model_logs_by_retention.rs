// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object containing logs usage data broken down by retention period.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogsByRetention {
    /// Indexed logs usage summary for each organization for each retention period with usage.
    #[serde(rename = "orgs")]
    pub orgs: Option<crate::datadogV1::model::LogsByRetentionOrgs>,
    /// Aggregated index logs usage for each retention period with usage.
    #[serde(rename = "usage")]
    pub usage: Option<Vec<crate::datadogV1::model::LogsRetentionAggSumUsage>>,
    /// Object containing a summary of indexed logs usage by retention period for a single month.
    #[serde(rename = "usage_by_month")]
    pub usage_by_month: Option<crate::datadogV1::model::LogsByRetentionMonthlyUsage>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogsByRetention {
    pub fn new() -> LogsByRetention {
        LogsByRetention {
            orgs: None,
            usage: None,
            usage_by_month: None,
            _unparsed: false,
        }
    }

    pub fn orgs(&mut self, value: crate::datadogV1::model::LogsByRetentionOrgs) -> &mut Self {
        self.orgs = Some(value);
        self
    }

    pub fn usage(
        &mut self,
        value: Vec<crate::datadogV1::model::LogsRetentionAggSumUsage>,
    ) -> &mut Self {
        self.usage = Some(value);
        self
    }

    pub fn usage_by_month(
        &mut self,
        value: crate::datadogV1::model::LogsByRetentionMonthlyUsage,
    ) -> &mut Self {
        self.usage_by_month = Some(value);
        self
    }
}

impl Default for LogsByRetention {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for LogsByRetention {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsByRetentionVisitor;
        impl<'a> Visitor<'a> for LogsByRetentionVisitor {
            type Value = LogsByRetention;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut orgs: Option<crate::datadogV1::model::LogsByRetentionOrgs> = None;
                let mut usage: Option<Vec<crate::datadogV1::model::LogsRetentionAggSumUsage>> =
                    None;
                let mut usage_by_month: Option<
                    crate::datadogV1::model::LogsByRetentionMonthlyUsage,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "orgs" => {
                            if v.is_null() {
                                continue;
                            }
                            orgs = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "usage" => {
                            if v.is_null() {
                                continue;
                            }
                            usage = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "usage_by_month" => {
                            if v.is_null() {
                                continue;
                            }
                            usage_by_month =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = LogsByRetention {
                    orgs,
                    usage,
                    usage_by_month,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsByRetentionVisitor)
    }
}
