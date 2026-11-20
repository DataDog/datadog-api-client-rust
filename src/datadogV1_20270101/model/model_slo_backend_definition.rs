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
pub struct SloBackendDefinition {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "additional_query_filters")]
    pub additional_query_filters: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "global_time_target")]
    pub global_time_target: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "notes")]
    pub notes: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "show_error_budget")]
    pub show_error_budget: bool,
    /// Test-only generated schema for the dashboard versioning proof of concept.
	#[serialize_always]
    #[serde(rename = "slo_id")]
    pub slo_id: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "time")]
    pub time: Option<crate::datadogV1_20270101::model::WidgetTime>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "time_windows")]
    pub time_windows: Vec<crate::datadogV1_20270101::model::SloWidgetTimeWindow>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "title_align")]
    pub title_align: Option<crate::datadogV1_20270101::model::WidgetTextAlign>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "title_size")]
    pub title_size: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1_20270101::model::SloBackendDefinitionType,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "view_mode")]
    pub view_mode: crate::datadogV1_20270101::model::SloWidgetViewMode,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "view_type")]
    pub view_type: crate::datadogV1_20270101::model::SloBackendDefinitionViewType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl SloBackendDefinition {
    pub fn new(show_error_budget: bool, slo_id: Option<String>, time_windows: Vec<crate::datadogV1_20270101::model::SloWidgetTimeWindow>, type_: crate::datadogV1_20270101::model::SloBackendDefinitionType, view_mode: crate::datadogV1_20270101::model::SloWidgetViewMode, view_type: crate::datadogV1_20270101::model::SloBackendDefinitionViewType) -> SloBackendDefinition {
        SloBackendDefinition {
            additional_query_filters: None,
            description: None,
            global_time_target: None,
            notes: None,
            show_error_budget,
            slo_id,
            time: None,
            time_windows,
            title: None,
            title_align: None,
            title_size: None,
            type_,
            view_mode,
            view_type,
            _unparsed: false,
        }
    }
    
    pub fn additional_query_filters(mut self, value: String) -> Self {
        self.additional_query_filters = Some(value);
        self
    }
    
    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }
    
    pub fn global_time_target(mut self, value: String) -> Self {
        self.global_time_target = Some(value);
        self
    }
    
    pub fn notes(mut self, value: String) -> Self {
        self.notes = Some(value);
        self
    }
    
    pub fn time(mut self, value: crate::datadogV1_20270101::model::WidgetTime) -> Self {
        self.time = Some(value);
        self
    }
    
    pub fn title(mut self, value: String) -> Self {
        self.title = Some(value);
        self
    }
    
    pub fn title_align(mut self, value: crate::datadogV1_20270101::model::WidgetTextAlign) -> Self {
        self.title_align = Some(value);
        self
    }
    
    pub fn title_size(mut self, value: String) -> Self {
        self.title_size = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for SloBackendDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SloBackendDefinitionVisitor;
        impl<'a> Visitor<'a> for SloBackendDefinitionVisitor {
            type Value = SloBackendDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut additional_query_filters: Option<String> = None;
                let mut description: Option<String> = None;
                let mut global_time_target: Option<String> = None;
                let mut notes: Option<String> = None;
                let mut show_error_budget: Option<bool> = None;
                let mut slo_id: Option<Option<String>> = None;
                let mut time: Option<crate::datadogV1_20270101::model::WidgetTime> = None;
                let mut time_windows: Option<Vec<crate::datadogV1_20270101::model::SloWidgetTimeWindow>> = None;
                let mut title: Option<String> = None;
                let mut title_align: Option<crate::datadogV1_20270101::model::WidgetTextAlign> = None;
                let mut title_size: Option<String> = None;
                let mut type_: Option<crate::datadogV1_20270101::model::SloBackendDefinitionType> = None;
                let mut view_mode: Option<crate::datadogV1_20270101::model::SloWidgetViewMode> = None;
                let mut view_type: Option<crate::datadogV1_20270101::model::SloBackendDefinitionViewType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "additional_query_filters" => {
                            if v.is_null() {
                                continue;
                            }
                            additional_query_filters = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "global_time_target" => {
                            if v.is_null() {
                                continue;
                            }
                            global_time_target = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "notes" => {
                            if v.is_null() {
                                continue;
                            }
                            notes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "show_error_budget" => {
                            show_error_budget = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "slo_id" => {
                            slo_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "time" => {
                            if v.is_null() {
                                continue;
                            }
                            time = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _time) = time {
                                match _time {
                                    crate::datadogV1_20270101::model::WidgetTime::UnparsedObject(_time) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "time_windows" => {
                            time_windows = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "title" => {
                            if v.is_null() {
                                continue;
                            }
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "title_align" => {
                            if v.is_null() {
                                continue;
                            }
                            title_align = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _title_align) = title_align {
                                match _title_align {
                                    crate::datadogV1_20270101::model::WidgetTextAlign::UnparsedObject(_title_align) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "title_size" => {
                            if v.is_null() {
                                continue;
                            }
                            title_size = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1_20270101::model::SloBackendDefinitionType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "view_mode" => {
                            view_mode = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _view_mode) = view_mode {
                                match _view_mode {
                                    crate::datadogV1_20270101::model::SloWidgetViewMode::UnparsedObject(_view_mode) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "view_type" => {
                            view_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _view_type) = view_type {
                                match _view_type {
                                    crate::datadogV1_20270101::model::SloBackendDefinitionViewType::UnparsedObject(_view_type) => {
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
                let show_error_budget = show_error_budget.ok_or_else(|| M::Error::missing_field("show_error_budget"))?;
                let slo_id = slo_id.ok_or_else(|| M::Error::missing_field("slo_id"))?;
                let time_windows = time_windows.ok_or_else(|| M::Error::missing_field("time_windows"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;
                let view_mode = view_mode.ok_or_else(|| M::Error::missing_field("view_mode"))?;
                let view_type = view_type.ok_or_else(|| M::Error::missing_field("view_type"))?;

                let content = SloBackendDefinition {
                    additional_query_filters,
                    description,
                    global_time_target,
                    notes,
                    show_error_budget,
                    slo_id,
                    time,
                    time_windows,
                    title,
                    title_align,
                    title_size,
                    type_,
                    view_mode,
                    view_type,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SloBackendDefinitionVisitor)
    }
}
