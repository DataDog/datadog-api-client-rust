// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Asset risks.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AssetRisks {
    /// Whether the asset has access to sensitive data or not.
    #[serde(rename = "has_access_to_sensitive_data")]
    pub has_access_to_sensitive_data: Option<bool>,
    /// Whether the asset has privileged access or not.
    #[serde(rename = "has_privileged_access")]
    pub has_privileged_access: Option<bool>,
    /// Whether the asset is in production or not.
    #[serde(rename = "in_production")]
    pub in_production: bool,
    /// Whether the asset is publicly accessible or not.
    #[serde(rename = "is_publicly_accessible")]
    pub is_publicly_accessible: Option<bool>,
    /// Whether the asset is under attack or not.
    #[serde(rename = "under_attack")]
    pub under_attack: Option<bool>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AssetRisks {
    pub fn new(in_production: bool) -> AssetRisks {
        AssetRisks {
            has_access_to_sensitive_data: None,
            has_privileged_access: None,
            in_production,
            is_publicly_accessible: None,
            under_attack: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn has_access_to_sensitive_data(mut self, value: bool) -> Self {
        self.has_access_to_sensitive_data = Some(value);
        self
    }

    pub fn has_privileged_access(mut self, value: bool) -> Self {
        self.has_privileged_access = Some(value);
        self
    }

    pub fn is_publicly_accessible(mut self, value: bool) -> Self {
        self.is_publicly_accessible = Some(value);
        self
    }

    pub fn under_attack(mut self, value: bool) -> Self {
        self.under_attack = Some(value);
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

impl<'de> Deserialize<'de> for AssetRisks {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AssetRisksVisitor;
        impl<'a> Visitor<'a> for AssetRisksVisitor {
            type Value = AssetRisks;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut has_access_to_sensitive_data: Option<bool> = None;
                let mut has_privileged_access: Option<bool> = None;
                let mut in_production: Option<bool> = None;
                let mut is_publicly_accessible: Option<bool> = None;
                let mut under_attack: Option<bool> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "has_access_to_sensitive_data" => {
                            if v.is_null() {
                                continue;
                            }
                            has_access_to_sensitive_data =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "has_privileged_access" => {
                            if v.is_null() {
                                continue;
                            }
                            has_privileged_access =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "in_production" => {
                            in_production =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_publicly_accessible" => {
                            if v.is_null() {
                                continue;
                            }
                            is_publicly_accessible =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "under_attack" => {
                            if v.is_null() {
                                continue;
                            }
                            under_attack =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let in_production =
                    in_production.ok_or_else(|| M::Error::missing_field("in_production"))?;

                let content = AssetRisks {
                    has_access_to_sensitive_data,
                    has_privileged_access,
                    in_production,
                    is_publicly_accessible,
                    under_attack,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AssetRisksVisitor)
    }
}
