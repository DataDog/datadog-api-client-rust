// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A single detected Cloud Cost Management anomaly.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CostAnomaly {
    /// Actual cost incurred during the anomaly window.
    #[serde(rename = "actual_cost")]
    pub actual_cost: f64,
    /// Anomalous cost change relative to the expected baseline.
    #[serde(rename = "anomalous_cost_change")]
    pub anomalous_cost_change: f64,
    /// Anomaly end timestamp in Unix milliseconds.
    #[serde(rename = "anomaly_end")]
    pub anomaly_end: i64,
    /// Anomaly start timestamp in Unix milliseconds.
    #[serde(rename = "anomaly_start")]
    pub anomaly_start: i64,
    /// Map of correlated tag keys to the list of correlated tag values.
    #[serialize_always]
    #[serde(rename = "correlated_tags")]
    pub correlated_tags: Option<std::collections::BTreeMap<String, Vec<String>>>,
    /// Map of cost dimension keys to their values for the anomaly grouping.
    #[serde(rename = "dimensions")]
    pub dimensions: std::collections::BTreeMap<String, String>,
    /// Resolution metadata for an anomaly that has been dismissed.
    #[serde(rename = "dismissal")]
    pub dismissal: Option<crate::datadogV2::model::CostAnomalyDismissal>,
    /// Maximum cost observed during the anomaly window.
    #[serde(rename = "max_cost")]
    pub max_cost: f64,
    /// Cloud or SaaS provider associated with the anomaly (for example `aws`, `gcp`, `azure`).
    #[serde(rename = "provider")]
    pub provider: String,
    /// The metrics query that detected the anomaly.
    #[serde(rename = "query")]
    pub query: String,
    /// The unique identifier of the anomaly.
    #[serde(rename = "uuid")]
    pub uuid: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CostAnomaly {
    pub fn new(
        actual_cost: f64,
        anomalous_cost_change: f64,
        anomaly_end: i64,
        anomaly_start: i64,
        correlated_tags: Option<std::collections::BTreeMap<String, Vec<String>>>,
        dimensions: std::collections::BTreeMap<String, String>,
        max_cost: f64,
        provider: String,
        query: String,
        uuid: String,
    ) -> CostAnomaly {
        CostAnomaly {
            actual_cost,
            anomalous_cost_change,
            anomaly_end,
            anomaly_start,
            correlated_tags,
            dimensions,
            dismissal: None,
            max_cost,
            provider,
            query,
            uuid,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn dismissal(mut self, value: crate::datadogV2::model::CostAnomalyDismissal) -> Self {
        self.dismissal = Some(value);
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

impl<'de> Deserialize<'de> for CostAnomaly {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CostAnomalyVisitor;
        impl<'a> Visitor<'a> for CostAnomalyVisitor {
            type Value = CostAnomaly;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut actual_cost: Option<f64> = None;
                let mut anomalous_cost_change: Option<f64> = None;
                let mut anomaly_end: Option<i64> = None;
                let mut anomaly_start: Option<i64> = None;
                let mut correlated_tags: Option<
                    Option<std::collections::BTreeMap<String, Vec<String>>>,
                > = None;
                let mut dimensions: Option<std::collections::BTreeMap<String, String>> = None;
                let mut dismissal: Option<crate::datadogV2::model::CostAnomalyDismissal> = None;
                let mut max_cost: Option<f64> = None;
                let mut provider: Option<String> = None;
                let mut query: Option<String> = None;
                let mut uuid: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "actual_cost" => {
                            actual_cost =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "anomalous_cost_change" => {
                            anomalous_cost_change =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "anomaly_end" => {
                            anomaly_end =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "anomaly_start" => {
                            anomaly_start =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "correlated_tags" => {
                            correlated_tags =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dimensions" => {
                            dimensions = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dismissal" => {
                            if v.is_null() {
                                continue;
                            }
                            dismissal = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "max_cost" => {
                            max_cost = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "provider" => {
                            provider = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "uuid" => {
                            uuid = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let actual_cost =
                    actual_cost.ok_or_else(|| M::Error::missing_field("actual_cost"))?;
                let anomalous_cost_change = anomalous_cost_change
                    .ok_or_else(|| M::Error::missing_field("anomalous_cost_change"))?;
                let anomaly_end =
                    anomaly_end.ok_or_else(|| M::Error::missing_field("anomaly_end"))?;
                let anomaly_start =
                    anomaly_start.ok_or_else(|| M::Error::missing_field("anomaly_start"))?;
                let correlated_tags =
                    correlated_tags.ok_or_else(|| M::Error::missing_field("correlated_tags"))?;
                let dimensions = dimensions.ok_or_else(|| M::Error::missing_field("dimensions"))?;
                let max_cost = max_cost.ok_or_else(|| M::Error::missing_field("max_cost"))?;
                let provider = provider.ok_or_else(|| M::Error::missing_field("provider"))?;
                let query = query.ok_or_else(|| M::Error::missing_field("query"))?;
                let uuid = uuid.ok_or_else(|| M::Error::missing_field("uuid"))?;

                let content = CostAnomaly {
                    actual_cost,
                    anomalous_cost_change,
                    anomaly_end,
                    anomaly_start,
                    correlated_tags,
                    dimensions,
                    dismissal,
                    max_cost,
                    provider,
                    query,
                    uuid,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CostAnomalyVisitor)
    }
}
