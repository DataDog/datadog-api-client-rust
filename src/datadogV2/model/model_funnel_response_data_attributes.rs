// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FunnelResponseDataAttributes {
    #[serde(rename = "end_to_end_conversion_rate")]
    pub end_to_end_conversion_rate: Option<f64>,
    #[serde(rename = "end_to_end_elapsed_time")]
    pub end_to_end_elapsed_time: Option<crate::datadogV2::model::FunnelResponseElapsedTime>,
    #[serde(rename = "funnel_steps")]
    pub funnel_steps:
        Option<Vec<crate::datadogV2::model::FunnelResponseDataAttributesFunnelStepsItems>>,
    #[serde(rename = "initial_count")]
    pub initial_count: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FunnelResponseDataAttributes {
    pub fn new() -> FunnelResponseDataAttributes {
        FunnelResponseDataAttributes {
            end_to_end_conversion_rate: None,
            end_to_end_elapsed_time: None,
            funnel_steps: None,
            initial_count: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn end_to_end_conversion_rate(mut self, value: f64) -> Self {
        self.end_to_end_conversion_rate = Some(value);
        self
    }

    pub fn end_to_end_elapsed_time(
        mut self,
        value: crate::datadogV2::model::FunnelResponseElapsedTime,
    ) -> Self {
        self.end_to_end_elapsed_time = Some(value);
        self
    }

    pub fn funnel_steps(
        mut self,
        value: Vec<crate::datadogV2::model::FunnelResponseDataAttributesFunnelStepsItems>,
    ) -> Self {
        self.funnel_steps = Some(value);
        self
    }

    pub fn initial_count(mut self, value: i64) -> Self {
        self.initial_count = Some(value);
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

impl Default for FunnelResponseDataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for FunnelResponseDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FunnelResponseDataAttributesVisitor;
        impl<'a> Visitor<'a> for FunnelResponseDataAttributesVisitor {
            type Value = FunnelResponseDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut end_to_end_conversion_rate: Option<f64> = None;
                let mut end_to_end_elapsed_time: Option<
                    crate::datadogV2::model::FunnelResponseElapsedTime,
                > = None;
                let mut funnel_steps: Option<
                    Vec<crate::datadogV2::model::FunnelResponseDataAttributesFunnelStepsItems>,
                > = None;
                let mut initial_count: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "end_to_end_conversion_rate" => {
                            if v.is_null() {
                                continue;
                            }
                            end_to_end_conversion_rate =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "end_to_end_elapsed_time" => {
                            if v.is_null() {
                                continue;
                            }
                            end_to_end_elapsed_time =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "funnel_steps" => {
                            if v.is_null() {
                                continue;
                            }
                            funnel_steps =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "initial_count" => {
                            if v.is_null() {
                                continue;
                            }
                            initial_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = FunnelResponseDataAttributes {
                    end_to_end_conversion_rate,
                    end_to_end_elapsed_time,
                    funnel_steps,
                    initial_count,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FunnelResponseDataAttributesVisitor)
    }
}
