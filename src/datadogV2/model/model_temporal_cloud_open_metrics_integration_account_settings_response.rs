// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Settings configured on the Temporal Cloud OpenMetrics integration account.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TemporalCloudOpenMetricsIntegrationAccountSettingsResponse {
    /// When enabled, Datadog collects Temporal Cloud's [opt-in labels](<https://docs.temporal.io/cloud/metrics/openmetrics/metrics-reference#opt-in-labels>) (activity type, worker deployment name, and worker build ID) for more granularity on your existing metrics. Enabling them may increase metric collection latency.
    #[serde(rename = "include_opt_in_labels")]
    pub include_opt_in_labels: Option<bool>,
    /// Comma-separated list of Temporal Cloud namespaces to collect metrics for. An entry can use `*` as a wildcard; for example, `prod-*` matches every namespace starting with `prod-`. Empty or absent when metrics are collected for all namespaces.
    #[serde(rename = "namespaces_to_include")]
    pub namespaces_to_include: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TemporalCloudOpenMetricsIntegrationAccountSettingsResponse {
    pub fn new() -> TemporalCloudOpenMetricsIntegrationAccountSettingsResponse {
        TemporalCloudOpenMetricsIntegrationAccountSettingsResponse {
            include_opt_in_labels: None,
            namespaces_to_include: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn include_opt_in_labels(mut self, value: bool) -> Self {
        self.include_opt_in_labels = Some(value);
        self
    }

    pub fn namespaces_to_include(mut self, value: String) -> Self {
        self.namespaces_to_include = Some(value);
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

impl Default for TemporalCloudOpenMetricsIntegrationAccountSettingsResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TemporalCloudOpenMetricsIntegrationAccountSettingsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TemporalCloudOpenMetricsIntegrationAccountSettingsResponseVisitor;
        impl<'a> Visitor<'a> for TemporalCloudOpenMetricsIntegrationAccountSettingsResponseVisitor {
            type Value = TemporalCloudOpenMetricsIntegrationAccountSettingsResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut include_opt_in_labels: Option<bool> = None;
                let mut namespaces_to_include: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "include_opt_in_labels" => {
                            if v.is_null() {
                                continue;
                            }
                            include_opt_in_labels =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "namespaces_to_include" => {
                            if v.is_null() {
                                continue;
                            }
                            namespaces_to_include =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = TemporalCloudOpenMetricsIntegrationAccountSettingsResponse {
                    include_opt_in_labels,
                    namespaces_to_include,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(TemporalCloudOpenMetricsIntegrationAccountSettingsResponseVisitor)
    }
}
