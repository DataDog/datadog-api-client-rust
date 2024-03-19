// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Incident integration metadata for the Slack integration.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SlackIntegrationMetadata {
    /// Array of Slack channels in this integration metadata.
    #[serde(rename = "channels")]
    pub channels: Vec<crate::datadogV2::model::SlackIntegrationMetadataChannelItem>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SlackIntegrationMetadata {
    pub fn new(
        channels: Vec<crate::datadogV2::model::SlackIntegrationMetadataChannelItem>,
    ) -> SlackIntegrationMetadata {
        SlackIntegrationMetadata {
            channels,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for SlackIntegrationMetadata {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SlackIntegrationMetadataVisitor;
        impl<'a> Visitor<'a> for SlackIntegrationMetadataVisitor {
            type Value = SlackIntegrationMetadata;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut channels: Option<
                    Vec<crate::datadogV2::model::SlackIntegrationMetadataChannelItem>,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "channels" => {
                            channels = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let channels = channels.ok_or_else(|| M::Error::missing_field("channels"))?;

                let content = SlackIntegrationMetadata {
                    channels,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SlackIntegrationMetadataVisitor)
    }
}
