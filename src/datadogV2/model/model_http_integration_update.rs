// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `HTTPIntegrationUpdate` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct HTTPIntegrationUpdate {
    /// Base HTTP url for the integration.
    #[serde(rename = "base_url")]
    pub base_url: Option<String>,
    /// The definition of `HTTPCredentialsUpdate` object.
    #[serde(rename = "credentials")]
    pub credentials: Option<crate::datadogV2::model::HTTPCredentialsUpdate>,
    /// The definition of `HTTPIntegrationType` object.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::HTTPIntegrationType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl HTTPIntegrationUpdate {
    pub fn new(type_: crate::datadogV2::model::HTTPIntegrationType) -> HTTPIntegrationUpdate {
        HTTPIntegrationUpdate {
            base_url: None,
            credentials: None,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn base_url(mut self, value: String) -> Self {
        self.base_url = Some(value);
        self
    }

    pub fn credentials(mut self, value: crate::datadogV2::model::HTTPCredentialsUpdate) -> Self {
        self.credentials = Some(value);
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

impl<'de> Deserialize<'de> for HTTPIntegrationUpdate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HTTPIntegrationUpdateVisitor;
        impl<'a> Visitor<'a> for HTTPIntegrationUpdateVisitor {
            type Value = HTTPIntegrationUpdate;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut base_url: Option<String> = None;
                let mut credentials: Option<crate::datadogV2::model::HTTPCredentialsUpdate> = None;
                let mut type_: Option<crate::datadogV2::model::HTTPIntegrationType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "base_url" => {
                            if v.is_null() {
                                continue;
                            }
                            base_url = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "credentials" => {
                            if v.is_null() {
                                continue;
                            }
                            credentials =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _credentials) = credentials {
                                match _credentials {
                                    crate::datadogV2::model::HTTPCredentialsUpdate::UnparsedObject(_credentials) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::HTTPIntegrationType::UnparsedObject(_type_) => {
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
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = HTTPIntegrationUpdate {
                    base_url,
                    credentials,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(HTTPIntegrationUpdateVisitor)
    }
}
