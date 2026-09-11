// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Settings of the organization usage metrics dataflow.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SnowflakeOrganizationUsageMetricsIntegrationDataflowSettingsResponse {
    /// Period each metric aggregates over. When `true`, metrics aggregate the past 24 hours on a rolling basis; when `false`, they aggregate the current day so far.
    #[serde(rename = "organization_usage_metrics_aggregate_last_24h")]
    pub organization_usage_metrics_aggregate_last_24h: Option<bool>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SnowflakeOrganizationUsageMetricsIntegrationDataflowSettingsResponse {
    pub fn new() -> SnowflakeOrganizationUsageMetricsIntegrationDataflowSettingsResponse {
        SnowflakeOrganizationUsageMetricsIntegrationDataflowSettingsResponse {
            organization_usage_metrics_aggregate_last_24h: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn organization_usage_metrics_aggregate_last_24h(mut self, value: bool) -> Self {
        self.organization_usage_metrics_aggregate_last_24h = Some(value);
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

impl Default for SnowflakeOrganizationUsageMetricsIntegrationDataflowSettingsResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de>
    for SnowflakeOrganizationUsageMetricsIntegrationDataflowSettingsResponse
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SnowflakeOrganizationUsageMetricsIntegrationDataflowSettingsResponseVisitor;
        impl<'a> Visitor<'a>
            for SnowflakeOrganizationUsageMetricsIntegrationDataflowSettingsResponseVisitor
        {
            type Value = SnowflakeOrganizationUsageMetricsIntegrationDataflowSettingsResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut organization_usage_metrics_aggregate_last_24h: Option<bool> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
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
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content =
                    SnowflakeOrganizationUsageMetricsIntegrationDataflowSettingsResponse {
                        organization_usage_metrics_aggregate_last_24h,
                        additional_properties,
                        _unparsed,
                    };

                Ok(content)
            }
        }

        deserializer.deserialize_any(
            SnowflakeOrganizationUsageMetricsIntegrationDataflowSettingsResponseVisitor,
        )
    }
}
