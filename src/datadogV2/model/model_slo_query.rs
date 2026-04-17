// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A query for SLO status, error budget, and burn rate metrics.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SloQuery {
    /// Additional filters applied to the SLO query.
    #[serde(rename = "additional_query_filters")]
    pub additional_query_filters: Option<String>,
    /// A data source for SLO queries.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV2::model::SloDataSource,
    /// How SLO results are grouped in the response.
    #[serde(rename = "group_mode")]
    pub group_mode: Option<crate::datadogV2::model::SlosGroupMode>,
    /// The SLO measurement to retrieve.
    #[serde(rename = "measure")]
    pub measure: crate::datadogV2::model::SlosMeasure,
    /// The variable name for use in formulas.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The unique identifier of the SLO to query.
    #[serde(rename = "slo_id")]
    pub slo_id: String,
    /// The type of SLO definition being queried.
    #[serde(rename = "slo_query_type")]
    pub slo_query_type: Option<crate::datadogV2::model::SlosQueryType>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SloQuery {
    pub fn new(
        data_source: crate::datadogV2::model::SloDataSource,
        measure: crate::datadogV2::model::SlosMeasure,
        slo_id: String,
    ) -> SloQuery {
        SloQuery {
            additional_query_filters: None,
            data_source,
            group_mode: None,
            measure,
            name: None,
            slo_id,
            slo_query_type: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_query_filters(mut self, value: String) -> Self {
        self.additional_query_filters = Some(value);
        self
    }

    pub fn group_mode(mut self, value: crate::datadogV2::model::SlosGroupMode) -> Self {
        self.group_mode = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn slo_query_type(mut self, value: crate::datadogV2::model::SlosQueryType) -> Self {
        self.slo_query_type = Some(value);
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

impl<'de> Deserialize<'de> for SloQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SloQueryVisitor;
        impl<'a> Visitor<'a> for SloQueryVisitor {
            type Value = SloQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut additional_query_filters: Option<String> = None;
                let mut data_source: Option<crate::datadogV2::model::SloDataSource> = None;
                let mut group_mode: Option<crate::datadogV2::model::SlosGroupMode> = None;
                let mut measure: Option<crate::datadogV2::model::SlosMeasure> = None;
                let mut name: Option<String> = None;
                let mut slo_id: Option<String> = None;
                let mut slo_query_type: Option<crate::datadogV2::model::SlosQueryType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "additional_query_filters" => {
                            if v.is_null() {
                                continue;
                            }
                            additional_query_filters =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "data_source" => {
                            data_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_source) = data_source {
                                match _data_source {
                                    crate::datadogV2::model::SloDataSource::UnparsedObject(
                                        _data_source,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "group_mode" => {
                            if v.is_null() {
                                continue;
                            }
                            group_mode = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _group_mode) = group_mode {
                                match _group_mode {
                                    crate::datadogV2::model::SlosGroupMode::UnparsedObject(
                                        _group_mode,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "measure" => {
                            measure = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _measure) = measure {
                                match _measure {
                                    crate::datadogV2::model::SlosMeasure::UnparsedObject(
                                        _measure,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "slo_id" => {
                            slo_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "slo_query_type" => {
                            if v.is_null() {
                                continue;
                            }
                            slo_query_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _slo_query_type) = slo_query_type {
                                match _slo_query_type {
                                    crate::datadogV2::model::SlosQueryType::UnparsedObject(
                                        _slo_query_type,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let data_source =
                    data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;
                let measure = measure.ok_or_else(|| M::Error::missing_field("measure"))?;
                let slo_id = slo_id.ok_or_else(|| M::Error::missing_field("slo_id"))?;

                let content = SloQuery {
                    additional_query_filters,
                    data_source,
                    group_mode,
                    measure,
                    name,
                    slo_id,
                    slo_query_type,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SloQueryVisitor)
    }
}
