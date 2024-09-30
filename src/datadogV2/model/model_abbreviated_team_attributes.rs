// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `AbbreviatedTeamAttributes` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AbbreviatedTeamAttributes {
    /// Unicode representation of the avatar for the team, limited to a single grapheme
    #[serde(rename = "avatar", default, with = "::serde_with::rust::double_option")]
    pub avatar: Option<Option<String>>,
    /// Banner selection for the team
    #[serde(rename = "banner")]
    pub banner: Option<i64>,
    /// The team's identifier
    #[serde(rename = "handle")]
    pub handle: String,
    /// The `AbbreviatedTeamAttributes` `handles`.
    #[serde(rename = "handles")]
    pub handles: Option<String>,
    /// The `AbbreviatedTeamAttributes` `is_open_membership`.
    #[serde(rename = "is_open_membership")]
    pub is_open_membership: Option<bool>,
    /// The name of the team
    #[serde(rename = "name")]
    pub name: String,
    /// A brief summary of the team
    #[serde(rename = "summary")]
    pub summary: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AbbreviatedTeamAttributes {
    pub fn new(handle: String, name: String) -> AbbreviatedTeamAttributes {
        AbbreviatedTeamAttributes {
            avatar: None,
            banner: None,
            handle,
            handles: None,
            is_open_membership: None,
            name,
            summary: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn avatar(mut self, value: Option<String>) -> Self {
        self.avatar = Some(value);
        self
    }

    pub fn banner(mut self, value: i64) -> Self {
        self.banner = Some(value);
        self
    }

    pub fn handles(mut self, value: String) -> Self {
        self.handles = Some(value);
        self
    }

    pub fn is_open_membership(mut self, value: bool) -> Self {
        self.is_open_membership = Some(value);
        self
    }

    pub fn summary(mut self, value: String) -> Self {
        self.summary = Some(value);
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

impl<'de> Deserialize<'de> for AbbreviatedTeamAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AbbreviatedTeamAttributesVisitor;
        impl<'a> Visitor<'a> for AbbreviatedTeamAttributesVisitor {
            type Value = AbbreviatedTeamAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut avatar: Option<Option<String>> = None;
                let mut banner: Option<i64> = None;
                let mut handle: Option<String> = None;
                let mut handles: Option<String> = None;
                let mut is_open_membership: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut summary: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "avatar" => {
                            avatar = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "banner" => {
                            if v.is_null() {
                                continue;
                            }
                            banner = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "handle" => {
                            handle = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "handles" => {
                            if v.is_null() {
                                continue;
                            }
                            handles = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_open_membership" => {
                            if v.is_null() {
                                continue;
                            }
                            is_open_membership =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "summary" => {
                            if v.is_null() {
                                continue;
                            }
                            summary = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let handle = handle.ok_or_else(|| M::Error::missing_field("handle"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = AbbreviatedTeamAttributes {
                    avatar,
                    banner,
                    handle,
                    handles,
                    is_open_membership,
                    name,
                    summary,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AbbreviatedTeamAttributesVisitor)
    }
}
