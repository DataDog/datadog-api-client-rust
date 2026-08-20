// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a journey funnel response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsJourneyFunnelResponseAttributes {
    /// Conversion rate from the first step to the last step.
    #[serde(rename = "end_to_end_conversion_rate")]
    pub end_to_end_conversion_rate: f64,
    /// Elapsed time statistics (min/max/avg in milliseconds).
    #[serde(rename = "end_to_end_elapsed_time")]
    pub end_to_end_elapsed_time: crate::datadogV2::model::ProductAnalyticsElapsedTime,
    /// The funnel steps, in the order given by the search expression.
    #[serde(rename = "funnel_steps")]
    pub funnel_steps: Vec<crate::datadogV2::model::ProductAnalyticsJourneyFunnelStep>,
    /// Number of entities that entered the funnel.
    #[serde(rename = "initial_count")]
    pub initial_count: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsJourneyFunnelResponseAttributes {
    pub fn new(
        end_to_end_conversion_rate: f64,
        end_to_end_elapsed_time: crate::datadogV2::model::ProductAnalyticsElapsedTime,
        funnel_steps: Vec<crate::datadogV2::model::ProductAnalyticsJourneyFunnelStep>,
        initial_count: i64,
    ) -> ProductAnalyticsJourneyFunnelResponseAttributes {
        ProductAnalyticsJourneyFunnelResponseAttributes {
            end_to_end_conversion_rate,
            end_to_end_elapsed_time,
            funnel_steps,
            initial_count,
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

impl<'de> Deserialize<'de> for ProductAnalyticsJourneyFunnelResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsJourneyFunnelResponseAttributesVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsJourneyFunnelResponseAttributesVisitor {
            type Value = ProductAnalyticsJourneyFunnelResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut end_to_end_conversion_rate: Option<f64> = None;
                let mut end_to_end_elapsed_time: Option<
                    crate::datadogV2::model::ProductAnalyticsElapsedTime,
                > = None;
                let mut funnel_steps: Option<
                    Vec<crate::datadogV2::model::ProductAnalyticsJourneyFunnelStep>,
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
                            end_to_end_conversion_rate =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "end_to_end_elapsed_time" => {
                            end_to_end_elapsed_time =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "funnel_steps" => {
                            funnel_steps =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "initial_count" => {
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
                let end_to_end_conversion_rate = end_to_end_conversion_rate
                    .ok_or_else(|| M::Error::missing_field("end_to_end_conversion_rate"))?;
                let end_to_end_elapsed_time = end_to_end_elapsed_time
                    .ok_or_else(|| M::Error::missing_field("end_to_end_elapsed_time"))?;
                let funnel_steps =
                    funnel_steps.ok_or_else(|| M::Error::missing_field("funnel_steps"))?;
                let initial_count =
                    initial_count.ok_or_else(|| M::Error::missing_field("initial_count"))?;

                let content = ProductAnalyticsJourneyFunnelResponseAttributes {
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

        deserializer.deserialize_any(ProductAnalyticsJourneyFunnelResponseAttributesVisitor)
    }
}
