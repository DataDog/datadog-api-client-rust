// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The Slack channel configuration.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SlackIntegrationChannel {
    /// Configuration options for what is shown in an alert event message.
    #[serde(rename = "display")]
    pub display: Option<crate::datadogV1::model::SlackIntegrationChannelDisplay>,
    /// Your channel name.
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SlackIntegrationChannel {
    pub fn new() -> SlackIntegrationChannel {
        SlackIntegrationChannel {
            display: None,
            name: None,
            _unparsed: false,
        }
    }

    pub fn display(
        &mut self,
        value: crate::datadogV1::model::SlackIntegrationChannelDisplay,
    ) -> &mut Self {
        self.display = Some(value);
        self
    }

    pub fn name(&mut self, value: String) -> &mut Self {
        self.name = Some(value);
        self
    }
}

impl Default for SlackIntegrationChannel {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SlackIntegrationChannel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SlackIntegrationChannelVisitor;
        impl<'a> Visitor<'a> for SlackIntegrationChannelVisitor {
            type Value = SlackIntegrationChannel;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut display: Option<crate::datadogV1::model::SlackIntegrationChannelDisplay> =
                    None;
                let mut name: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "display" => {
                            if v.is_null() {
                                continue;
                            }
                            display = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SlackIntegrationChannel {
                    display,
                    name,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SlackIntegrationChannelVisitor)
    }
}
