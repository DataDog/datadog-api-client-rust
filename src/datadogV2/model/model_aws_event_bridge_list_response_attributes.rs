// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An object describing the EventBridge configuration for multiple accounts.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AWSEventBridgeListResponseAttributes {
    /// List of accounts with their event sources.
    #[serde(rename = "accounts")]
    pub accounts: Option<Vec<crate::datadogV2::model::AWSEventBridgeAccountConfiguration>>,
    /// True if the EventBridge integration is enabled for your organization.
    #[serde(rename = "is_installed")]
    pub is_installed: Option<bool>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AWSEventBridgeListResponseAttributes {
    pub fn new() -> AWSEventBridgeListResponseAttributes {
        AWSEventBridgeListResponseAttributes {
            accounts: None,
            is_installed: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn accounts(
        mut self,
        value: Vec<crate::datadogV2::model::AWSEventBridgeAccountConfiguration>,
    ) -> Self {
        self.accounts = Some(value);
        self
    }

    pub fn is_installed(mut self, value: bool) -> Self {
        self.is_installed = Some(value);
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

impl Default for AWSEventBridgeListResponseAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AWSEventBridgeListResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AWSEventBridgeListResponseAttributesVisitor;
        impl<'a> Visitor<'a> for AWSEventBridgeListResponseAttributesVisitor {
            type Value = AWSEventBridgeListResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut accounts: Option<
                    Vec<crate::datadogV2::model::AWSEventBridgeAccountConfiguration>,
                > = None;
                let mut is_installed: Option<bool> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "accounts" => {
                            if v.is_null() {
                                continue;
                            }
                            accounts = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_installed" => {
                            if v.is_null() {
                                continue;
                            }
                            is_installed =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = AWSEventBridgeListResponseAttributes {
                    accounts,
                    is_installed,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AWSEventBridgeListResponseAttributesVisitor)
    }
}
