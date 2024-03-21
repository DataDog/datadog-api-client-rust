// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// SLO thresholds (target and optionally warning) for a single time window.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SLOThreshold {
    /// The target value for the service level indicator within the corresponding
    /// timeframe.
    #[serde(rename = "target")]
    pub target: f64,
    /// A string representation of the target that indicates its precision.
    /// It uses trailing zeros to show significant decimal places (for example `98.00`).
    ///
    /// Always included in service level objective responses. Ignored in
    /// create/update requests.
    #[serde(rename = "target_display")]
    pub target_display: Option<String>,
    /// The SLO time window options.
    #[serde(rename = "timeframe")]
    pub timeframe: crate::datadogV1::model::SLOTimeframe,
    /// The warning value for the service level objective.
    #[serde(rename = "warning")]
    pub warning: Option<f64>,
    /// A string representation of the warning target (see the description of
    /// the `target_display` field for details).
    ///
    /// Included in service level objective responses if a warning target exists.
    /// Ignored in create/update requests.
    #[serde(rename = "warning_display")]
    pub warning_display: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SLOThreshold {
    pub fn new(target: f64, timeframe: crate::datadogV1::model::SLOTimeframe) -> SLOThreshold {
        SLOThreshold {
            target,
            target_display: None,
            timeframe,
            warning: None,
            warning_display: None,
            _unparsed: false,
        }
    }

    pub fn target_display(mut self, value: String) -> Self {
        self.target_display = Some(value);
        self
    }

    pub fn warning(mut self, value: f64) -> Self {
        self.warning = Some(value);
        self
    }

    pub fn warning_display(mut self, value: String) -> Self {
        self.warning_display = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for SLOThreshold {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SLOThresholdVisitor;
        impl<'a> Visitor<'a> for SLOThresholdVisitor {
            type Value = SLOThreshold;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut target: Option<f64> = None;
                let mut target_display: Option<String> = None;
                let mut timeframe: Option<crate::datadogV1::model::SLOTimeframe> = None;
                let mut warning: Option<f64> = None;
                let mut warning_display: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "target" => {
                            target = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "target_display" => {
                            if v.is_null() {
                                continue;
                            }
                            target_display =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timeframe" => {
                            timeframe = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _timeframe) = timeframe {
                                match _timeframe {
                                    crate::datadogV1::model::SLOTimeframe::UnparsedObject(
                                        _timeframe,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "warning" => {
                            if v.is_null() {
                                continue;
                            }
                            warning = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "warning_display" => {
                            if v.is_null() {
                                continue;
                            }
                            warning_display =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let target = target.ok_or_else(|| M::Error::missing_field("target"))?;
                let timeframe = timeframe.ok_or_else(|| M::Error::missing_field("timeframe"))?;

                let content = SLOThreshold {
                    target,
                    target_display,
                    timeframe,
                    warning,
                    warning_display,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SLOThresholdVisitor)
    }
}
