// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// API Key & Secret authentication for Twilio.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TwilioBasicAuth {
    /// Twilio API Key SID for authentication. Create from Twilio Console > Account > API Keys & Tokens.
    #[serde(rename = "api_key")]
    pub api_key: String,
    /// Twilio API Key Secret (token) corresponding to the API Key SID. This field is not returned by the API.
    #[serde(rename = "api_key_token")]
    pub api_key_token: String,
    /// Authentication method discriminator.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::TwilioBasicAuthType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TwilioBasicAuth {
    pub fn new(
        api_key: String,
        api_key_token: String,
        type_: crate::datadogV2::model::TwilioBasicAuthType,
    ) -> TwilioBasicAuth {
        TwilioBasicAuth {
            api_key,
            api_key_token,
            type_,
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

impl<'de> Deserialize<'de> for TwilioBasicAuth {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TwilioBasicAuthVisitor;
        impl<'a> Visitor<'a> for TwilioBasicAuthVisitor {
            type Value = TwilioBasicAuth;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut api_key: Option<String> = None;
                let mut api_key_token: Option<String> = None;
                let mut type_: Option<crate::datadogV2::model::TwilioBasicAuthType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "api_key" => {
                            api_key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "api_key_token" => {
                            api_key_token =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::TwilioBasicAuthType::UnparsedObject(_type_) => {
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
                let api_key = api_key.ok_or_else(|| M::Error::missing_field("api_key"))?;
                let api_key_token =
                    api_key_token.ok_or_else(|| M::Error::missing_field("api_key_token"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = TwilioBasicAuth {
                    api_key,
                    api_key_token,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TwilioBasicAuthVisitor)
    }
}
