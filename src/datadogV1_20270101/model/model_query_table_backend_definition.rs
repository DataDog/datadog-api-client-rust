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
pub struct QueryTableBackendDefinition {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "auto_horizontal_scrolling")]
    pub auto_horizontal_scrolling: Option<bool>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "custom_links")]
    pub custom_links: Option<Vec<crate::datadogV1_20270101::model::WidgetCustomLink>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "has_search_bar")]
    pub has_search_bar: Option<crate::datadogV1_20270101::model::HasSearchBar>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "notes")]
    pub notes: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "num_sticky_columns")]
    pub num_sticky_columns: Option<f64>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "requests")]
    pub requests: Vec<crate::datadogV1_20270101::model::QueryTableBackendRequest>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "time")]
    pub time: Option<crate::datadogV1_20270101::model::WidgetTime>,
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
    pub type_: crate::datadogV1_20270101::model::QueryTableBackendDefinitionType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl QueryTableBackendDefinition {
    pub fn new(requests: Vec<crate::datadogV1_20270101::model::QueryTableBackendRequest>, type_: crate::datadogV1_20270101::model::QueryTableBackendDefinitionType) -> QueryTableBackendDefinition {
        QueryTableBackendDefinition {
            auto_horizontal_scrolling: None,
            custom_links: None,
            description: None,
            has_search_bar: None,
            notes: None,
            num_sticky_columns: None,
            requests,
            time: None,
            title: None,
            title_align: None,
            title_size: None,
            type_,
            _unparsed: false,
        }
    }
    
    pub fn auto_horizontal_scrolling(mut self, value: bool) -> Self {
        self.auto_horizontal_scrolling = Some(value);
        self
    }
    
    pub fn custom_links(mut self, value: Vec<crate::datadogV1_20270101::model::WidgetCustomLink>) -> Self {
        self.custom_links = Some(value);
        self
    }
    
    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }
    
    pub fn has_search_bar(mut self, value: crate::datadogV1_20270101::model::HasSearchBar) -> Self {
        self.has_search_bar = Some(value);
        self
    }
    
    pub fn notes(mut self, value: String) -> Self {
        self.notes = Some(value);
        self
    }
    
    pub fn num_sticky_columns(mut self, value: f64) -> Self {
        self.num_sticky_columns = Some(value);
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



impl<'de> Deserialize<'de> for QueryTableBackendDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct QueryTableBackendDefinitionVisitor;
        impl<'a> Visitor<'a> for QueryTableBackendDefinitionVisitor {
            type Value = QueryTableBackendDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut auto_horizontal_scrolling: Option<bool> = None;
                let mut custom_links: Option<Vec<crate::datadogV1_20270101::model::WidgetCustomLink>> = None;
                let mut description: Option<String> = None;
                let mut has_search_bar: Option<crate::datadogV1_20270101::model::HasSearchBar> = None;
                let mut notes: Option<String> = None;
                let mut num_sticky_columns: Option<f64> = None;
                let mut requests: Option<Vec<crate::datadogV1_20270101::model::QueryTableBackendRequest>> = None;
                let mut time: Option<crate::datadogV1_20270101::model::WidgetTime> = None;
                let mut title: Option<String> = None;
                let mut title_align: Option<crate::datadogV1_20270101::model::WidgetTextAlign> = None;
                let mut title_size: Option<String> = None;
                let mut type_: Option<crate::datadogV1_20270101::model::QueryTableBackendDefinitionType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "auto_horizontal_scrolling" => {
                            if v.is_null() {
                                continue;
                            }
                            auto_horizontal_scrolling = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "custom_links" => {
                            if v.is_null() {
                                continue;
                            }
                            custom_links = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "has_search_bar" => {
                            if v.is_null() {
                                continue;
                            }
                            has_search_bar = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _has_search_bar) = has_search_bar {
                                match _has_search_bar {
                                    crate::datadogV1_20270101::model::HasSearchBar::UnparsedObject(_has_search_bar) => {
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
                        "num_sticky_columns" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            num_sticky_columns = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "requests" => {
                            requests = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                                    crate::datadogV1_20270101::model::QueryTableBackendDefinitionType::UnparsedObject(_type_) => {
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
                let requests = requests.ok_or_else(|| M::Error::missing_field("requests"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = QueryTableBackendDefinition {
                    auto_horizontal_scrolling,
                    custom_links,
                    description,
                    has_search_bar,
                    notes,
                    num_sticky_columns,
                    requests,
                    time,
                    title,
                    title_align,
                    title_size,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(QueryTableBackendDefinitionVisitor)
    }
}
