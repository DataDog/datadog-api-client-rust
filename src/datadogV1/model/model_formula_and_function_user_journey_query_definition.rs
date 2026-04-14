// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A formula and functions User Journey query for defining funnel, timeseries, and scalar visualizations over journey data.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FormulaAndFunctionUserJourneyQueryDefinition {
    /// Compute configuration for User Journey formula queries.
    #[serde(rename = "compute")]
    pub compute: crate::datadogV1::model::UserJourneyFormulaCompute,
    /// Data source for user journey funnel queries.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1::model::ProductAnalyticsFunnelDataSource,
    /// Group by configuration.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<crate::datadogV1::model::UserJourneyFormulaGroupBy>>,
    /// Name of the query for use in formulas.
    #[serde(rename = "name")]
    pub name: String,
    /// User journey search configuration.
    #[serde(rename = "search")]
    pub search: crate::datadogV1::model::UserJourneySearch,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FormulaAndFunctionUserJourneyQueryDefinition {
    pub fn new(
        compute: crate::datadogV1::model::UserJourneyFormulaCompute,
        data_source: crate::datadogV1::model::ProductAnalyticsFunnelDataSource,
        name: String,
        search: crate::datadogV1::model::UserJourneySearch,
    ) -> FormulaAndFunctionUserJourneyQueryDefinition {
        FormulaAndFunctionUserJourneyQueryDefinition {
            compute,
            data_source,
            group_by: None,
            name,
            search,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn group_by(
        mut self,
        value: Vec<crate::datadogV1::model::UserJourneyFormulaGroupBy>,
    ) -> Self {
        self.group_by = Some(value);
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

impl<'de> Deserialize<'de> for FormulaAndFunctionUserJourneyQueryDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FormulaAndFunctionUserJourneyQueryDefinitionVisitor;
        impl<'a> Visitor<'a> for FormulaAndFunctionUserJourneyQueryDefinitionVisitor {
            type Value = FormulaAndFunctionUserJourneyQueryDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut compute: Option<crate::datadogV1::model::UserJourneyFormulaCompute> = None;
                let mut data_source: Option<
                    crate::datadogV1::model::ProductAnalyticsFunnelDataSource,
                > = None;
                let mut group_by: Option<Vec<crate::datadogV1::model::UserJourneyFormulaGroupBy>> =
                    None;
                let mut name: Option<String> = None;
                let mut search: Option<crate::datadogV1::model::UserJourneySearch> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "compute" => {
                            compute = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "data_source" => {
                            data_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_source) = data_source {
                                match _data_source {
                                    crate::datadogV1::model::ProductAnalyticsFunnelDataSource::UnparsedObject(_data_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "group_by" => {
                            if v.is_null() {
                                continue;
                            }
                            group_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "search" => {
                            search = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let compute = compute.ok_or_else(|| M::Error::missing_field("compute"))?;
                let data_source =
                    data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let search = search.ok_or_else(|| M::Error::missing_field("search"))?;

                let content = FormulaAndFunctionUserJourneyQueryDefinition {
                    compute,
                    data_source,
                    group_by,
                    name,
                    search,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FormulaAndFunctionUserJourneyQueryDefinitionVisitor)
    }
}
