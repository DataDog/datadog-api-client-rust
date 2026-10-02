// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Datadog source and query that supply values for the metric.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsMetricV2DTODataAttributesPercentileAggregationDatadogMetricMeasure {
    /// Name of the Datadog event field used by this measure.
    #[serde(rename = "column_name")]
    pub column_name: Option<String>,
    /// Data type of the source column.
    #[serde(rename = "column_type")]
    pub column_type: Option<String>,
    /// Conditions used to select the metric's source data.
    #[serde(rename = "filters")]
    pub filters: Option<serde_json::Value>,
    /// Display name of the Datadog measure.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Query used to retrieve the Datadog measure.
    #[serde(rename = "query")]
    pub query: Option<String>,
    /// Filter applied to the Datadog source definition.
    #[serde(rename = "source_definition_filter")]
    pub source_definition_filter: Option<serde_json::Value>,
    /// Subtype of the Datadog data source.
    #[serde(rename = "source_subtype")]
    pub source_subtype: Option<String>,
    /// Type of Datadog data source used for the measure.
    #[serde(rename = "source_type")]
    pub source_type: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsMetricV2DTODataAttributesPercentileAggregationDatadogMetricMeasure {
    pub fn new() -> ExperimentsMetricV2DTODataAttributesPercentileAggregationDatadogMetricMeasure {
        ExperimentsMetricV2DTODataAttributesPercentileAggregationDatadogMetricMeasure {
            column_name: None,
            column_type: None,
            filters: None,
            name: None,
            query: None,
            source_definition_filter: None,
            source_subtype: None,
            source_type: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn column_name(mut self, value: String) -> Self {
        self.column_name = Some(value);
        self
    }

    pub fn column_type(mut self, value: String) -> Self {
        self.column_type = Some(value);
        self
    }

    pub fn filters(mut self, value: serde_json::Value) -> Self {
        self.filters = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn query(mut self, value: String) -> Self {
        self.query = Some(value);
        self
    }

    pub fn source_definition_filter(mut self, value: serde_json::Value) -> Self {
        self.source_definition_filter = Some(value);
        self
    }

    pub fn source_subtype(mut self, value: String) -> Self {
        self.source_subtype = Some(value);
        self
    }

    pub fn source_type(mut self, value: String) -> Self {
        self.source_type = Some(value);
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

impl Default for ExperimentsMetricV2DTODataAttributesPercentileAggregationDatadogMetricMeasure {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de>
    for ExperimentsMetricV2DTODataAttributesPercentileAggregationDatadogMetricMeasure
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsMetricV2DTODataAttributesPercentileAggregationDatadogMetricMeasureVisitor;
        impl<'a> Visitor<'a>
            for ExperimentsMetricV2DTODataAttributesPercentileAggregationDatadogMetricMeasureVisitor
        {
            type Value =
                ExperimentsMetricV2DTODataAttributesPercentileAggregationDatadogMetricMeasure;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut column_name: Option<String> = None;
                let mut column_type: Option<String> = None;
                let mut filters: Option<serde_json::Value> = None;
                let mut name: Option<String> = None;
                let mut query: Option<String> = None;
                let mut source_definition_filter: Option<serde_json::Value> = None;
                let mut source_subtype: Option<String> = None;
                let mut source_type: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "column_name" => {
                            if v.is_null() {
                                continue;
                            }
                            column_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "column_type" => {
                            if v.is_null() {
                                continue;
                            }
                            column_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "filters" => {
                            if v.is_null() {
                                continue;
                            }
                            filters = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            if v.is_null() {
                                continue;
                            }
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source_definition_filter" => {
                            if v.is_null() {
                                continue;
                            }
                            source_definition_filter =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source_subtype" => {
                            if v.is_null() {
                                continue;
                            }
                            source_subtype =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source_type" => {
                            if v.is_null() {
                                continue;
                            }
                            source_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content =
                    ExperimentsMetricV2DTODataAttributesPercentileAggregationDatadogMetricMeasure {
                        column_name,
                        column_type,
                        filters,
                        name,
                        query,
                        source_definition_filter,
                        source_subtype,
                        source_type,
                        additional_properties,
                        _unparsed,
                    };

                Ok(content)
            }
        }

        deserializer.deserialize_any(
            ExperimentsMetricV2DTODataAttributesPercentileAggregationDatadogMetricMeasureVisitor,
        )
    }
}
