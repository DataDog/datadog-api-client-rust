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
pub struct DistributionBackendRequest20270101OneOf4455773536 {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "number_format")]
    pub number_format: Option<crate::datadogV1_20270101::model::WidgetNumberFormat>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "query")]
    pub query: crate::datadogV1_20270101::model::DistributionBackendRequestAnyOfAnyOfItemQuery,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "request_type")]
    pub request_type: crate::datadogV1_20270101::model::DistributionBackendRequestAnyOfAnyOfItemRequestType,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "style")]
    pub style: Option<crate::datadogV1_20270101::model::RequestStyle>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl DistributionBackendRequest20270101OneOf4455773536 {
    pub fn new(query: crate::datadogV1_20270101::model::DistributionBackendRequestAnyOfAnyOfItemQuery, request_type: crate::datadogV1_20270101::model::DistributionBackendRequestAnyOfAnyOfItemRequestType) -> DistributionBackendRequest20270101OneOf4455773536 {
        DistributionBackendRequest20270101OneOf4455773536 {
            number_format: None,
            query,
            request_type,
            style: None,
            _unparsed: false,
        }
    }
    
    pub fn number_format(mut self, value: crate::datadogV1_20270101::model::WidgetNumberFormat) -> Self {
        self.number_format = Some(value);
        self
    }
    
    pub fn style(mut self, value: crate::datadogV1_20270101::model::RequestStyle) -> Self {
        self.style = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for DistributionBackendRequest20270101OneOf4455773536 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DistributionBackendRequest20270101OneOf4455773536Visitor;
        impl<'a> Visitor<'a> for DistributionBackendRequest20270101OneOf4455773536Visitor {
            type Value = DistributionBackendRequest20270101OneOf4455773536;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut number_format: Option<crate::datadogV1_20270101::model::WidgetNumberFormat> = None;
                let mut query: Option<crate::datadogV1_20270101::model::DistributionBackendRequestAnyOfAnyOfItemQuery> = None;
                let mut request_type: Option<crate::datadogV1_20270101::model::DistributionBackendRequestAnyOfAnyOfItemRequestType> = None;
                let mut style: Option<crate::datadogV1_20270101::model::RequestStyle> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "number_format" => {
                            if v.is_null() {
                                continue;
                            }
                            number_format = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "query" => {
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "request_type" => {
                            request_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _request_type) = request_type {
                                match _request_type {
                                    crate::datadogV1_20270101::model::DistributionBackendRequestAnyOfAnyOfItemRequestType::UnparsedObject(_request_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "style" => {
                            if v.is_null() {
                                continue;
                            }
                            style = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let query = query.ok_or_else(|| M::Error::missing_field("query"))?;
                let request_type = request_type.ok_or_else(|| M::Error::missing_field("request_type"))?;

                let content = DistributionBackendRequest20270101OneOf4455773536 {
                    number_format,
                    query,
                    request_type,
                    style,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DistributionBackendRequest20270101OneOf4455773536Visitor)
    }
}
