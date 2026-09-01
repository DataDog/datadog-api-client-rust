// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Configuration for a Slack emoji reaction trigger.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SlackReactionConfig {
    /// The Slack emoji reaction name.
    #[serde(rename = "reactionEmoji")]
    pub reaction_emoji: String,
    /// The Slack workspace ID.
    #[serde(rename = "teamId")]
    pub team_id: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SlackReactionConfig {
    pub fn new(reaction_emoji: String, team_id: String) -> SlackReactionConfig {
        SlackReactionConfig {
            reaction_emoji,
            team_id,
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

impl<'de> Deserialize<'de> for SlackReactionConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SlackReactionConfigVisitor;
        impl<'a> Visitor<'a> for SlackReactionConfigVisitor {
            type Value = SlackReactionConfig;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut reaction_emoji: Option<String> = None;
                let mut team_id: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "reactionEmoji" => {
                            reaction_emoji =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "teamId" => {
                            team_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let reaction_emoji =
                    reaction_emoji.ok_or_else(|| M::Error::missing_field("reaction_emoji"))?;
                let team_id = team_id.ok_or_else(|| M::Error::missing_field("team_id"))?;

                let content = SlackReactionConfig {
                    reaction_emoji,
                    team_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SlackReactionConfigVisitor)
    }
}
