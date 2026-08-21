// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A single step of the funnel with its conversion counts and timings.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsJourneyFunnelStep {
    /// Elapsed time statistics (min/max/avg in milliseconds).
    #[serde(rename = "elapsed_time_to_next_step")]
    pub elapsed_time_to_next_step: crate::datadogV2::model::ProductAnalyticsElapsedTime,
    /// Breakdown of this step by the requested group-by facets.
    #[serde(rename = "groups")]
    pub groups: Vec<crate::datadogV2::model::ProductAnalyticsJourneyFunnelStepGroup>,
    /// Label of the step, derived from the node alias.
    #[serde(rename = "label")]
    pub label: String,
    /// Unit of the elapsed time values.
    #[serde(rename = "unit")]
    pub unit: String,
    /// Value of the computed metric at this step.
    #[serde(rename = "value")]
    pub value: f64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsJourneyFunnelStep {
    pub fn new(
        elapsed_time_to_next_step: crate::datadogV2::model::ProductAnalyticsElapsedTime,
        groups: Vec<crate::datadogV2::model::ProductAnalyticsJourneyFunnelStepGroup>,
        label: String,
        unit: String,
        value: f64,
    ) -> ProductAnalyticsJourneyFunnelStep {
        ProductAnalyticsJourneyFunnelStep {
            elapsed_time_to_next_step,
            groups,
            label,
            unit,
            value,
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

impl<'de> Deserialize<'de> for ProductAnalyticsJourneyFunnelStep {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsJourneyFunnelStepVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsJourneyFunnelStepVisitor {
            type Value = ProductAnalyticsJourneyFunnelStep;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut elapsed_time_to_next_step: Option<
                    crate::datadogV2::model::ProductAnalyticsElapsedTime,
                > = None;
                let mut groups: Option<
                    Vec<crate::datadogV2::model::ProductAnalyticsJourneyFunnelStepGroup>,
                > = None;
                let mut label: Option<String> = None;
                let mut unit: Option<String> = None;
                let mut value: Option<f64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "elapsed_time_to_next_step" => {
                            elapsed_time_to_next_step =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "groups" => {
                            groups = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "label" => {
                            label = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "unit" => {
                            unit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "value" => {
                            value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let elapsed_time_to_next_step = elapsed_time_to_next_step
                    .ok_or_else(|| M::Error::missing_field("elapsed_time_to_next_step"))?;
                let groups = groups.ok_or_else(|| M::Error::missing_field("groups"))?;
                let label = label.ok_or_else(|| M::Error::missing_field("label"))?;
                let unit = unit.ok_or_else(|| M::Error::missing_field("unit"))?;
                let value = value.ok_or_else(|| M::Error::missing_field("value"))?;

                let content = ProductAnalyticsJourneyFunnelStep {
                    elapsed_time_to_next_step,
                    groups,
                    label,
                    unit,
                    value,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsJourneyFunnelStepVisitor)
    }
}
