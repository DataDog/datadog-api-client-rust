// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Item in the Slack integration metadata channel array.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SlackIntegrationMetadataChannelItem {
    /// Slack channel ID.
    #[serde(rename = "channel_id")]
    pub channel_id: String,
    /// Name of the Slack channel.
    #[serde(rename = "channel_name")]
    pub channel_name: String,
    /// URL redirecting to the Slack channel.
    #[serde(rename = "redirect_url")]
    pub redirect_url: String,
    /// Slack team ID.
    #[serde(rename = "team_id")]
    pub team_id: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SlackIntegrationMetadataChannelItem {
    pub fn new(
        channel_id: String,
        channel_name: String,
        redirect_url: String,
    ) -> SlackIntegrationMetadataChannelItem {
        SlackIntegrationMetadataChannelItem {
            channel_id,
            channel_name,
            redirect_url,
            team_id: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn team_id(mut self, value: String) -> Self {
        self.team_id = Some(value);
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

impl<'de> Deserialize<'de> for SlackIntegrationMetadataChannelItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SlackIntegrationMetadataChannelItemVisitor;
        impl<'a> Visitor<'a> for SlackIntegrationMetadataChannelItemVisitor {
            type Value = SlackIntegrationMetadataChannelItem;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut channel_id: Option<String> = None;
                let mut channel_name: Option<String> = None;
                let mut redirect_url: Option<String> = None;
                let mut team_id: Option<String> = None;
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
                        "channel_name" => {
                            channel_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "redirect_url" => {
                            redirect_url =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "team_id" => {
                            if v.is_null() {
                                continue;
                            }
                            team_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let channel_id = channel_id.ok_or_else(|| M::Error::missing_field("channel_id"))?;
                let channel_name =
                    channel_name.ok_or_else(|| M::Error::missing_field("channel_name"))?;
                let redirect_url =
                    redirect_url.ok_or_else(|| M::Error::missing_field("redirect_url"))?;

                let content = SlackIntegrationMetadataChannelItem {
                    channel_id,
                    channel_name,
                    redirect_url,
                    team_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SlackIntegrationMetadataChannelItemVisitor)
    }
}
