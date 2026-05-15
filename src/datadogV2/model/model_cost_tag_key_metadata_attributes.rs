// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a Cloud Cost Management tag key metadata entry.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CostTagKeyMetadataAttributes {
    /// Number of unique tag values observed for this tag key, keyed by cloud account ID.
    #[serde(rename = "cardinality_by_account")]
    pub cardinality_by_account: std::collections::BTreeMap<String, i64>,
    /// Total cost (in the report currency) of cost line items that carry this tag key for the requested period.
    #[serde(rename = "cost_covered")]
    pub cost_covered: f64,
    /// The day this row corresponds to, in `YYYY-MM-DD` format. Present only when `filter[daily]=true`; omitted for the monthly roll-up returned by default.
    #[serde(rename = "date")]
    pub date: Option<String>,
    /// The Cloud Cost Management metric this row aggregates, for example `aws.cost.net.amortized`.
    #[serde(rename = "metric")]
    pub metric: String,
    /// Number of cost rows that carry this tag key over the requested period.
    #[serde(rename = "row_count")]
    pub row_count: i64,
    /// Origins where this tag key was observed (for example, `aws-user-defined`).
    #[serde(rename = "tag_sources")]
    pub tag_sources: Vec<String>,
    /// A sample of the most frequent tag values observed for this tag key, keyed by cloud account ID.
    #[serde(rename = "top_values_by_account")]
    pub top_values_by_account: std::collections::BTreeMap<String, Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CostTagKeyMetadataAttributes {
    pub fn new(
        cardinality_by_account: std::collections::BTreeMap<String, i64>,
        cost_covered: f64,
        metric: String,
        row_count: i64,
        tag_sources: Vec<String>,
        top_values_by_account: std::collections::BTreeMap<String, Vec<String>>,
    ) -> CostTagKeyMetadataAttributes {
        CostTagKeyMetadataAttributes {
            cardinality_by_account,
            cost_covered,
            date: None,
            metric,
            row_count,
            tag_sources,
            top_values_by_account,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn date(mut self, value: String) -> Self {
        self.date = Some(value);
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

impl<'de> Deserialize<'de> for CostTagKeyMetadataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CostTagKeyMetadataAttributesVisitor;
        impl<'a> Visitor<'a> for CostTagKeyMetadataAttributesVisitor {
            type Value = CostTagKeyMetadataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut cardinality_by_account: Option<std::collections::BTreeMap<String, i64>> =
                    None;
                let mut cost_covered: Option<f64> = None;
                let mut date: Option<String> = None;
                let mut metric: Option<String> = None;
                let mut row_count: Option<i64> = None;
                let mut tag_sources: Option<Vec<String>> = None;
                let mut top_values_by_account: Option<
                    std::collections::BTreeMap<String, Vec<String>>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "cardinality_by_account" => {
                            cardinality_by_account =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cost_covered" => {
                            cost_covered =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "date" => {
                            if v.is_null() {
                                continue;
                            }
                            date = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metric" => {
                            metric = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "row_count" => {
                            row_count = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tag_sources" => {
                            tag_sources =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "top_values_by_account" => {
                            top_values_by_account =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let cardinality_by_account = cardinality_by_account
                    .ok_or_else(|| M::Error::missing_field("cardinality_by_account"))?;
                let cost_covered =
                    cost_covered.ok_or_else(|| M::Error::missing_field("cost_covered"))?;
                let metric = metric.ok_or_else(|| M::Error::missing_field("metric"))?;
                let row_count = row_count.ok_or_else(|| M::Error::missing_field("row_count"))?;
                let tag_sources =
                    tag_sources.ok_or_else(|| M::Error::missing_field("tag_sources"))?;
                let top_values_by_account = top_values_by_account
                    .ok_or_else(|| M::Error::missing_field("top_values_by_account"))?;

                let content = CostTagKeyMetadataAttributes {
                    cardinality_by_account,
                    cost_covered,
                    date,
                    metric,
                    row_count,
                    tag_sources,
                    top_values_by_account,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CostTagKeyMetadataAttributesVisitor)
    }
}
