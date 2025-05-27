// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Sends a message to a Slack channel.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SendSlackMessageAction {
    /// The channel ID.
    #[serde(rename = "channel")]
    pub channel: String,
    /// Indicates that the action is a send Slack message action.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::SendSlackMessageActionType,
    /// The workspace ID.
    #[serde(rename = "workspace")]
    pub workspace: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SendSlackMessageAction {
    pub fn new(
        channel: String,
        type_: crate::datadogV2::model::SendSlackMessageActionType,
        workspace: String,
    ) -> SendSlackMessageAction {
        SendSlackMessageAction {
            channel,
            type_,
            workspace,
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

impl<'de> Deserialize<'de> for SendSlackMessageAction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SendSlackMessageActionVisitor;
        impl<'a> Visitor<'a> for SendSlackMessageActionVisitor {
            type Value = SendSlackMessageAction;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut channel: Option<String> = None;
                let mut type_: Option<crate::datadogV2::model::SendSlackMessageActionType> = None;
                let mut workspace: Option<String> = None;
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
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::SendSlackMessageActionType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "workspace" => {
                            workspace = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let channel = channel.ok_or_else(|| M::Error::missing_field("channel"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;
                let workspace = workspace.ok_or_else(|| M::Error::missing_field("workspace"))?;

                let content = SendSlackMessageAction {
                    channel,
                    type_,
                    workspace,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SendSlackMessageActionVisitor)
    }
}
