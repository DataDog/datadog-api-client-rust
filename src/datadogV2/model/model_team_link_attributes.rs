// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Team link attributes
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TeamLinkAttributes {
    /// The link's label
    #[serde(rename = "label")]
    pub label: String,
    /// The link's position, used to sort links for the team
    #[serde(rename = "position")]
    pub position: Option<i32>,
    /// ID of the team the link is associated with
    #[serde(rename = "team_id")]
    pub team_id: Option<String>,
    /// The URL for the link
    #[serde(rename = "url")]
    pub url: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TeamLinkAttributes {
    pub fn new(label: String, url: String) -> TeamLinkAttributes {
        TeamLinkAttributes {
            label,
            position: None,
            team_id: None,
            url,
            _unparsed: false,
        }
    }

    pub fn position(mut self, value: i32) -> Self {
        self.position = Some(value);
        self
    }

    pub fn team_id(mut self, value: String) -> Self {
        self.team_id = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for TeamLinkAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TeamLinkAttributesVisitor;
        impl<'a> Visitor<'a> for TeamLinkAttributesVisitor {
            type Value = TeamLinkAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut label: Option<String> = None;
                let mut position: Option<i32> = None;
                let mut team_id: Option<String> = None;
                let mut url: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "label" => {
                            label = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "position" => {
                            if v.is_null() {
                                continue;
                            }
                            position = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "team_id" => {
                            if v.is_null() {
                                continue;
                            }
                            team_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "url" => {
                            url = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let label = label.ok_or_else(|| M::Error::missing_field("label"))?;
                let url = url.ok_or_else(|| M::Error::missing_field("url"))?;

                let content = TeamLinkAttributes {
                    label,
                    position,
                    team_id,
                    url,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TeamLinkAttributesVisitor)
    }
}
