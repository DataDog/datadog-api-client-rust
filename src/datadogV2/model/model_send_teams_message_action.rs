// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Sends a message to a Microsoft Teams channel.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SendTeamsMessageAction {
    /// The channel ID.
    #[serde(rename = "channel")]
    pub channel: String,
    /// The team ID.
    #[serde(rename = "team")]
    pub team: String,
    /// The tenant ID.
    #[serde(rename = "tenant")]
    pub tenant: String,
    /// Indicates that the action is a send Microsoft Teams message action.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::SendTeamsMessageActionType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SendTeamsMessageAction {
    pub fn new(
        channel: String,
        team: String,
        tenant: String,
        type_: crate::datadogV2::model::SendTeamsMessageActionType,
    ) -> SendTeamsMessageAction {
        SendTeamsMessageAction {
            channel,
            team,
            tenant,
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

impl<'de> Deserialize<'de> for SendTeamsMessageAction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SendTeamsMessageActionVisitor;
        impl<'a> Visitor<'a> for SendTeamsMessageActionVisitor {
            type Value = SendTeamsMessageAction;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut channel: Option<String> = None;
                let mut team: Option<String> = None;
                let mut tenant: Option<String> = None;
                let mut type_: Option<crate::datadogV2::model::SendTeamsMessageActionType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "channel" => {
                            channel = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "team" => {
                            team = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tenant" => {
                            tenant = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::SendTeamsMessageActionType::UnparsedObject(_type_) => {
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
                let channel = channel.ok_or_else(|| M::Error::missing_field("channel"))?;
                let team = team.ok_or_else(|| M::Error::missing_field("team"))?;
                let tenant = tenant.ok_or_else(|| M::Error::missing_field("tenant"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = SendTeamsMessageAction {
                    channel,
                    team,
                    tenant,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SendTeamsMessageActionVisitor)
    }
}
