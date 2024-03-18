// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Updated list stream widget.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ListStreamQuery {
    /// Compute configuration for the List Stream Widget. Compute can be used only with the logs_transaction_stream (from 1 to 5 items) list stream source.
    #[serde(rename = "compute")]
    pub compute: Option<Vec<crate::datadogV1::model::ListStreamComputeItems>>,
    /// Source from which to query items to display in the stream.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1::model::ListStreamSource,
    /// Size to use to display an event.
    #[serde(rename = "event_size")]
    pub event_size: Option<crate::datadogV1::model::WidgetEventSize>,
    /// Group by configuration for the List Stream Widget. Group by can be used only with logs_pattern_stream (up to 3 items) or logs_transaction_stream (one group by item is required) list stream source.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<crate::datadogV1::model::ListStreamGroupByItems>>,
    /// List of indexes.
    #[serde(rename = "indexes")]
    pub indexes: Option<Vec<String>>,
    /// Widget query.
    #[serde(rename = "query_string")]
    pub query_string: String,
    /// Which column and order to sort by
    #[serde(rename = "sort")]
    pub sort: Option<crate::datadogV1::model::WidgetFieldSort>,
    /// Option for storage location. Feature in Private Beta.
    #[serde(rename = "storage")]
    pub storage: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ListStreamQuery {
    pub fn new(
        data_source: crate::datadogV1::model::ListStreamSource,
        query_string: String,
    ) -> ListStreamQuery {
        ListStreamQuery {
            compute: None,
            data_source,
            event_size: None,
            group_by: None,
            indexes: None,
            query_string,
            sort: None,
            storage: None,
            _unparsed: false,
        }
    }

    pub fn compute(mut self, value: Vec<crate::datadogV1::model::ListStreamComputeItems>) -> Self {
        self.compute = Some(value);
        self
    }

    pub fn event_size(mut self, value: crate::datadogV1::model::WidgetEventSize) -> Self {
        self.event_size = Some(value);
        self
    }

    pub fn group_by(mut self, value: Vec<crate::datadogV1::model::ListStreamGroupByItems>) -> Self {
        self.group_by = Some(value);
        self
    }

    pub fn indexes(mut self, value: Vec<String>) -> Self {
        self.indexes = Some(value);
        self
    }

    pub fn sort(mut self, value: crate::datadogV1::model::WidgetFieldSort) -> Self {
        self.sort = Some(value);
        self
    }

    pub fn storage(mut self, value: String) -> Self {
        self.storage = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for ListStreamQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ListStreamQueryVisitor;
        impl<'a> Visitor<'a> for ListStreamQueryVisitor {
            type Value = ListStreamQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut compute: Option<Vec<crate::datadogV1::model::ListStreamComputeItems>> =
                    None;
                let mut data_source: Option<crate::datadogV1::model::ListStreamSource> = None;
                let mut event_size: Option<crate::datadogV1::model::WidgetEventSize> = None;
                let mut group_by: Option<Vec<crate::datadogV1::model::ListStreamGroupByItems>> =
                    None;
                let mut indexes: Option<Vec<String>> = None;
                let mut query_string: Option<String> = None;
                let mut sort: Option<crate::datadogV1::model::WidgetFieldSort> = None;
                let mut storage: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "compute" => {
                            if v.is_null() {
                                continue;
                            }
                            compute = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "data_source" => {
                            data_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_source) = data_source {
                                match _data_source {
                                    crate::datadogV1::model::ListStreamSource::UnparsedObject(
                                        _data_source,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "event_size" => {
                            if v.is_null() {
                                continue;
                            }
                            event_size = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _event_size) = event_size {
                                match _event_size {
                                    crate::datadogV1::model::WidgetEventSize::UnparsedObject(
                                        _event_size,
                                    ) => {
                                        _unparsed = true;
                                    }
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
                        "indexes" => {
                            if v.is_null() {
                                continue;
                            }
                            indexes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query_string" => {
                            query_string =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sort" => {
                            if v.is_null() {
                                continue;
                            }
                            sort = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "storage" => {
                            if v.is_null() {
                                continue;
                            }
                            storage = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let data_source =
                    data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;
                let query_string =
                    query_string.ok_or_else(|| M::Error::missing_field("query_string"))?;

                let content = ListStreamQuery {
                    compute,
                    data_source,
                    event_size,
                    group_by,
                    indexes,
                    query_string,
                    sort,
                    storage,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ListStreamQueryVisitor)
    }
}
