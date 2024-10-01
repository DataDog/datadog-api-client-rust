// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Handle attributes.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MicrosoftTeamsApiHandleRequestAttributes {
    /// Channel id.
    #[serde(rename = "channel_id")]
    pub channel_id: String,
    /// Handle name.
    #[serde(rename = "name")]
    pub name: String,
    /// Team id.
    #[serde(rename = "team_id")]
    pub team_id: String,
    /// Tenant id.
    #[serde(rename = "tenant_id")]
    pub tenant_id: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MicrosoftTeamsApiHandleRequestAttributes {
    pub fn new(
        channel_id: String,
        name: String,
        team_id: String,
        tenant_id: String,
    ) -> MicrosoftTeamsApiHandleRequestAttributes {
        MicrosoftTeamsApiHandleRequestAttributes {
            channel_id,
            name,
            team_id,
            tenant_id,
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

impl<'de> Deserialize<'de> for MicrosoftTeamsApiHandleRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MicrosoftTeamsApiHandleRequestAttributesVisitor;
        impl<'a> Visitor<'a> for MicrosoftTeamsApiHandleRequestAttributesVisitor {
            type Value = MicrosoftTeamsApiHandleRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut channel_id: Option<String> = None;
                let mut name: Option<String> = None;
                let mut team_id: Option<String> = None;
                let mut tenant_id: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "channel_id" => {
                            channel_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "team_id" => {
                            team_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tenant_id" => {
                            tenant_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let channel_id = channel_id.ok_or_else(|| M::Error::missing_field("channel_id"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let team_id = team_id.ok_or_else(|| M::Error::missing_field("team_id"))?;
                let tenant_id = tenant_id.ok_or_else(|| M::Error::missing_field("tenant_id"))?;

                let content = MicrosoftTeamsApiHandleRequestAttributes {
                    channel_id,
                    name,
                    team_id,
                    tenant_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MicrosoftTeamsApiHandleRequestAttributesVisitor)
    }
}
