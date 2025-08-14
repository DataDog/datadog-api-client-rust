// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `HTTPTokenAuth` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct HTTPTokenAuth {
    /// The definition of `HTTPBody` object.
    #[serde(rename = "body")]
    pub body: Option<crate::datadogV2::model::HTTPBody>,
    /// The `HTTPTokenAuth` `headers`.
    #[serde(rename = "headers")]
    pub headers: Option<Vec<crate::datadogV2::model::HTTPHeader>>,
    /// The `HTTPTokenAuth` `tokens`.
    #[serde(rename = "tokens")]
    pub tokens: Option<Vec<crate::datadogV2::model::HTTPToken>>,
    /// The definition of the `HTTPTokenAuth` object.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::HTTPTokenAuthType,
    /// The `HTTPTokenAuth` `url_parameters`.
    #[serde(rename = "url_parameters")]
    pub url_parameters: Option<Vec<crate::datadogV2::model::UrlParam>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl HTTPTokenAuth {
    pub fn new(type_: crate::datadogV2::model::HTTPTokenAuthType) -> HTTPTokenAuth {
        HTTPTokenAuth {
            body: None,
            headers: None,
            tokens: None,
            type_,
            url_parameters: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn body(mut self, value: crate::datadogV2::model::HTTPBody) -> Self {
        self.body = Some(value);
        self
    }

    pub fn headers(mut self, value: Vec<crate::datadogV2::model::HTTPHeader>) -> Self {
        self.headers = Some(value);
        self
    }

    pub fn tokens(mut self, value: Vec<crate::datadogV2::model::HTTPToken>) -> Self {
        self.tokens = Some(value);
        self
    }

    pub fn url_parameters(mut self, value: Vec<crate::datadogV2::model::UrlParam>) -> Self {
        self.url_parameters = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for HTTPTokenAuth {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HTTPTokenAuthVisitor;
        impl<'a> Visitor<'a> for HTTPTokenAuthVisitor {
            type Value = HTTPTokenAuth;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut body: Option<crate::datadogV2::model::HTTPBody> = None;
                let mut headers: Option<Vec<crate::datadogV2::model::HTTPHeader>> = None;
                let mut tokens: Option<Vec<crate::datadogV2::model::HTTPToken>> = None;
                let mut type_: Option<crate::datadogV2::model::HTTPTokenAuthType> = None;
                let mut url_parameters: Option<Vec<crate::datadogV2::model::UrlParam>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "body" => {
                            if v.is_null() {
                                continue;
                            }
                            body = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "headers" => {
                            if v.is_null() {
                                continue;
                            }
                            headers = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tokens" => {
                            if v.is_null() {
                                continue;
                            }
                            tokens = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::HTTPTokenAuthType::UnparsedObject(
                                        _type_,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "url_parameters" => {
                            if v.is_null() {
                                continue;
                            }
                            url_parameters =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = HTTPTokenAuth {
                    body,
                    headers,
                    tokens,
                    type_,
                    url_parameters,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(HTTPTokenAuthVisitor)
    }
}
