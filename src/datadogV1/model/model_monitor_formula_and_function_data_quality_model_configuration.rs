// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Tuning options for the anomaly detection model used by the monitor.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MonitorFormulaAndFunctionDataQualityModelConfiguration {
    /// Number of days after which an open alert is automatically resolved.
    /// When unset, alerts stay open until the measure returns within bounds.
    #[serde(rename = "auto_resolve_days")]
    pub auto_resolve_days: Option<i32>,
    /// Whether to alert when the measure stops changing entirely.
    /// Defaults to `true`.
    #[serde(rename = "enable_flatline_detection")]
    pub enable_flatline_detection: Option<bool>,
    /// Function applied to the measure before it is compared against the predicted bounds.
    #[serde(rename = "function")]
    pub function: Option<crate::datadogV1::model::MonitorFormulaAndFunctionDataQualityDiffFunction>,
    /// Minimum distance between the predicted value and the lower bound. Widening the
    /// lower bound to at least this size suppresses alerts on small downward deviations.
    /// When unset, no minimum is enforced.
    #[serde(rename = "min_lower_bound_size")]
    pub min_lower_bound_size: Option<f64>,
    /// Minimum distance between the predicted value and the upper bound. Widening the
    /// upper bound to at least this size suppresses alerts on small upward deviations.
    /// When unset, no minimum is enforced.
    #[serde(rename = "min_upper_bound_size")]
    pub min_upper_bound_size: Option<f64>,
    /// Restricts which predicted bound the monitor alerts on. `UPPER_ONLY` alerts only when
    /// the measure rises above the upper bound, `LOWER_ONLY` only when it falls below the
    /// lower bound. When unset, the monitor alerts on both.
    #[serde(rename = "model_bounds_override")]
    pub model_bounds_override:
        Option<crate::datadogV1::model::MonitorFormulaAndFunctionDataQualityModelBoundsOverride>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MonitorFormulaAndFunctionDataQualityModelConfiguration {
    pub fn new() -> MonitorFormulaAndFunctionDataQualityModelConfiguration {
        MonitorFormulaAndFunctionDataQualityModelConfiguration {
            auto_resolve_days: None,
            enable_flatline_detection: None,
            function: None,
            min_lower_bound_size: None,
            min_upper_bound_size: None,
            model_bounds_override: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn auto_resolve_days(mut self, value: i32) -> Self {
        self.auto_resolve_days = Some(value);
        self
    }

    pub fn enable_flatline_detection(mut self, value: bool) -> Self {
        self.enable_flatline_detection = Some(value);
        self
    }

    pub fn function(
        mut self,
        value: crate::datadogV1::model::MonitorFormulaAndFunctionDataQualityDiffFunction,
    ) -> Self {
        self.function = Some(value);
        self
    }

    pub fn min_lower_bound_size(mut self, value: f64) -> Self {
        self.min_lower_bound_size = Some(value);
        self
    }

    pub fn min_upper_bound_size(mut self, value: f64) -> Self {
        self.min_upper_bound_size = Some(value);
        self
    }

    pub fn model_bounds_override(
        mut self,
        value: crate::datadogV1::model::MonitorFormulaAndFunctionDataQualityModelBoundsOverride,
    ) -> Self {
        self.model_bounds_override = Some(value);
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

impl Default for MonitorFormulaAndFunctionDataQualityModelConfiguration {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for MonitorFormulaAndFunctionDataQualityModelConfiguration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MonitorFormulaAndFunctionDataQualityModelConfigurationVisitor;
        impl<'a> Visitor<'a> for MonitorFormulaAndFunctionDataQualityModelConfigurationVisitor {
            type Value = MonitorFormulaAndFunctionDataQualityModelConfiguration;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut auto_resolve_days: Option<i32> = None;
                let mut enable_flatline_detection: Option<bool> = None;
                let mut function: Option<
                    crate::datadogV1::model::MonitorFormulaAndFunctionDataQualityDiffFunction,
                > = None;
                let mut min_lower_bound_size: Option<f64> = None;
                let mut min_upper_bound_size: Option<f64> = None;
                let mut model_bounds_override: Option<crate::datadogV1::model::MonitorFormulaAndFunctionDataQualityModelBoundsOverride> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "auto_resolve_days" => {
                            if v.is_null() {
                                continue;
                            }
                            auto_resolve_days =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enable_flatline_detection" => {
                            if v.is_null() {
                                continue;
                            }
                            enable_flatline_detection =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "function" => {
                            if v.is_null() {
                                continue;
                            }
                            function = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _function) = function {
                                match _function {
                                    crate::datadogV1::model::MonitorFormulaAndFunctionDataQualityDiffFunction::UnparsedObject(_function) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "min_lower_bound_size" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            min_lower_bound_size =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "min_upper_bound_size" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            min_upper_bound_size =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "model_bounds_override" => {
                            if v.is_null() {
                                continue;
                            }
                            model_bounds_override =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _model_bounds_override) = model_bounds_override {
                                match _model_bounds_override {
                                    crate::datadogV1::model::MonitorFormulaAndFunctionDataQualityModelBoundsOverride::UnparsedObject(_model_bounds_override) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = MonitorFormulaAndFunctionDataQualityModelConfiguration {
                    auto_resolve_days,
                    enable_flatline_detection,
                    function,
                    min_lower_bound_size,
                    min_upper_bound_size,
                    model_bounds_override,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MonitorFormulaAndFunctionDataQualityModelConfigurationVisitor)
    }
}
