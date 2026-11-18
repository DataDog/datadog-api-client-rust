// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Alert thresholds of a Product Analytics monitor.
/// Keep `critical` in sync with the comparison in the monitor query; the query is not derived from these values.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsMonitorThresholds {
    /// Value that triggers a critical alert.
    #[serde(rename = "critical")]
    pub critical: Option<f64>,
    /// Value at which a critical alert recovers.
    #[serde(rename = "critical_recovery")]
    pub critical_recovery: Option<f64>,
    /// Value that triggers a warning.
    #[serde(rename = "warning")]
    pub warning: Option<f64>,
    /// Value at which a warning recovers.
    #[serde(rename = "warning_recovery")]
    pub warning_recovery: Option<f64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsMonitorThresholds {
    pub fn new() -> ProductAnalyticsMonitorThresholds {
        ProductAnalyticsMonitorThresholds {
            critical: None,
            critical_recovery: None,
            warning: None,
            warning_recovery: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn critical(mut self, value: f64) -> Self {
        self.critical = Some(value);
        self
    }

    pub fn critical_recovery(mut self, value: f64) -> Self {
        self.critical_recovery = Some(value);
        self
    }

    pub fn warning(mut self, value: f64) -> Self {
        self.warning = Some(value);
        self
    }

    pub fn warning_recovery(mut self, value: f64) -> Self {
        self.warning_recovery = Some(value);
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

impl Default for ProductAnalyticsMonitorThresholds {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ProductAnalyticsMonitorThresholds {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsMonitorThresholdsVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsMonitorThresholdsVisitor {
            type Value = ProductAnalyticsMonitorThresholds;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut critical: Option<f64> = None;
                let mut critical_recovery: Option<f64> = None;
                let mut warning: Option<f64> = None;
                let mut warning_recovery: Option<f64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "critical" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            critical = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "critical_recovery" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            critical_recovery =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "warning" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            warning = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "warning_recovery" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            warning_recovery =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ProductAnalyticsMonitorThresholds {
                    critical,
                    critical_recovery,
                    warning,
                    warning_recovery,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsMonitorThresholdsVisitor)
    }
}
