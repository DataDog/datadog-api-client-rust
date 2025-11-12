// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for an on-call email.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EmailAttributes {
    /// Whether the email is currently active.
    #[serde(rename = "active")]
    pub active: Option<bool>,
    /// Email address.
    #[serde(rename = "address")]
    pub address: Option<String>,
    /// Optional display alias for the email.
    #[serde(rename = "alias")]
    pub alias: Option<String>,
    /// Preferred content formats for notifications.
    #[serde(rename = "formats")]
    pub formats: Option<Vec<crate::datadogV2::model::EmailFormatType>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl EmailAttributes {
    pub fn new() -> EmailAttributes {
        EmailAttributes {
            active: None,
            address: None,
            alias: None,
            formats: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn active(mut self, value: bool) -> Self {
        self.active = Some(value);
        self
    }

    pub fn address(mut self, value: String) -> Self {
        self.address = Some(value);
        self
    }

    pub fn alias(mut self, value: String) -> Self {
        self.alias = Some(value);
        self
    }

    pub fn formats(mut self, value: Vec<crate::datadogV2::model::EmailFormatType>) -> Self {
        self.formats = Some(value);
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

impl Default for EmailAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for EmailAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EmailAttributesVisitor;
        impl<'a> Visitor<'a> for EmailAttributesVisitor {
            type Value = EmailAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut active: Option<bool> = None;
                let mut address: Option<String> = None;
                let mut alias: Option<String> = None;
                let mut formats: Option<Vec<crate::datadogV2::model::EmailFormatType>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "active" => {
                            if v.is_null() {
                                continue;
                            }
                            active = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "address" => {
                            if v.is_null() {
                                continue;
                            }
                            address = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "alias" => {
                            if v.is_null() {
                                continue;
                            }
                            alias = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "formats" => {
                            if v.is_null() {
                                continue;
                            }
                            formats = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = EmailAttributes {
                    active,
                    address,
                    alias,
                    formats,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(EmailAttributesVisitor)
    }
}
