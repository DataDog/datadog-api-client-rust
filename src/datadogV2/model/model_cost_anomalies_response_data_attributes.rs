// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Cost anomaly results and aggregated totals for the queried window.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CostAnomaliesResponseDataAttributes {
    /// The list of cost anomalies that match the request.
    #[serde(rename = "anomalies")]
    pub anomalies: Vec<crate::datadogV2::model::CostAnomaly>,
    /// Average daily anomalous cost change across the queried window.
    #[serde(rename = "avg_daily_anomalous_cost")]
    pub avg_daily_anomalous_cost: f64,
    /// Total actual cost spent across the queried window for the matching providers.
    #[serde(rename = "total_actual_cost")]
    pub total_actual_cost: f64,
    /// Sum of the anomalous cost change across all returned anomalies.
    #[serde(rename = "total_anomalous_cost")]
    pub total_anomalous_cost: f64,
    /// Total number of anomalies that match the request.
    #[serde(rename = "total_count")]
    pub total_count: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CostAnomaliesResponseDataAttributes {
    pub fn new(
        anomalies: Vec<crate::datadogV2::model::CostAnomaly>,
        avg_daily_anomalous_cost: f64,
        total_actual_cost: f64,
        total_anomalous_cost: f64,
        total_count: i64,
    ) -> CostAnomaliesResponseDataAttributes {
        CostAnomaliesResponseDataAttributes {
            anomalies,
            avg_daily_anomalous_cost,
            total_actual_cost,
            total_anomalous_cost,
            total_count,
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

impl<'de> Deserialize<'de> for CostAnomaliesResponseDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CostAnomaliesResponseDataAttributesVisitor;
        impl<'a> Visitor<'a> for CostAnomaliesResponseDataAttributesVisitor {
            type Value = CostAnomaliesResponseDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut anomalies: Option<Vec<crate::datadogV2::model::CostAnomaly>> = None;
                let mut avg_daily_anomalous_cost: Option<f64> = None;
                let mut total_actual_cost: Option<f64> = None;
                let mut total_anomalous_cost: Option<f64> = None;
                let mut total_count: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "anomalies" => {
                            anomalies = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "avg_daily_anomalous_cost" => {
                            avg_daily_anomalous_cost =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_actual_cost" => {
                            total_actual_cost =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_anomalous_cost" => {
                            total_anomalous_cost =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_count" => {
                            total_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let anomalies = anomalies.ok_or_else(|| M::Error::missing_field("anomalies"))?;
                let avg_daily_anomalous_cost = avg_daily_anomalous_cost
                    .ok_or_else(|| M::Error::missing_field("avg_daily_anomalous_cost"))?;
                let total_actual_cost = total_actual_cost
                    .ok_or_else(|| M::Error::missing_field("total_actual_cost"))?;
                let total_anomalous_cost = total_anomalous_cost
                    .ok_or_else(|| M::Error::missing_field("total_anomalous_cost"))?;
                let total_count =
                    total_count.ok_or_else(|| M::Error::missing_field("total_count"))?;

                let content = CostAnomaliesResponseDataAttributes {
                    anomalies,
                    avg_daily_anomalous_cost,
                    total_actual_cost,
                    total_anomalous_cost,
                    total_count,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CostAnomaliesResponseDataAttributesVisitor)
    }
}
