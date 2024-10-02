// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Channel attributes.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MicrosoftTeamsChannelInfoResponseAttributes {
    /// Indicates if this is the primary channel.
    #[serde(rename = "is_primary")]
    pub is_primary: Option<bool>,
    /// Team id.
    #[serde(rename = "team_id")]
    pub team_id: Option<String>,
    /// Tenant id.
    #[serde(rename = "tenant_id")]
    pub tenant_id: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MicrosoftTeamsChannelInfoResponseAttributes {
    pub fn new() -> MicrosoftTeamsChannelInfoResponseAttributes {
        MicrosoftTeamsChannelInfoResponseAttributes {
            is_primary: None,
            team_id: None,
            tenant_id: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn is_primary(mut self, value: bool) -> Self {
        self.is_primary = Some(value);
        self
    }

    pub fn team_id(mut self, value: String) -> Self {
        self.team_id = Some(value);
        self
    }

    pub fn tenant_id(mut self, value: String) -> Self {
        self.tenant_id = Some(value);
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

impl Default for MicrosoftTeamsChannelInfoResponseAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for MicrosoftTeamsChannelInfoResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MicrosoftTeamsChannelInfoResponseAttributesVisitor;
        impl<'a> Visitor<'a> for MicrosoftTeamsChannelInfoResponseAttributesVisitor {
            type Value = MicrosoftTeamsChannelInfoResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut is_primary: Option<bool> = None;
                let mut team_id: Option<String> = None;
                let mut tenant_id: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "is_primary" => {
                            if v.is_null() {
                                continue;
                            }
                            is_primary = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "team_id" => {
                            if v.is_null() {
                                continue;
                            }
                            team_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tenant_id" => {
                            if v.is_null() {
                                continue;
                            }
                            tenant_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = MicrosoftTeamsChannelInfoResponseAttributes {
                    is_primary,
                    team_id,
                    tenant_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MicrosoftTeamsChannelInfoResponseAttributesVisitor)
    }
}
