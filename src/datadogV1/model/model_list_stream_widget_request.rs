// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Updated list stream widget.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ListStreamWidgetRequest {
    /// Widget columns.
    #[serde(rename = "columns")]
    pub columns: Vec<crate::datadogV1::model::ListStreamColumn>,
    /// Updated list stream widget.
    #[serde(rename = "query")]
    pub query: crate::datadogV1::model::ListStreamQuery,
    /// Widget response format.
    #[serde(rename = "response_format")]
    pub response_format: crate::datadogV1::model::ListStreamResponseFormat,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ListStreamWidgetRequest {
    pub fn new(
        columns: Vec<crate::datadogV1::model::ListStreamColumn>,
        query: crate::datadogV1::model::ListStreamQuery,
        response_format: crate::datadogV1::model::ListStreamResponseFormat,
    ) -> ListStreamWidgetRequest {
        ListStreamWidgetRequest {
            columns,
            query,
            response_format,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for ListStreamWidgetRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ListStreamWidgetRequestVisitor;
        impl<'a> Visitor<'a> for ListStreamWidgetRequestVisitor {
            type Value = ListStreamWidgetRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut columns: Option<Vec<crate::datadogV1::model::ListStreamColumn>> = None;
                let mut query: Option<crate::datadogV1::model::ListStreamQuery> = None;
                let mut response_format: Option<crate::datadogV1::model::ListStreamResponseFormat> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "columns" => {
                            columns = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "response_format" => {
                            response_format =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _response_format) = response_format {
                                match _response_format {
                                    crate::datadogV1::model::ListStreamResponseFormat::UnparsedObject(_response_format) => {
                                        _unparsed = true;
                                    },
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
                let columns = columns.ok_or_else(|| M::Error::missing_field("columns"))?;
                let query = query.ok_or_else(|| M::Error::missing_field("query"))?;
                let response_format =
                    response_format.ok_or_else(|| M::Error::missing_field("response_format"))?;

                let content = ListStreamWidgetRequest {
                    columns,
                    query,
                    response_format,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ListStreamWidgetRequestVisitor)
    }
}
