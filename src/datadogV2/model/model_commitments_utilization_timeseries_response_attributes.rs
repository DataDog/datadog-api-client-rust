// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a timeseries utilization response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CommitmentsUtilizationTimeseriesResponseAttributes {
    /// Metadata reporting that the cost data lacks the unused costs of some commitments, as with Azure exports
    /// scoped below the billing profile. Utilization is then overstated, and realized savings and effective
    /// savings rate are upper bounds. Omitted when the unused costs are available.
    #[serde(rename = "meta")]
    pub meta: Option<crate::datadogV2::model::CommitmentsUnusedCostMeta>,
    /// The query that produces the response values, in the same format as dashboard and notebook widget
    /// requests, so a client can graph the values in a widget. Each formula's `id` names the response value
    /// that it produces.
    #[serde(rename = "query_definition")]
    pub query_definition: Option<crate::datadogV2::model::CommitmentsQueryDefinition>,
    /// Timeseries data as a map of series names to their corresponding value arrays.
    #[serde(rename = "series")]
    pub series: std::collections::BTreeMap<String, Vec<f64>>,
    /// Unix timestamps in seconds for the timeseries data points.
    #[serde(rename = "times")]
    pub times: Vec<i64>,
    /// Unit metadata for a numeric metric. Unset fields are omitted, and the unit is `null` when it is unknown.
    #[serde(rename = "unit", default, with = "::serde_with::rust::double_option")]
    pub unit: Option<Option<crate::datadogV2::model::CommitmentsUnit>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CommitmentsUtilizationTimeseriesResponseAttributes {
    pub fn new(
        series: std::collections::BTreeMap<String, Vec<f64>>,
        times: Vec<i64>,
    ) -> CommitmentsUtilizationTimeseriesResponseAttributes {
        CommitmentsUtilizationTimeseriesResponseAttributes {
            meta: None,
            query_definition: None,
            series,
            times,
            unit: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn meta(mut self, value: crate::datadogV2::model::CommitmentsUnusedCostMeta) -> Self {
        self.meta = Some(value);
        self
    }

    pub fn query_definition(
        mut self,
        value: crate::datadogV2::model::CommitmentsQueryDefinition,
    ) -> Self {
        self.query_definition = Some(value);
        self
    }

    pub fn unit(mut self, value: Option<crate::datadogV2::model::CommitmentsUnit>) -> Self {
        self.unit = Some(value);
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

impl<'de> Deserialize<'de> for CommitmentsUtilizationTimeseriesResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CommitmentsUtilizationTimeseriesResponseAttributesVisitor;
        impl<'a> Visitor<'a> for CommitmentsUtilizationTimeseriesResponseAttributesVisitor {
            type Value = CommitmentsUtilizationTimeseriesResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut meta: Option<crate::datadogV2::model::CommitmentsUnusedCostMeta> = None;
                let mut query_definition: Option<
                    crate::datadogV2::model::CommitmentsQueryDefinition,
                > = None;
                let mut series: Option<std::collections::BTreeMap<String, Vec<f64>>> = None;
                let mut times: Option<Vec<i64>> = None;
                let mut unit: Option<Option<crate::datadogV2::model::CommitmentsUnit>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "meta" => {
                            if v.is_null() {
                                continue;
                            }
                            meta = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query_definition" => {
                            if v.is_null() {
                                continue;
                            }
                            query_definition =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "series" => {
                            series = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "times" => {
                            times = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "unit" => {
                            unit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let series = series.ok_or_else(|| M::Error::missing_field("series"))?;
                let times = times.ok_or_else(|| M::Error::missing_field("times"))?;

                let content = CommitmentsUtilizationTimeseriesResponseAttributes {
                    meta,
                    query_definition,
                    series,
                    times,
                    unit,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CommitmentsUtilizationTimeseriesResponseAttributesVisitor)
    }
}
