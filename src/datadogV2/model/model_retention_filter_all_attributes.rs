// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes of the retention filter.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RetentionFilterAllAttributes {
    /// The creation timestamp of the retention filter.
    #[serde(rename = "created_at")]
    pub created_at: Option<i64>,
    /// The creator of the retention filter.
    #[serde(rename = "created_by")]
    pub created_by: Option<String>,
    /// Shows whether the filter can be edited.
    #[serde(rename = "editable")]
    pub editable: Option<bool>,
    /// The status of the retention filter (Enabled/Disabled).
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// The execution order of the retention filter.
    #[serde(rename = "execution_order")]
    pub execution_order: Option<i64>,
    /// The spans filter used to index spans.
    #[serde(rename = "filter")]
    pub filter: Option<crate::datadogV2::model::SpansFilter>,
    /// The type of retention filter.
    #[serde(rename = "filter_type")]
    pub filter_type: Option<crate::datadogV2::model::RetentionFilterAllType>,
    /// The modification timestamp of the retention filter.
    #[serde(rename = "modified_at")]
    pub modified_at: Option<i64>,
    /// The modifier of the retention filter.
    #[serde(rename = "modified_by")]
    pub modified_by: Option<String>,
    /// The name of the retention filter.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Sample rate to apply to spans going through this retention filter,
    /// a value of 1.0 keeps all spans matching the query.
    #[serde(rename = "rate")]
    pub rate: Option<f64>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RetentionFilterAllAttributes {
    pub fn new() -> RetentionFilterAllAttributes {
        RetentionFilterAllAttributes {
            created_at: None,
            created_by: None,
            editable: None,
            enabled: None,
            execution_order: None,
            filter: None,
            filter_type: None,
            modified_at: None,
            modified_by: None,
            name: None,
            rate: None,
            _unparsed: false,
        }
    }

    pub fn created_at(mut self, value: i64) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn created_by(mut self, value: String) -> Self {
        self.created_by = Some(value);
        self
    }

    pub fn editable(mut self, value: bool) -> Self {
        self.editable = Some(value);
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn execution_order(mut self, value: i64) -> Self {
        self.execution_order = Some(value);
        self
    }

    pub fn filter(mut self, value: crate::datadogV2::model::SpansFilter) -> Self {
        self.filter = Some(value);
        self
    }

    pub fn filter_type(mut self, value: crate::datadogV2::model::RetentionFilterAllType) -> Self {
        self.filter_type = Some(value);
        self
    }

    pub fn modified_at(mut self, value: i64) -> Self {
        self.modified_at = Some(value);
        self
    }

    pub fn modified_by(mut self, value: String) -> Self {
        self.modified_by = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn rate(mut self, value: f64) -> Self {
        self.rate = Some(value);
        self
    }
}

impl Default for RetentionFilterAllAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for RetentionFilterAllAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RetentionFilterAllAttributesVisitor;
        impl<'a> Visitor<'a> for RetentionFilterAllAttributesVisitor {
            type Value = RetentionFilterAllAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<i64> = None;
                let mut created_by: Option<String> = None;
                let mut editable: Option<bool> = None;
                let mut enabled: Option<bool> = None;
                let mut execution_order: Option<i64> = None;
                let mut filter: Option<crate::datadogV2::model::SpansFilter> = None;
                let mut filter_type: Option<crate::datadogV2::model::RetentionFilterAllType> = None;
                let mut modified_at: Option<i64> = None;
                let mut modified_by: Option<String> = None;
                let mut name: Option<String> = None;
                let mut rate: Option<f64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_by" => {
                            if v.is_null() {
                                continue;
                            }
                            created_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "editable" => {
                            if v.is_null() {
                                continue;
                            }
                            editable = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "execution_order" => {
                            if v.is_null() {
                                continue;
                            }
                            execution_order =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "filter" => {
                            if v.is_null() {
                                continue;
                            }
                            filter = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "filter_type" => {
                            if v.is_null() {
                                continue;
                            }
                            filter_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _filter_type) = filter_type {
                                match _filter_type {
                                    crate::datadogV2::model::RetentionFilterAllType::UnparsedObject(_filter_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "modified_at" => {
                            if v.is_null() {
                                continue;
                            }
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_by" => {
                            if v.is_null() {
                                continue;
                            }
                            modified_by =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rate" => {
                            if v.is_null() {
                                continue;
                            }
                            rate = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = RetentionFilterAllAttributes {
                    created_at,
                    created_by,
                    editable,
                    enabled,
                    execution_order,
                    filter,
                    filter_type,
                    modified_at,
                    modified_by,
                    name,
                    rate,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RetentionFilterAllAttributesVisitor)
    }
}
