// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of an aggregated waterfall response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AggregatedWaterfallResponseAttributes {
    /// The RUM application ID that was analyzed.
    #[serde(rename = "application_id")]
    pub application_id: String,
    /// Performance criteria to filter view instances by a metric threshold.
    #[serde(rename = "criteria")]
    pub criteria: Option<crate::datadogV2::model::AggregatedWaterfallPerformanceCriteria>,
    /// Start of the analyzed time range as a Unix timestamp in seconds.
    #[serde(rename = "from")]
    pub from: i64,
    /// Network resources in chronological waterfall order.
    #[serde(rename = "resources")]
    pub resources: Vec<crate::datadogV2::model::AggregatedResource>,
    /// List of RUM view IDs sampled for this aggregation, capped at 50.
    #[serde(rename = "sampled_view_ids")]
    pub sampled_view_ids: Vec<String>,
    /// End of the analyzed time range as a Unix timestamp in seconds.
    #[serde(rename = "to")]
    pub to: i64,
    /// Overall cache hit rate across all sampled views.
    #[serde(rename = "total_cache_hit_rate_pct")]
    pub total_cache_hit_rate_pct: f64,
    /// Number of view instances included in the analysis.
    #[serde(rename = "view_count")]
    pub view_count: i32,
    /// The RUM view name that was analyzed.
    #[serde(rename = "view_name")]
    pub view_name: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AggregatedWaterfallResponseAttributes {
    pub fn new(
        application_id: String,
        from: i64,
        resources: Vec<crate::datadogV2::model::AggregatedResource>,
        sampled_view_ids: Vec<String>,
        to: i64,
        total_cache_hit_rate_pct: f64,
        view_count: i32,
        view_name: String,
    ) -> AggregatedWaterfallResponseAttributes {
        AggregatedWaterfallResponseAttributes {
            application_id,
            criteria: None,
            from,
            resources,
            sampled_view_ids,
            to,
            total_cache_hit_rate_pct,
            view_count,
            view_name,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn criteria(
        mut self,
        value: crate::datadogV2::model::AggregatedWaterfallPerformanceCriteria,
    ) -> Self {
        self.criteria = Some(value);
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

impl<'de> Deserialize<'de> for AggregatedWaterfallResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AggregatedWaterfallResponseAttributesVisitor;
        impl<'a> Visitor<'a> for AggregatedWaterfallResponseAttributesVisitor {
            type Value = AggregatedWaterfallResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut application_id: Option<String> = None;
                let mut criteria: Option<
                    crate::datadogV2::model::AggregatedWaterfallPerformanceCriteria,
                > = None;
                let mut from: Option<i64> = None;
                let mut resources: Option<Vec<crate::datadogV2::model::AggregatedResource>> = None;
                let mut sampled_view_ids: Option<Vec<String>> = None;
                let mut to: Option<i64> = None;
                let mut total_cache_hit_rate_pct: Option<f64> = None;
                let mut view_count: Option<i32> = None;
                let mut view_name: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "application_id" => {
                            application_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "criteria" => {
                            if v.is_null() {
                                continue;
                            }
                            criteria = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "from" => {
                            from = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resources" => {
                            resources = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sampled_view_ids" => {
                            sampled_view_ids =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "to" => {
                            to = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_cache_hit_rate_pct" => {
                            total_cache_hit_rate_pct =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "view_count" => {
                            view_count = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "view_name" => {
                            view_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let application_id =
                    application_id.ok_or_else(|| M::Error::missing_field("application_id"))?;
                let from = from.ok_or_else(|| M::Error::missing_field("from"))?;
                let resources = resources.ok_or_else(|| M::Error::missing_field("resources"))?;
                let sampled_view_ids =
                    sampled_view_ids.ok_or_else(|| M::Error::missing_field("sampled_view_ids"))?;
                let to = to.ok_or_else(|| M::Error::missing_field("to"))?;
                let total_cache_hit_rate_pct = total_cache_hit_rate_pct
                    .ok_or_else(|| M::Error::missing_field("total_cache_hit_rate_pct"))?;
                let view_count = view_count.ok_or_else(|| M::Error::missing_field("view_count"))?;
                let view_name = view_name.ok_or_else(|| M::Error::missing_field("view_name"))?;

                let content = AggregatedWaterfallResponseAttributes {
                    application_id,
                    criteria,
                    from,
                    resources,
                    sampled_view_ids,
                    to,
                    total_cache_hit_rate_pct,
                    view_count,
                    view_name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AggregatedWaterfallResponseAttributesVisitor)
    }
}
