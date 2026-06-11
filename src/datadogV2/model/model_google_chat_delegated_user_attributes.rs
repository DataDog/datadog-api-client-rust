// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Google Chat delegated user attributes.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GoogleChatDelegatedUserAttributes {
    /// The delegated user's display name.
    #[serde(rename = "display_name")]
    pub display_name: Option<String>,
    /// The delegated user's email address.
    #[serde(rename = "email")]
    pub email: Option<String>,
    /// The list of features enabled for the delegated user.
    #[serde(rename = "features")]
    pub features: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GoogleChatDelegatedUserAttributes {
    pub fn new() -> GoogleChatDelegatedUserAttributes {
        GoogleChatDelegatedUserAttributes {
            display_name: None,
            email: None,
            features: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn display_name(mut self, value: String) -> Self {
        self.display_name = Some(value);
        self
    }

    pub fn email(mut self, value: String) -> Self {
        self.email = Some(value);
        self
    }

    pub fn features(mut self, value: Vec<String>) -> Self {
        self.features = Some(value);
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

impl Default for GoogleChatDelegatedUserAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for GoogleChatDelegatedUserAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GoogleChatDelegatedUserAttributesVisitor;
        impl<'a> Visitor<'a> for GoogleChatDelegatedUserAttributesVisitor {
            type Value = GoogleChatDelegatedUserAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut display_name: Option<String> = None;
                let mut email: Option<String> = None;
                let mut features: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "display_name" => {
                            if v.is_null() {
                                continue;
                            }
                            display_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "email" => {
                            if v.is_null() {
                                continue;
                            }
                            email = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "features" => {
                            if v.is_null() {
                                continue;
                            }
                            features = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = GoogleChatDelegatedUserAttributes {
                    display_name,
                    email,
                    features,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GoogleChatDelegatedUserAttributesVisitor)
    }
}
