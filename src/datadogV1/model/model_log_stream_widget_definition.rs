// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The Log Stream displays a log flow matching the defined query. Only available on FREE layout dashboards.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogStreamWidgetDefinition {
    /// Which columns to display on the widget.
    #[serde(rename = "columns")]
    pub columns: Option<Vec<String>>,
    /// An array of index names to query in the stream. Use [] to query all indexes at once.
    #[serde(rename = "indexes")]
    pub indexes: Option<Vec<String>>,
    /// ID of the log set to use.
    #[deprecated]
    #[serde(rename = "logset")]
    pub logset: Option<String>,
    /// Amount of log lines to display
    #[serde(rename = "message_display")]
    pub message_display: Option<crate::datadogV1::model::WidgetMessageDisplay>,
    /// Query to filter the log stream with.
    #[serde(rename = "query")]
    pub query: Option<String>,
    /// Whether to show the date column or not
    #[serde(rename = "show_date_column")]
    pub show_date_column: Option<bool>,
    /// Whether to show the message column or not
    #[serde(rename = "show_message_column")]
    pub show_message_column: Option<bool>,
    /// Which column and order to sort by
    #[serde(rename = "sort")]
    pub sort: Option<crate::datadogV1::model::WidgetFieldSort>,
    /// Time setting for the widget.
    #[serde(rename = "time")]
    pub time: Option<crate::datadogV1::model::WidgetTime>,
    /// Title of the widget.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// How to align the text on the widget.
    #[serde(rename = "title_align")]
    pub title_align: Option<crate::datadogV1::model::WidgetTextAlign>,
    /// Size of the title.
    #[serde(rename = "title_size")]
    pub title_size: Option<String>,
    /// Type of the log stream widget.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::LogStreamWidgetDefinitionType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogStreamWidgetDefinition {
    pub fn new(
        type_: crate::datadogV1::model::LogStreamWidgetDefinitionType,
    ) -> LogStreamWidgetDefinition {
        #[allow(deprecated)]
        LogStreamWidgetDefinition {
            columns: None,
            indexes: None,
            logset: None,
            message_display: None,
            query: None,
            show_date_column: None,
            show_message_column: None,
            sort: None,
            time: None,
            title: None,
            title_align: None,
            title_size: None,
            type_,
            _unparsed: false,
        }
    }

    #[allow(deprecated)]
    pub fn columns(mut self, value: Vec<String>) -> Self {
        self.columns = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn indexes(mut self, value: Vec<String>) -> Self {
        self.indexes = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn logset(mut self, value: String) -> Self {
        self.logset = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn message_display(mut self, value: crate::datadogV1::model::WidgetMessageDisplay) -> Self {
        self.message_display = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn query(mut self, value: String) -> Self {
        self.query = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn show_date_column(mut self, value: bool) -> Self {
        self.show_date_column = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn show_message_column(mut self, value: bool) -> Self {
        self.show_message_column = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn sort(mut self, value: crate::datadogV1::model::WidgetFieldSort) -> Self {
        self.sort = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn time(mut self, value: crate::datadogV1::model::WidgetTime) -> Self {
        self.time = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn title(mut self, value: String) -> Self {
        self.title = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn title_align(mut self, value: crate::datadogV1::model::WidgetTextAlign) -> Self {
        self.title_align = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn title_size(mut self, value: String) -> Self {
        self.title_size = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for LogStreamWidgetDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogStreamWidgetDefinitionVisitor;
        impl<'a> Visitor<'a> for LogStreamWidgetDefinitionVisitor {
            type Value = LogStreamWidgetDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut columns: Option<Vec<String>> = None;
                let mut indexes: Option<Vec<String>> = None;
                let mut logset: Option<String> = None;
                let mut message_display: Option<crate::datadogV1::model::WidgetMessageDisplay> =
                    None;
                let mut query: Option<String> = None;
                let mut show_date_column: Option<bool> = None;
                let mut show_message_column: Option<bool> = None;
                let mut sort: Option<crate::datadogV1::model::WidgetFieldSort> = None;
                let mut time: Option<crate::datadogV1::model::WidgetTime> = None;
                let mut title: Option<String> = None;
                let mut title_align: Option<crate::datadogV1::model::WidgetTextAlign> = None;
                let mut title_size: Option<String> = None;
                let mut type_: Option<crate::datadogV1::model::LogStreamWidgetDefinitionType> =
                    None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "columns" => {
                            if v.is_null() {
                                continue;
                            }
                            columns = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "indexes" => {
                            if v.is_null() {
                                continue;
                            }
                            indexes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "logset" => {
                            if v.is_null() {
                                continue;
                            }
                            logset = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "message_display" => {
                            if v.is_null() {
                                continue;
                            }
                            message_display =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _message_display) = message_display {
                                match _message_display {
                                    crate::datadogV1::model::WidgetMessageDisplay::UnparsedObject(_message_display) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "query" => {
                            if v.is_null() {
                                continue;
                            }
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "show_date_column" => {
                            if v.is_null() {
                                continue;
                            }
                            show_date_column =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "show_message_column" => {
                            if v.is_null() {
                                continue;
                            }
                            show_message_column =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sort" => {
                            if v.is_null() {
                                continue;
                            }
                            sort = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "time" => {
                            if v.is_null() {
                                continue;
                            }
                            time = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                                    crate::datadogV1::model::LogStreamWidgetDefinitionType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                #[allow(deprecated)]
                let content = LogStreamWidgetDefinition {
                    columns,
                    indexes,
                    logset,
                    message_display,
                    query,
                    show_date_column,
                    show_message_column,
                    sort,
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

        deserializer.deserialize_any(LogStreamWidgetDefinitionVisitor)
    }
}
