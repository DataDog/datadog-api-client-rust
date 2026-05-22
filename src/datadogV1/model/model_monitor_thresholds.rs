// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// List of the different monitor threshold available.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MonitorThresholds {
    /// The monitor `CRITICAL` threshold.
    #[serde(rename = "critical")]
    pub critical: Option<f64>,
    /// Query evaluated as a dynamic `CRITICAL` threshold. Only supported on metric monitors with a formula query and options['variables']. Cannot be combined with static thresholds. This field is in preview.
    #[serde(rename = "critical_query")]
    pub critical_query: Option<String>,
    /// The monitor `CRITICAL` recovery threshold.
    #[serde(
        rename = "critical_recovery",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub critical_recovery: Option<Option<f64>>,
    /// Query evaluated as a dynamic `CRITICAL` recovery threshold. Only supported on metric monitors with a formula query and options['variables']. Cannot be combined with static thresholds. This field is in preview.
    #[serde(rename = "critical_recovery_query")]
    pub critical_recovery_query: Option<String>,
    /// The monitor `OK` threshold.
    #[serde(rename = "ok", default, with = "::serde_with::rust::double_option")]
    pub ok: Option<Option<f64>>,
    /// The monitor UNKNOWN threshold.
    #[serde(
        rename = "unknown",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub unknown: Option<Option<f64>>,
    /// The monitor `WARNING` threshold.
    #[serde(
        rename = "warning",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub warning: Option<Option<f64>>,
    /// The monitor `WARNING` recovery threshold.
    #[serde(
        rename = "warning_recovery",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub warning_recovery: Option<Option<f64>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MonitorThresholds {
    pub fn new() -> MonitorThresholds {
        MonitorThresholds {
            critical: None,
            critical_query: None,
            critical_recovery: None,
            critical_recovery_query: None,
            ok: None,
            unknown: None,
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

    pub fn critical_query(mut self, value: String) -> Self {
        self.critical_query = Some(value);
        self
    }

    pub fn critical_recovery(mut self, value: Option<f64>) -> Self {
        self.critical_recovery = Some(value);
        self
    }

    pub fn critical_recovery_query(mut self, value: String) -> Self {
        self.critical_recovery_query = Some(value);
        self
    }

    pub fn ok(mut self, value: Option<f64>) -> Self {
        self.ok = Some(value);
        self
    }

    pub fn unknown(mut self, value: Option<f64>) -> Self {
        self.unknown = Some(value);
        self
    }

    pub fn warning(mut self, value: Option<f64>) -> Self {
        self.warning = Some(value);
        self
    }

    pub fn warning_recovery(mut self, value: Option<f64>) -> Self {
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

impl Default for MonitorThresholds {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for MonitorThresholds {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MonitorThresholdsVisitor;
        impl<'a> Visitor<'a> for MonitorThresholdsVisitor {
            type Value = MonitorThresholds;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut critical: Option<f64> = None;
                let mut critical_query: Option<String> = None;
                let mut critical_recovery: Option<Option<f64>> = None;
                let mut critical_recovery_query: Option<String> = None;
                let mut ok: Option<Option<f64>> = None;
                let mut unknown: Option<Option<f64>> = None;
                let mut warning: Option<Option<f64>> = None;
                let mut warning_recovery: Option<Option<f64>> = None;
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
                        "critical_query" => {
                            if v.is_null() {
                                continue;
                            }
                            critical_query =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "critical_recovery" => {
                            if v.as_str() == Some("") {
                                continue;
                            }
                            critical_recovery =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "critical_recovery_query" => {
                            if v.is_null() {
                                continue;
                            }
                            critical_recovery_query =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ok" => {
                            if v.as_str() == Some("") {
                                continue;
                            }
                            ok = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "unknown" => {
                            if v.as_str() == Some("") {
                                continue;
                            }
                            unknown = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "warning" => {
                            if v.as_str() == Some("") {
                                continue;
                            }
                            warning = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "warning_recovery" => {
                            if v.as_str() == Some("") {
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

                let content = MonitorThresholds {
                    critical,
                    critical_query,
                    critical_recovery,
                    critical_recovery_query,
                    ok,
                    unknown,
                    warning,
                    warning_recovery,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MonitorThresholdsVisitor)
    }
}
