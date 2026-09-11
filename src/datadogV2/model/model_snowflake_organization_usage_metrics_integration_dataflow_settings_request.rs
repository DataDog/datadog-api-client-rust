// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Settings of the organization usage metrics dataflow. Only the fields provided are changed.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SnowflakeOrganizationUsageMetricsIntegrationDataflowSettingsRequest {
    /// Period each metric aggregates over. Set to `true` to aggregate the past 24 hours on a rolling basis, or `false` to aggregate the current day so far. Defaults to `false`.
    #[serde(rename = "organization_usage_metrics_aggregate_last_24h")]
    pub organization_usage_metrics_aggregate_last_24h: Option<bool>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SnowflakeOrganizationUsageMetricsIntegrationDataflowSettingsRequest {
    pub fn new() -> SnowflakeOrganizationUsageMetricsIntegrationDataflowSettingsRequest {
        SnowflakeOrganizationUsageMetricsIntegrationDataflowSettingsRequest {
            organization_usage_metrics_aggregate_last_24h: None,
            _unparsed: false,
        }
    }

    pub fn organization_usage_metrics_aggregate_last_24h(mut self, value: bool) -> Self {
        self.organization_usage_metrics_aggregate_last_24h = Some(value);
        self
    }
}

impl Default for SnowflakeOrganizationUsageMetricsIntegrationDataflowSettingsRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SnowflakeOrganizationUsageMetricsIntegrationDataflowSettingsRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SnowflakeOrganizationUsageMetricsIntegrationDataflowSettingsRequestVisitor;
        impl<'a> Visitor<'a>
            for SnowflakeOrganizationUsageMetricsIntegrationDataflowSettingsRequestVisitor
        {
            type Value = SnowflakeOrganizationUsageMetricsIntegrationDataflowSettingsRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut organization_usage_metrics_aggregate_last_24h: Option<bool> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "organization_usage_metrics_aggregate_last_24h" => {
                            if v.is_null() {
                                continue;
                            }
                            organization_usage_metrics_aggregate_last_24h =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = SnowflakeOrganizationUsageMetricsIntegrationDataflowSettingsRequest {
                    organization_usage_metrics_aggregate_last_24h,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(
            SnowflakeOrganizationUsageMetricsIntegrationDataflowSettingsRequestVisitor,
        )
    }
}
