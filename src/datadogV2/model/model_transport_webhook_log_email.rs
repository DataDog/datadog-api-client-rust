// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The email address details.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TransportWebhookLogEmail {
    /// The recipient email address.
    #[serde(rename = "address")]
    pub address: Option<String>,
    /// The recipient domain.
    #[serde(rename = "domain")]
    pub domain: Option<String>,
    /// The email subject line.
    #[serde(rename = "subject")]
    pub subject: Option<String>,
    /// Email categorization tags applied by the transport provider (for example, "transactional", "marketing").
    #[serde(rename = "type")]
    pub type_: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TransportWebhookLogEmail {
    pub fn new() -> TransportWebhookLogEmail {
        TransportWebhookLogEmail {
            address: None,
            domain: None,
            subject: None,
            type_: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn address(mut self, value: String) -> Self {
        self.address = Some(value);
        self
    }

    pub fn domain(mut self, value: String) -> Self {
        self.domain = Some(value);
        self
    }

    pub fn subject(mut self, value: String) -> Self {
        self.subject = Some(value);
        self
    }

    pub fn type_(mut self, value: Vec<String>) -> Self {
        self.type_ = Some(value);
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

impl Default for TransportWebhookLogEmail {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TransportWebhookLogEmail {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TransportWebhookLogEmailVisitor;
        impl<'a> Visitor<'a> for TransportWebhookLogEmailVisitor {
            type Value = TransportWebhookLogEmail;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut address: Option<String> = None;
                let mut domain: Option<String> = None;
                let mut subject: Option<String> = None;
                let mut type_: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "address" => {
                            if v.is_null() {
                                continue;
                            }
                            address = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "domain" => {
                            if v.is_null() {
                                continue;
                            }
                            domain = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "subject" => {
                            if v.is_null() {
                                continue;
                            }
                            subject = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = TransportWebhookLogEmail {
                    address,
                    domain,
                    subject,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TransportWebhookLogEmailVisitor)
    }
}
