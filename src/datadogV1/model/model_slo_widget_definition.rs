// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Use the SLO and uptime widget to track your SLOs (Service Level Objectives) and uptime on screenboards and timeboards.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SLOWidgetDefinition {
    /// Additional filters applied to the SLO query.
    #[serde(rename = "additional_query_filters")]
    pub additional_query_filters: Option<String>,
    /// Defined global time target.
    #[serde(rename = "global_time_target")]
    pub global_time_target: Option<String>,
    /// Defined error budget.
    #[serde(rename = "show_error_budget")]
    pub show_error_budget: Option<bool>,
    /// ID of the SLO displayed.
    #[serde(rename = "slo_id")]
    pub slo_id: Option<String>,
    /// Times being monitored.
    #[serde(rename = "time_windows")]
    pub time_windows: Option<Vec<crate::datadogV1::model::WidgetTimeWindows>>,
    /// Title of the widget.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// How to align the text on the widget.
    #[serde(rename = "title_align")]
    pub title_align: Option<crate::datadogV1::model::WidgetTextAlign>,
    /// Size of the title.
    #[serde(rename = "title_size")]
    pub title_size: Option<String>,
    /// Type of the SLO widget.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::SLOWidgetDefinitionType,
    /// Define how you want the SLO to be displayed.
    #[serde(rename = "view_mode")]
    pub view_mode: Option<crate::datadogV1::model::WidgetViewMode>,
    /// Type of view displayed by the widget.
    #[serde(rename = "view_type")]
    pub view_type: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SLOWidgetDefinition {
    pub fn new(
        type_: crate::datadogV1::model::SLOWidgetDefinitionType,
        view_type: String,
    ) -> SLOWidgetDefinition {
        SLOWidgetDefinition {
            additional_query_filters: None,
            global_time_target: None,
            show_error_budget: None,
            slo_id: None,
            time_windows: None,
            title: None,
            title_align: None,
            title_size: None,
            type_,
            view_mode: None,
            view_type,
            _unparsed: false,
        }
    }

    pub fn additional_query_filters(mut self, value: String) -> Self {
        self.additional_query_filters = Some(value);
        self
    }

    pub fn global_time_target(mut self, value: String) -> Self {
        self.global_time_target = Some(value);
        self
    }

    pub fn show_error_budget(mut self, value: bool) -> Self {
        self.show_error_budget = Some(value);
        self
    }

    pub fn slo_id(mut self, value: String) -> Self {
        self.slo_id = Some(value);
        self
    }

    pub fn time_windows(mut self, value: Vec<crate::datadogV1::model::WidgetTimeWindows>) -> Self {
        self.time_windows = Some(value);
        self
    }

    pub fn title(mut self, value: String) -> Self {
        self.title = Some(value);
        self
    }

    pub fn title_align(mut self, value: crate::datadogV1::model::WidgetTextAlign) -> Self {
        self.title_align = Some(value);
        self
    }

    pub fn title_size(mut self, value: String) -> Self {
        self.title_size = Some(value);
        self
    }

    pub fn view_mode(mut self, value: crate::datadogV1::model::WidgetViewMode) -> Self {
        self.view_mode = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for SLOWidgetDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SLOWidgetDefinitionVisitor;
        impl<'a> Visitor<'a> for SLOWidgetDefinitionVisitor {
            type Value = SLOWidgetDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut additional_query_filters: Option<String> = None;
                let mut global_time_target: Option<String> = None;
                let mut show_error_budget: Option<bool> = None;
                let mut slo_id: Option<String> = None;
                let mut time_windows: Option<Vec<crate::datadogV1::model::WidgetTimeWindows>> =
                    None;
                let mut title: Option<String> = None;
                let mut title_align: Option<crate::datadogV1::model::WidgetTextAlign> = None;
                let mut title_size: Option<String> = None;
                let mut type_: Option<crate::datadogV1::model::SLOWidgetDefinitionType> = None;
                let mut view_mode: Option<crate::datadogV1::model::WidgetViewMode> = None;
                let mut view_type: Option<String> = None;
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
                        "global_time_target" => {
                            if v.is_null() {
                                continue;
                            }
                            global_time_target =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "show_error_budget" => {
                            if v.is_null() {
                                continue;
                            }
                            show_error_budget =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "slo_id" => {
                            if v.is_null() {
                                continue;
                            }
                            slo_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "time_windows" => {
                            if v.is_null() {
                                continue;
                            }
                            time_windows =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "title" => {
                            if v.is_null() {
                                continue;
                            }
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "title_align" => {
                            if v.is_null() {
                                continue;
                            }
                            title_align =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _title_align) = title_align {
                                match _title_align {
                                    crate::datadogV1::model::WidgetTextAlign::UnparsedObject(
                                        _title_align,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "title_size" => {
                            if v.is_null() {
                                continue;
                            }
                            title_size = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::SLOWidgetDefinitionType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "view_mode" => {
                            if v.is_null() {
                                continue;
                            }
                            view_mode = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _view_mode) = view_mode {
                                match _view_mode {
                                    crate::datadogV1::model::WidgetViewMode::UnparsedObject(
                                        _view_mode,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "view_type" => {
                            view_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;
                let view_type = view_type.ok_or_else(|| M::Error::missing_field("view_type"))?;

                let content = SLOWidgetDefinition {
                    additional_query_filters,
                    global_time_target,
                    show_error_budget,
                    slo_id,
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

        deserializer.deserialize_any(SLOWidgetDefinitionVisitor)
    }
}
