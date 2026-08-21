// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Breakdown of a funnel step for one combination of group-by values.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsJourneyFunnelStepGroup {
    /// Number of entities in this group that reached the next step.
    #[serde(rename = "conversion_count")]
    pub conversion_count: i64,
    /// Elapsed time statistics (min/max/avg in milliseconds).
    #[serde(rename = "elapsed_time_to_next_step")]
    pub elapsed_time_to_next_step: crate::datadogV2::model::ProductAnalyticsElapsedTime,
    /// Group-by values identifying this cohort.
    #[serde(rename = "group_tags")]
    pub group_tags: Vec<String>,
    /// Value of the computed metric for this group at this step.
    #[serde(rename = "value")]
    pub value: f64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsJourneyFunnelStepGroup {
    pub fn new(
        conversion_count: i64,
        elapsed_time_to_next_step: crate::datadogV2::model::ProductAnalyticsElapsedTime,
        group_tags: Vec<String>,
        value: f64,
    ) -> ProductAnalyticsJourneyFunnelStepGroup {
        ProductAnalyticsJourneyFunnelStepGroup {
            conversion_count,
            elapsed_time_to_next_step,
            group_tags,
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

impl<'de> Deserialize<'de> for ProductAnalyticsJourneyFunnelStepGroup {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsJourneyFunnelStepGroupVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsJourneyFunnelStepGroupVisitor {
            type Value = ProductAnalyticsJourneyFunnelStepGroup;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut conversion_count: Option<i64> = None;
                let mut elapsed_time_to_next_step: Option<
                    crate::datadogV2::model::ProductAnalyticsElapsedTime,
                > = None;
                let mut group_tags: Option<Vec<String>> = None;
                let mut value: Option<f64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "conversion_count" => {
                            conversion_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "elapsed_time_to_next_step" => {
                            elapsed_time_to_next_step =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "group_tags" => {
                            group_tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let conversion_count =
                    conversion_count.ok_or_else(|| M::Error::missing_field("conversion_count"))?;
                let elapsed_time_to_next_step = elapsed_time_to_next_step
                    .ok_or_else(|| M::Error::missing_field("elapsed_time_to_next_step"))?;
                let group_tags = group_tags.ok_or_else(|| M::Error::missing_field("group_tags"))?;
                let value = value.ok_or_else(|| M::Error::missing_field("value"))?;

                let content = ProductAnalyticsJourneyFunnelStepGroup {
                    conversion_count,
                    elapsed_time_to_next_step,
                    group_tags,
                    value,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsJourneyFunnelStepGroupVisitor)
    }
}
