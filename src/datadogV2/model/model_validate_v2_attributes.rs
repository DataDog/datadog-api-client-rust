// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of the API key validation response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ValidateV2Attributes {
    /// The UUID of the API key.
    #[serde(rename = "api_key_id")]
    pub api_key_id: String,
    /// List of scope names associated with the API key.
    #[serde(rename = "api_key_scopes")]
    pub api_key_scopes: Vec<String>,
    /// Whether the API key is valid.
    #[serde(rename = "valid")]
    pub valid: bool,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ValidateV2Attributes {
    pub fn new(
        api_key_id: String,
        api_key_scopes: Vec<String>,
        valid: bool,
    ) -> ValidateV2Attributes {
        ValidateV2Attributes {
            api_key_id,
            api_key_scopes,
            valid,
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

impl<'de> Deserialize<'de> for ValidateV2Attributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ValidateV2AttributesVisitor;
        impl<'a> Visitor<'a> for ValidateV2AttributesVisitor {
            type Value = ValidateV2Attributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut api_key_id: Option<String> = None;
                let mut api_key_scopes: Option<Vec<String>> = None;
                let mut valid: Option<bool> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "api_key_id" => {
                            api_key_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "api_key_scopes" => {
                            api_key_scopes =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "valid" => {
                            valid = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let api_key_id = api_key_id.ok_or_else(|| M::Error::missing_field("api_key_id"))?;
                let api_key_scopes =
                    api_key_scopes.ok_or_else(|| M::Error::missing_field("api_key_scopes"))?;
                let valid = valid.ok_or_else(|| M::Error::missing_field("valid"))?;

                let content = ValidateV2Attributes {
                    api_key_id,
                    api_key_scopes,
                    valid,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ValidateV2AttributesVisitor)
    }
}
