// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FormulaSloQuery {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "additional_query_filters")]
    pub additional_query_filters: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "apply_corrections")]
    pub apply_corrections: Option<bool>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1_20270101::model::FormulaSloQueryDataSource,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "group_mode")]
    pub group_mode: crate::datadogV1_20270101::model::SloGroupMode,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "measure")]
    pub measure: crate::datadogV1_20270101::model::SloMeasure,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "name")]
    pub name: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "slo_id")]
    pub slo_id: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "slo_query_type")]
    pub slo_query_type: crate::datadogV1_20270101::model::SloQueryType,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "slo_time_window")]
    pub slo_time_window: Option<crate::datadogV1_20270101::model::SloTimeWindow>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl FormulaSloQuery {
    pub fn new(data_source: crate::datadogV1_20270101::model::FormulaSloQueryDataSource, group_mode: crate::datadogV1_20270101::model::SloGroupMode, measure: crate::datadogV1_20270101::model::SloMeasure, name: String, slo_id: String, slo_query_type: crate::datadogV1_20270101::model::SloQueryType) -> FormulaSloQuery {
        FormulaSloQuery {
            additional_query_filters: None,
            apply_corrections: None,
            data_source,
            group_mode,
            measure,
            name,
            slo_id,
            slo_query_type,
            slo_time_window: None,
            _unparsed: false,
        }
    }
    
    pub fn additional_query_filters(mut self, value: String) -> Self {
        self.additional_query_filters = Some(value);
        self
    }
    
    pub fn apply_corrections(mut self, value: bool) -> Self {
        self.apply_corrections = Some(value);
        self
    }
    
    pub fn slo_time_window(mut self, value: crate::datadogV1_20270101::model::SloTimeWindow) -> Self {
        self.slo_time_window = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for FormulaSloQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FormulaSloQueryVisitor;
        impl<'a> Visitor<'a> for FormulaSloQueryVisitor {
            type Value = FormulaSloQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut additional_query_filters: Option<String> = None;
                let mut apply_corrections: Option<bool> = None;
                let mut data_source: Option<crate::datadogV1_20270101::model::FormulaSloQueryDataSource> = None;
                let mut group_mode: Option<crate::datadogV1_20270101::model::SloGroupMode> = None;
                let mut measure: Option<crate::datadogV1_20270101::model::SloMeasure> = None;
                let mut name: Option<String> = None;
                let mut slo_id: Option<String> = None;
                let mut slo_query_type: Option<crate::datadogV1_20270101::model::SloQueryType> = None;
                let mut slo_time_window: Option<crate::datadogV1_20270101::model::SloTimeWindow> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "additional_query_filters" => {
                            if v.is_null() {
                                continue;
                            }
                            additional_query_filters = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "apply_corrections" => {
                            if v.is_null() {
                                continue;
                            }
                            apply_corrections = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "data_source" => {
                            data_source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_source) = data_source {
                                match _data_source {
                                    crate::datadogV1_20270101::model::FormulaSloQueryDataSource::UnparsedObject(_data_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "group_mode" => {
                            group_mode = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _group_mode) = group_mode {
                                match _group_mode {
                                    crate::datadogV1_20270101::model::SloGroupMode::UnparsedObject(_group_mode) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "measure" => {
                            measure = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _measure) = measure {
                                match _measure {
                                    crate::datadogV1_20270101::model::SloMeasure::UnparsedObject(_measure) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "slo_id" => {
                            slo_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "slo_query_type" => {
                            slo_query_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _slo_query_type) = slo_query_type {
                                match _slo_query_type {
                                    crate::datadogV1_20270101::model::SloQueryType::UnparsedObject(_slo_query_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "slo_time_window" => {
                            if v.is_null() {
                                continue;
                            }
                            slo_time_window = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _slo_time_window) = slo_time_window {
                                match _slo_time_window {
                                    crate::datadogV1_20270101::model::SloTimeWindow::UnparsedObject(_slo_time_window) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let data_source = data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;
                let group_mode = group_mode.ok_or_else(|| M::Error::missing_field("group_mode"))?;
                let measure = measure.ok_or_else(|| M::Error::missing_field("measure"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let slo_id = slo_id.ok_or_else(|| M::Error::missing_field("slo_id"))?;
                let slo_query_type = slo_query_type.ok_or_else(|| M::Error::missing_field("slo_query_type"))?;

                let content = FormulaSloQuery {
                    additional_query_filters,
                    apply_corrections,
                    data_source,
                    group_mode,
                    measure,
                    name,
                    slo_id,
                    slo_query_type,
                    slo_time_window,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FormulaSloQueryVisitor)
    }
}
