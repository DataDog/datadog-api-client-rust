// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Available values for a specific facet key.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ModelLabFacetValuesAttributes {
    /// The name of the facet.
    #[serde(rename = "facet_name")]
    pub facet_name: String,
    /// The type of the facet.
    #[serde(rename = "facet_type")]
    pub facet_type: String,
    /// The ranges for each metric statistic.
    #[serde(rename = "metric_stat_ranges")]
    pub metric_stat_ranges: Option<Vec<crate::datadogV2::model::ModelLabMetricStatRange>>,
    /// The numeric range of values for a facet.
    #[serde(rename = "numeric_range")]
    pub numeric_range: Option<crate::datadogV2::model::ModelLabNumericRange>,
    /// The list of available string values for this facet.
    #[serde(rename = "values")]
    pub values: Vec<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ModelLabFacetValuesAttributes {
    pub fn new(
        facet_name: String,
        facet_type: String,
        values: Vec<String>,
    ) -> ModelLabFacetValuesAttributes {
        ModelLabFacetValuesAttributes {
            facet_name,
            facet_type,
            metric_stat_ranges: None,
            numeric_range: None,
            values,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn metric_stat_ranges(
        mut self,
        value: Vec<crate::datadogV2::model::ModelLabMetricStatRange>,
    ) -> Self {
        self.metric_stat_ranges = Some(value);
        self
    }

    pub fn numeric_range(mut self, value: crate::datadogV2::model::ModelLabNumericRange) -> Self {
        self.numeric_range = Some(value);
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

impl<'de> Deserialize<'de> for ModelLabFacetValuesAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ModelLabFacetValuesAttributesVisitor;
        impl<'a> Visitor<'a> for ModelLabFacetValuesAttributesVisitor {
            type Value = ModelLabFacetValuesAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut facet_name: Option<String> = None;
                let mut facet_type: Option<String> = None;
                let mut metric_stat_ranges: Option<
                    Vec<crate::datadogV2::model::ModelLabMetricStatRange>,
                > = None;
                let mut numeric_range: Option<crate::datadogV2::model::ModelLabNumericRange> = None;
                let mut values: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "facet_name" => {
                            facet_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "facet_type" => {
                            facet_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metric_stat_ranges" => {
                            if v.is_null() {
                                continue;
                            }
                            metric_stat_ranges =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "numeric_range" => {
                            if v.is_null() {
                                continue;
                            }
                            numeric_range =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "values" => {
                            values = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let facet_name = facet_name.ok_or_else(|| M::Error::missing_field("facet_name"))?;
                let facet_type = facet_type.ok_or_else(|| M::Error::missing_field("facet_type"))?;
                let values = values.ok_or_else(|| M::Error::missing_field("values"))?;

                let content = ModelLabFacetValuesAttributes {
                    facet_name,
                    facet_type,
                    metric_stat_ranges,
                    numeric_range,
                    values,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ModelLabFacetValuesAttributesVisitor)
    }
}
