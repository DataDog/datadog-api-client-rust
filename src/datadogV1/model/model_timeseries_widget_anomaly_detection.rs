// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Anomaly detection configuration for a timeseries widget.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TimeseriesWidgetAnomalyDetection {
    /// Sensitivity level for anomaly detection. Use `never_detect` to disable anomaly detection.
    #[serde(rename = "detection_sensitivity")]
    pub detection_sensitivity: crate::datadogV1::model::TimeseriesWidgetAnomalyDetectionSensitivity,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TimeseriesWidgetAnomalyDetection {
    pub fn new(
        detection_sensitivity: crate::datadogV1::model::TimeseriesWidgetAnomalyDetectionSensitivity,
    ) -> TimeseriesWidgetAnomalyDetection {
        TimeseriesWidgetAnomalyDetection {
            detection_sensitivity,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for TimeseriesWidgetAnomalyDetection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TimeseriesWidgetAnomalyDetectionVisitor;
        impl<'a> Visitor<'a> for TimeseriesWidgetAnomalyDetectionVisitor {
            type Value = TimeseriesWidgetAnomalyDetection;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut detection_sensitivity: Option<
                    crate::datadogV1::model::TimeseriesWidgetAnomalyDetectionSensitivity,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "detection_sensitivity" => {
                            detection_sensitivity =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _detection_sensitivity) = detection_sensitivity {
                                match _detection_sensitivity {
                                    crate::datadogV1::model::TimeseriesWidgetAnomalyDetectionSensitivity::UnparsedObject(_detection_sensitivity) => {
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
                let detection_sensitivity = detection_sensitivity
                    .ok_or_else(|| M::Error::missing_field("detection_sensitivity"))?;

                let content = TimeseriesWidgetAnomalyDetection {
                    detection_sensitivity,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TimeseriesWidgetAnomalyDetectionVisitor)
    }
}
