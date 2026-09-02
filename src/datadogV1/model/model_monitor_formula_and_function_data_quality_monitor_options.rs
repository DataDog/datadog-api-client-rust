// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Monitor configuration options for data quality queries.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MonitorFormulaAndFunctionDataQualityMonitorOptions {
    /// Crontab expression to override the default schedule.
    #[serde(rename = "crontab_override")]
    pub crontab_override: Option<String>,
    /// Custom SQL query for the monitor.
    #[serde(rename = "custom_sql")]
    pub custom_sql: Option<String>,
    /// Custom WHERE clause for the query.
    #[serde(rename = "custom_where")]
    pub custom_where: Option<String>,
    /// Columns to group results by.
    #[serde(rename = "group_by_columns")]
    pub group_by_columns: Option<Vec<String>>,
    /// Tuning options for the anomaly detection model used by the monitor.
    #[serde(rename = "model_configuration")]
    pub model_configuration:
        Option<crate::datadogV1::model::MonitorFormulaAndFunctionDataQualityModelConfiguration>,
    /// Override for the model type used in anomaly detection.
    #[serde(rename = "model_type_override")]
    pub model_type_override:
        Option<crate::datadogV1::model::MonitorFormulaAndFunctionDataQualityModelTypeOverride>,
    /// Sensitivity of the anomaly detection model, expressed as a multiplier on the width
    /// of the predicted bounds. Higher values widen the bounds and produce fewer alerts;
    /// lower values tighten them and produce more alerts. Defaults to `3.0`.
    #[serde(rename = "sensitivity")]
    pub sensitivity: Option<f64>,
    /// Configuration for a source to target monitor, which compares the same measure
    /// across two data entities and alerts on the difference between them.
    #[serde(rename = "source_to_target_config")]
    pub source_to_target_config:
        Option<crate::datadogV1::model::MonitorFormulaAndFunctionDataQualitySourceToTargetConfig>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MonitorFormulaAndFunctionDataQualityMonitorOptions {
    pub fn new() -> MonitorFormulaAndFunctionDataQualityMonitorOptions {
        MonitorFormulaAndFunctionDataQualityMonitorOptions {
            crontab_override: None,
            custom_sql: None,
            custom_where: None,
            group_by_columns: None,
            model_configuration: None,
            model_type_override: None,
            sensitivity: None,
            source_to_target_config: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn crontab_override(mut self, value: String) -> Self {
        self.crontab_override = Some(value);
        self
    }

    pub fn custom_sql(mut self, value: String) -> Self {
        self.custom_sql = Some(value);
        self
    }

    pub fn custom_where(mut self, value: String) -> Self {
        self.custom_where = Some(value);
        self
    }

    pub fn group_by_columns(mut self, value: Vec<String>) -> Self {
        self.group_by_columns = Some(value);
        self
    }

    pub fn model_configuration(
        mut self,
        value: crate::datadogV1::model::MonitorFormulaAndFunctionDataQualityModelConfiguration,
    ) -> Self {
        self.model_configuration = Some(value);
        self
    }

    pub fn model_type_override(
        mut self,
        value: crate::datadogV1::model::MonitorFormulaAndFunctionDataQualityModelTypeOverride,
    ) -> Self {
        self.model_type_override = Some(value);
        self
    }

    pub fn sensitivity(mut self, value: f64) -> Self {
        self.sensitivity = Some(value);
        self
    }

    pub fn source_to_target_config(
        mut self,
        value: crate::datadogV1::model::MonitorFormulaAndFunctionDataQualitySourceToTargetConfig,
    ) -> Self {
        self.source_to_target_config = Some(value);
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

impl Default for MonitorFormulaAndFunctionDataQualityMonitorOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for MonitorFormulaAndFunctionDataQualityMonitorOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MonitorFormulaAndFunctionDataQualityMonitorOptionsVisitor;
        impl<'a> Visitor<'a> for MonitorFormulaAndFunctionDataQualityMonitorOptionsVisitor {
            type Value = MonitorFormulaAndFunctionDataQualityMonitorOptions;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut crontab_override: Option<String> = None;
                let mut custom_sql: Option<String> = None;
                let mut custom_where: Option<String> = None;
                let mut group_by_columns: Option<Vec<String>> = None;
                let mut model_configuration: Option<
                    crate::datadogV1::model::MonitorFormulaAndFunctionDataQualityModelConfiguration,
                > = None;
                let mut model_type_override: Option<
                    crate::datadogV1::model::MonitorFormulaAndFunctionDataQualityModelTypeOverride,
                > = None;
                let mut sensitivity: Option<f64> = None;
                let mut source_to_target_config: Option<crate::datadogV1::model::MonitorFormulaAndFunctionDataQualitySourceToTargetConfig> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "crontab_override" => {
                            if v.is_null() {
                                continue;
                            }
                            crontab_override =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "custom_sql" => {
                            if v.is_null() {
                                continue;
                            }
                            custom_sql = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "custom_where" => {
                            if v.is_null() {
                                continue;
                            }
                            custom_where =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "group_by_columns" => {
                            if v.is_null() {
                                continue;
                            }
                            group_by_columns =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "model_configuration" => {
                            if v.is_null() {
                                continue;
                            }
                            model_configuration =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "model_type_override" => {
                            if v.is_null() {
                                continue;
                            }
                            model_type_override =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _model_type_override) = model_type_override {
                                match _model_type_override {
                                    crate::datadogV1::model::MonitorFormulaAndFunctionDataQualityModelTypeOverride::UnparsedObject(_model_type_override) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "sensitivity" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            sensitivity =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source_to_target_config" => {
                            if v.is_null() {
                                continue;
                            }
                            source_to_target_config =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = MonitorFormulaAndFunctionDataQualityMonitorOptions {
                    crontab_override,
                    custom_sql,
                    custom_where,
                    group_by_columns,
                    model_configuration,
                    model_type_override,
                    sensitivity,
                    source_to_target_config,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MonitorFormulaAndFunctionDataQualityMonitorOptionsVisitor)
    }
}
