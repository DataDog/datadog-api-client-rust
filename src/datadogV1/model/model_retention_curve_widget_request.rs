// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Retention curve widget request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RetentionCurveWidgetRequest {
    /// Retention query definition.
    #[serde(rename = "query")]
    pub query: crate::datadogV1::model::RetentionQuery,
    /// Request type for retention curve widget.
    #[serde(rename = "request_type")]
    pub request_type: crate::datadogV1::model::RetentionCurveRequestType,
    /// Style configuration for retention curve.
    #[serde(rename = "style")]
    pub style: Option<crate::datadogV1::model::RetentionCurveStyle>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RetentionCurveWidgetRequest {
    pub fn new(
        query: crate::datadogV1::model::RetentionQuery,
        request_type: crate::datadogV1::model::RetentionCurveRequestType,
    ) -> RetentionCurveWidgetRequest {
        RetentionCurveWidgetRequest {
            query,
            request_type,
            style: None,
            _unparsed: false,
        }
    }

    pub fn style(mut self, value: crate::datadogV1::model::RetentionCurveStyle) -> Self {
        self.style = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for RetentionCurveWidgetRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RetentionCurveWidgetRequestVisitor;
        impl<'a> Visitor<'a> for RetentionCurveWidgetRequestVisitor {
            type Value = RetentionCurveWidgetRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut query: Option<crate::datadogV1::model::RetentionQuery> = None;
                let mut request_type: Option<crate::datadogV1::model::RetentionCurveRequestType> =
                    None;
                let mut style: Option<crate::datadogV1::model::RetentionCurveStyle> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "query" => {
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "request_type" => {
                            request_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _request_type) = request_type {
                                match _request_type {
                                    crate::datadogV1::model::RetentionCurveRequestType::UnparsedObject(_request_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "style" => {
                            if v.is_null() {
                                continue;
                            }
                            style = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let query = query.ok_or_else(|| M::Error::missing_field("query"))?;
                let request_type =
                    request_type.ok_or_else(|| M::Error::missing_field("request_type"))?;

                let content = RetentionCurveWidgetRequest {
                    query,
                    request_type,
                    style,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RetentionCurveWidgetRequestVisitor)
    }
}
