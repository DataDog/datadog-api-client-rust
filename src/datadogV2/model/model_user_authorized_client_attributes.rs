// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a user authorized client.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UserAuthorizedClientAttributes {
    /// The date and time this authorization was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Whether the user has disabled this authorization.
    #[serde(rename = "disabled")]
    pub disabled: bool,
    /// The date and time this authorization was last exercised.
    #[serialize_always]
    #[serde(rename = "last_exercised")]
    pub last_exercised: Option<chrono::DateTime<chrono::Utc>>,
    /// The date and time this authorization was last modified.
    #[serde(rename = "modified_at")]
    pub modified_at: chrono::DateTime<chrono::Utc>,
    /// Whether the organization has disabled this authorization.
    #[serde(rename = "org_disabled")]
    pub org_disabled: bool,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UserAuthorizedClientAttributes {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        disabled: bool,
        last_exercised: Option<chrono::DateTime<chrono::Utc>>,
        modified_at: chrono::DateTime<chrono::Utc>,
        org_disabled: bool,
    ) -> UserAuthorizedClientAttributes {
        UserAuthorizedClientAttributes {
            created_at,
            disabled,
            last_exercised,
            modified_at,
            org_disabled,
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

impl<'de> Deserialize<'de> for UserAuthorizedClientAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UserAuthorizedClientAttributesVisitor;
        impl<'a> Visitor<'a> for UserAuthorizedClientAttributesVisitor {
            type Value = UserAuthorizedClientAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut disabled: Option<bool> = None;
                let mut last_exercised: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut modified_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut org_disabled: Option<bool> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "disabled" => {
                            disabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_exercised" => {
                            last_exercised =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_at" => {
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "org_disabled" => {
                            org_disabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let disabled = disabled.ok_or_else(|| M::Error::missing_field("disabled"))?;
                let last_exercised =
                    last_exercised.ok_or_else(|| M::Error::missing_field("last_exercised"))?;
                let modified_at =
                    modified_at.ok_or_else(|| M::Error::missing_field("modified_at"))?;
                let org_disabled =
                    org_disabled.ok_or_else(|| M::Error::missing_field("org_disabled"))?;

                let content = UserAuthorizedClientAttributes {
                    created_at,
                    disabled,
                    last_exercised,
                    modified_at,
                    org_disabled,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UserAuthorizedClientAttributesVisitor)
    }
}
