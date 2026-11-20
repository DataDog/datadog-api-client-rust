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
pub struct ManageStatusBackendDefinition {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "color_preference")]
    pub color_preference: Option<crate::datadogV1_20270101::model::ManageStatusBackendDefinitionColorPreference>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "count")]
    pub count: Option<f64>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "display_format")]
    pub display_format: Option<crate::datadogV1_20270101::model::ManageStatusBackendDefinitionDisplayFormat>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "hide_zero_counts")]
    pub hide_zero_counts: Option<bool>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "last_triggered_format")]
    pub last_triggered_format: Option<crate::datadogV1_20270101::model::ManageStatusBackendDefinitionLastTriggeredFormat>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "notes")]
    pub notes: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "query")]
    pub query: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "show_investigation")]
    pub show_investigation: Option<bool>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "show_last_triggered")]
    pub show_last_triggered: Option<bool>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "show_priority")]
    pub show_priority: Option<bool>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "show_status")]
    pub show_status: Option<bool>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "sort")]
    pub sort: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "start")]
    pub start: Option<f64>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "summary_type")]
    pub summary_type: Option<crate::datadogV1_20270101::model::ManageStatusBackendDefinitionSummaryType>,
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
    pub type_: crate::datadogV1_20270101::model::ManageStatusBackendDefinitionType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ManageStatusBackendDefinition {
    pub fn new(query: String, type_: crate::datadogV1_20270101::model::ManageStatusBackendDefinitionType) -> ManageStatusBackendDefinition {
        ManageStatusBackendDefinition {
            color_preference: None,
            count: None,
            description: None,
            display_format: None,
            hide_zero_counts: None,
            last_triggered_format: None,
            notes: None,
            query,
            show_investigation: None,
            show_last_triggered: None,
            show_priority: None,
            show_status: None,
            sort: None,
            start: None,
            summary_type: None,
            title: None,
            title_align: None,
            title_size: None,
            type_,
            _unparsed: false,
        }
    }
    
    pub fn color_preference(mut self, value: crate::datadogV1_20270101::model::ManageStatusBackendDefinitionColorPreference) -> Self {
        self.color_preference = Some(value);
        self
    }
    
    pub fn count(mut self, value: f64) -> Self {
        self.count = Some(value);
        self
    }
    
    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }
    
    pub fn display_format(mut self, value: crate::datadogV1_20270101::model::ManageStatusBackendDefinitionDisplayFormat) -> Self {
        self.display_format = Some(value);
        self
    }
    
    pub fn hide_zero_counts(mut self, value: bool) -> Self {
        self.hide_zero_counts = Some(value);
        self
    }
    
    pub fn last_triggered_format(mut self, value: crate::datadogV1_20270101::model::ManageStatusBackendDefinitionLastTriggeredFormat) -> Self {
        self.last_triggered_format = Some(value);
        self
    }
    
    pub fn notes(mut self, value: String) -> Self {
        self.notes = Some(value);
        self
    }
    
    pub fn show_investigation(mut self, value: bool) -> Self {
        self.show_investigation = Some(value);
        self
    }
    
    pub fn show_last_triggered(mut self, value: bool) -> Self {
        self.show_last_triggered = Some(value);
        self
    }
    
    pub fn show_priority(mut self, value: bool) -> Self {
        self.show_priority = Some(value);
        self
    }
    
    pub fn show_status(mut self, value: bool) -> Self {
        self.show_status = Some(value);
        self
    }
    
    pub fn sort(mut self, value: String) -> Self {
        self.sort = Some(value);
        self
    }
    
    pub fn start(mut self, value: f64) -> Self {
        self.start = Some(value);
        self
    }
    
    pub fn summary_type(mut self, value: crate::datadogV1_20270101::model::ManageStatusBackendDefinitionSummaryType) -> Self {
        self.summary_type = Some(value);
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



impl<'de> Deserialize<'de> for ManageStatusBackendDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ManageStatusBackendDefinitionVisitor;
        impl<'a> Visitor<'a> for ManageStatusBackendDefinitionVisitor {
            type Value = ManageStatusBackendDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut color_preference: Option<crate::datadogV1_20270101::model::ManageStatusBackendDefinitionColorPreference> = None;
                let mut count: Option<f64> = None;
                let mut description: Option<String> = None;
                let mut display_format: Option<crate::datadogV1_20270101::model::ManageStatusBackendDefinitionDisplayFormat> = None;
                let mut hide_zero_counts: Option<bool> = None;
                let mut last_triggered_format: Option<crate::datadogV1_20270101::model::ManageStatusBackendDefinitionLastTriggeredFormat> = None;
                let mut notes: Option<String> = None;
                let mut query: Option<String> = None;
                let mut show_investigation: Option<bool> = None;
                let mut show_last_triggered: Option<bool> = None;
                let mut show_priority: Option<bool> = None;
                let mut show_status: Option<bool> = None;
                let mut sort: Option<String> = None;
                let mut start: Option<f64> = None;
                let mut summary_type: Option<crate::datadogV1_20270101::model::ManageStatusBackendDefinitionSummaryType> = None;
                let mut title: Option<String> = None;
                let mut title_align: Option<crate::datadogV1_20270101::model::WidgetTextAlign> = None;
                let mut title_size: Option<String> = None;
                let mut type_: Option<crate::datadogV1_20270101::model::ManageStatusBackendDefinitionType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "color_preference" => {
                            if v.is_null() {
                                continue;
                            }
                            color_preference = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _color_preference) = color_preference {
                                match _color_preference {
                                    crate::datadogV1_20270101::model::ManageStatusBackendDefinitionColorPreference::UnparsedObject(_color_preference) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "count" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            count = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "display_format" => {
                            if v.is_null() {
                                continue;
                            }
                            display_format = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _display_format) = display_format {
                                match _display_format {
                                    crate::datadogV1_20270101::model::ManageStatusBackendDefinitionDisplayFormat::UnparsedObject(_display_format) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "hide_zero_counts" => {
                            if v.is_null() {
                                continue;
                            }
                            hide_zero_counts = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "last_triggered_format" => {
                            if v.is_null() {
                                continue;
                            }
                            last_triggered_format = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _last_triggered_format) = last_triggered_format {
                                match _last_triggered_format {
                                    crate::datadogV1_20270101::model::ManageStatusBackendDefinitionLastTriggeredFormat::UnparsedObject(_last_triggered_format) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "notes" => {
                            if v.is_null() {
                                continue;
                            }
                            notes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "query" => {
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "show_investigation" => {
                            if v.is_null() {
                                continue;
                            }
                            show_investigation = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "show_last_triggered" => {
                            if v.is_null() {
                                continue;
                            }
                            show_last_triggered = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "show_priority" => {
                            if v.is_null() {
                                continue;
                            }
                            show_priority = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "show_status" => {
                            if v.is_null() {
                                continue;
                            }
                            show_status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "sort" => {
                            if v.is_null() {
                                continue;
                            }
                            sort = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "start" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            start = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "summary_type" => {
                            if v.is_null() {
                                continue;
                            }
                            summary_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _summary_type) = summary_type {
                                match _summary_type {
                                    crate::datadogV1_20270101::model::ManageStatusBackendDefinitionSummaryType::UnparsedObject(_summary_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
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
                                    crate::datadogV1_20270101::model::ManageStatusBackendDefinitionType::UnparsedObject(_type_) => {
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
                let query = query.ok_or_else(|| M::Error::missing_field("query"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ManageStatusBackendDefinition {
                    color_preference,
                    count,
                    description,
                    display_format,
                    hide_zero_counts,
                    last_triggered_format,
                    notes,
                    query,
                    show_investigation,
                    show_last_triggered,
                    show_priority,
                    show_status,
                    sort,
                    start,
                    summary_type,
                    title,
                    title_align,
                    title_size,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ManageStatusBackendDefinitionVisitor)
    }
}
