// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Settings for updating the Temporal Cloud OpenMetrics integration account. Only the fields provided are changed.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TemporalCloudOpenMetricsIntegrationAccountSettingsUpdate {
    /// When enabled, Datadog collects Temporal Cloud's [opt-in labels](<https://docs.temporal.io/cloud/metrics/openmetrics/metrics-reference#opt-in-labels>) (activity type, worker deployment name, and worker build ID) for more granularity on your existing metrics. Enabling them may increase metric collection latency.
    #[serde(rename = "include_opt_in_labels")]
    pub include_opt_in_labels: Option<bool>,
    /// Comma-separated list of Temporal Cloud namespaces to collect metrics for. An entry can use `*` as a wildcard; for example, `prod-*` matches every namespace starting with `prod-`. Entries may contain only lowercase letters, digits, hyphens, periods and `*`; any other value is rejected with a `422` response. Omit it or leave it empty to collect metrics for all namespaces.
    #[serde(rename = "namespaces_to_include")]
    pub namespaces_to_include: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TemporalCloudOpenMetricsIntegrationAccountSettingsUpdate {
    pub fn new() -> TemporalCloudOpenMetricsIntegrationAccountSettingsUpdate {
        TemporalCloudOpenMetricsIntegrationAccountSettingsUpdate {
            include_opt_in_labels: None,
            namespaces_to_include: None,
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
}

impl Default for TemporalCloudOpenMetricsIntegrationAccountSettingsUpdate {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TemporalCloudOpenMetricsIntegrationAccountSettingsUpdate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TemporalCloudOpenMetricsIntegrationAccountSettingsUpdateVisitor;
        impl<'a> Visitor<'a> for TemporalCloudOpenMetricsIntegrationAccountSettingsUpdateVisitor {
            type Value = TemporalCloudOpenMetricsIntegrationAccountSettingsUpdate;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut include_opt_in_labels: Option<bool> = None;
                let mut namespaces_to_include: Option<String> = None;
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
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = TemporalCloudOpenMetricsIntegrationAccountSettingsUpdate {
                    include_opt_in_labels,
                    namespaces_to_include,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(TemporalCloudOpenMetricsIntegrationAccountSettingsUpdateVisitor)
    }
}
