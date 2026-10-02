// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// One step in the protocol's traffic exposure schedule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsPublicProtocolResponseDataAttributesExposureScheduleRolloutStepsItems {
    /// Fraction of traffic exposed during this rollout step.
    #[serde(rename = "exposure_ratio")]
    pub exposure_ratio: Option<f64>,
    /// Index of the group that contains this rollout step.
    #[serde(rename = "grouped_step_index")]
    pub grouped_step_index: Option<i64>,
    /// Duration of this rollout step, in milliseconds.
    #[serde(rename = "interval_ms")]
    pub interval_ms: Option<i64>,
    /// Whether this schedule entry represents a pause.
    #[serde(rename = "is_pause_record")]
    pub is_pause_record: Option<bool>,
    /// Position of this entry in the ordered configuration.
    #[serde(rename = "order_position")]
    pub order_position: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsPublicProtocolResponseDataAttributesExposureScheduleRolloutStepsItems {
    pub fn new() -> ExperimentsPublicProtocolResponseDataAttributesExposureScheduleRolloutStepsItems
    {
        ExperimentsPublicProtocolResponseDataAttributesExposureScheduleRolloutStepsItems {
            exposure_ratio: None,
            grouped_step_index: None,
            interval_ms: None,
            is_pause_record: None,
            order_position: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn exposure_ratio(mut self, value: f64) -> Self {
        self.exposure_ratio = Some(value);
        self
    }

    pub fn grouped_step_index(mut self, value: i64) -> Self {
        self.grouped_step_index = Some(value);
        self
    }

    pub fn interval_ms(mut self, value: i64) -> Self {
        self.interval_ms = Some(value);
        self
    }

    pub fn is_pause_record(mut self, value: bool) -> Self {
        self.is_pause_record = Some(value);
        self
    }

    pub fn order_position(mut self, value: i64) -> Self {
        self.order_position = Some(value);
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

impl Default for ExperimentsPublicProtocolResponseDataAttributesExposureScheduleRolloutStepsItems {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de>
    for ExperimentsPublicProtocolResponseDataAttributesExposureScheduleRolloutStepsItems
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsPublicProtocolResponseDataAttributesExposureScheduleRolloutStepsItemsVisitor;
        impl<'a> Visitor<'a> for ExperimentsPublicProtocolResponseDataAttributesExposureScheduleRolloutStepsItemsVisitor {
            type Value = ExperimentsPublicProtocolResponseDataAttributesExposureScheduleRolloutStepsItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut exposure_ratio: Option<f64> = None;
                let mut grouped_step_index: Option<i64> = None;
                let mut interval_ms: Option<i64> = None;
                let mut is_pause_record: Option<bool> = None;
                let mut order_position: Option<i64> = None;
                    let mut additional_properties: std::collections::BTreeMap<String, serde_json::Value> = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "exposure_ratio" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            exposure_ratio = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "grouped_step_index" => {
                            if v.is_null() {
                                continue;
                            }
                            grouped_step_index = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "interval_ms" => {
                            if v.is_null() {
                                continue;
                            }
                            interval_ms = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "is_pause_record" => {
                            if v.is_null() {
                                continue;
                            }
                            is_pause_record = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "order_position" => {
                            if v.is_null() {
                                continue;
                            }
                            order_position = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        },
                    }
                }

                let content = ExperimentsPublicProtocolResponseDataAttributesExposureScheduleRolloutStepsItems {
                    exposure_ratio,
                    grouped_step_index,
                    interval_ms,
                    is_pause_record,
                    order_position,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(
            ExperimentsPublicProtocolResponseDataAttributesExposureScheduleRolloutStepsItemsVisitor,
        )
    }
}
