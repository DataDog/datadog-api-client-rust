// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a timeseries coverage response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CommitmentsCoverageTimeseriesResponseAttributes {
    /// A coverage timeseries metric. For AWS and Azure, `series` holds the `on_demand_only`, `spot_only`, `ri`,
    /// and `sp` buckets, plus `ppp` for Azure pre-purchase plans when enabled. For GCP, `series` holds the
    /// `on_demand_only` and `flex_cud` buckets.
    #[serde(rename = "cost")]
    pub cost: crate::datadogV2::model::CommitmentsCoverageTimeseriesMetric,
    /// A coverage timeseries metric. For AWS and Azure, `series` holds the `on_demand_only`, `spot_only`, `ri`,
    /// and `sp` buckets, plus `ppp` for Azure pre-purchase plans when enabled. For GCP, `series` holds the
    /// `on_demand_only` and `flex_cud` buckets.
    #[serde(rename = "hours")]
    pub hours: crate::datadogV2::model::CommitmentsCoverageTimeseriesMetric,
    /// The query that produces the response values, in the same format as dashboard and notebook widget
    /// requests, so a client can graph the values in a widget. Each formula's `id` names the response value
    /// that it produces.
    #[serde(rename = "query_definition")]
    pub query_definition: Option<crate::datadogV2::model::CommitmentsQueryDefinition>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CommitmentsCoverageTimeseriesResponseAttributes {
    pub fn new(
        cost: crate::datadogV2::model::CommitmentsCoverageTimeseriesMetric,
        hours: crate::datadogV2::model::CommitmentsCoverageTimeseriesMetric,
    ) -> CommitmentsCoverageTimeseriesResponseAttributes {
        CommitmentsCoverageTimeseriesResponseAttributes {
            cost,
            hours,
            query_definition: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn query_definition(
        mut self,
        value: crate::datadogV2::model::CommitmentsQueryDefinition,
    ) -> Self {
        self.query_definition = Some(value);
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

impl<'de> Deserialize<'de> for CommitmentsCoverageTimeseriesResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CommitmentsCoverageTimeseriesResponseAttributesVisitor;
        impl<'a> Visitor<'a> for CommitmentsCoverageTimeseriesResponseAttributesVisitor {
            type Value = CommitmentsCoverageTimeseriesResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut cost: Option<crate::datadogV2::model::CommitmentsCoverageTimeseriesMetric> =
                    None;
                let mut hours: Option<
                    crate::datadogV2::model::CommitmentsCoverageTimeseriesMetric,
                > = None;
                let mut query_definition: Option<
                    crate::datadogV2::model::CommitmentsQueryDefinition,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "cost" => {
                            cost = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "hours" => {
                            hours = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query_definition" => {
                            if v.is_null() {
                                continue;
                            }
                            query_definition =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let cost = cost.ok_or_else(|| M::Error::missing_field("cost"))?;
                let hours = hours.ok_or_else(|| M::Error::missing_field("hours"))?;

                let content = CommitmentsCoverageTimeseriesResponseAttributes {
                    cost,
                    hours,
                    query_definition,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CommitmentsCoverageTimeseriesResponseAttributesVisitor)
    }
}
