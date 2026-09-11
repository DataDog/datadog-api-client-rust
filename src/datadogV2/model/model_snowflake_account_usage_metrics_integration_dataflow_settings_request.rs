// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Settings of the account usage metrics dataflow. Only the fields provided are changed.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SnowflakeAccountUsageMetricsIntegrationDataflowSettingsRequest {
    /// Period each metric aggregates over. Set to `true` to aggregate the past 24 hours on a rolling basis, or `false` to aggregate the current day so far. Defaults to `false`.
    #[serde(rename = "account_usage_metrics_aggregate_last_24h")]
    pub account_usage_metrics_aggregate_last_24h: Option<bool>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SnowflakeAccountUsageMetricsIntegrationDataflowSettingsRequest {
    pub fn new() -> SnowflakeAccountUsageMetricsIntegrationDataflowSettingsRequest {
        SnowflakeAccountUsageMetricsIntegrationDataflowSettingsRequest {
            account_usage_metrics_aggregate_last_24h: None,
            _unparsed: false,
        }
    }

    pub fn account_usage_metrics_aggregate_last_24h(mut self, value: bool) -> Self {
        self.account_usage_metrics_aggregate_last_24h = Some(value);
        self
    }
}

impl Default for SnowflakeAccountUsageMetricsIntegrationDataflowSettingsRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SnowflakeAccountUsageMetricsIntegrationDataflowSettingsRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SnowflakeAccountUsageMetricsIntegrationDataflowSettingsRequestVisitor;
        impl<'a> Visitor<'a> for SnowflakeAccountUsageMetricsIntegrationDataflowSettingsRequestVisitor {
            type Value = SnowflakeAccountUsageMetricsIntegrationDataflowSettingsRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account_usage_metrics_aggregate_last_24h: Option<bool> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "account_usage_metrics_aggregate_last_24h" => {
                            if v.is_null() {
                                continue;
                            }
                            account_usage_metrics_aggregate_last_24h =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = SnowflakeAccountUsageMetricsIntegrationDataflowSettingsRequest {
                    account_usage_metrics_aggregate_last_24h,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(SnowflakeAccountUsageMetricsIntegrationDataflowSettingsRequestVisitor)
    }
}
