// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Daily AI coding tool activity for a single user. Each entry reports whether the user was
/// active on a given day and which AI tools and models they used.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AIImpactUserActivityAttributes {
    /// The day the activity refers to, in `YYYY-MM-DD` format.
    #[serde(rename = "day")]
    pub day: String,
    /// Whether the user actively used the listed AI tools on that day.
    #[serde(rename = "is_active")]
    pub is_active: bool,
    /// The AI models the user used on that day, for example `claude-sonnet-4.5` or `gpt-5`.
    /// Values are lowercased and duplicates are removed.
    #[serde(rename = "models")]
    pub models: Option<Vec<String>>,
    /// The AI coding tools the user used on that day, for example `Claude Code`, `Cursor`, or
    /// `GitHub Copilot`. Known tools are normalized to a canonical name (`claude_code`, `cursor`,
    /// `copilot`), and other values are converted to snake case. Entries must not be empty.
    #[serde(rename = "tools")]
    pub tools: Vec<String>,
    /// The email address of the user. It is case-insensitive and is matched against the
    /// email addresses of commit authors.
    #[serde(rename = "user_email")]
    pub user_email: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AIImpactUserActivityAttributes {
    pub fn new(
        day: String,
        is_active: bool,
        tools: Vec<String>,
        user_email: String,
    ) -> AIImpactUserActivityAttributes {
        AIImpactUserActivityAttributes {
            day,
            is_active,
            models: None,
            tools,
            user_email,
            _unparsed: false,
        }
    }

    pub fn models(mut self, value: Vec<String>) -> Self {
        self.models = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for AIImpactUserActivityAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AIImpactUserActivityAttributesVisitor;
        impl<'a> Visitor<'a> for AIImpactUserActivityAttributesVisitor {
            type Value = AIImpactUserActivityAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut day: Option<String> = None;
                let mut is_active: Option<bool> = None;
                let mut models: Option<Vec<String>> = None;
                let mut tools: Option<Vec<String>> = None;
                let mut user_email: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "day" => {
                            day = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_active" => {
                            is_active = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "models" => {
                            if v.is_null() {
                                continue;
                            }
                            models = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tools" => {
                            tools = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "user_email" => {
                            user_email = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let day = day.ok_or_else(|| M::Error::missing_field("day"))?;
                let is_active = is_active.ok_or_else(|| M::Error::missing_field("is_active"))?;
                let tools = tools.ok_or_else(|| M::Error::missing_field("tools"))?;
                let user_email = user_email.ok_or_else(|| M::Error::missing_field("user_email"))?;

                let content = AIImpactUserActivityAttributes {
                    day,
                    is_active,
                    models,
                    tools,
                    user_email,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AIImpactUserActivityAttributesVisitor)
    }
}
