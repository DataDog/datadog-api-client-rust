// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of the organization.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OrganizationAttributes {
    /// Creation time of the organization.
    #[serde(rename = "created_at")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Description of the organization.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Whether or not the organization is disabled.
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
    /// Time of last organization modification.
    #[serde(rename = "modified_at")]
    pub modified_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Name of the organization.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Public ID of the organization.
    #[serde(rename = "public_id")]
    pub public_id: Option<String>,
    /// Sharing type of the organization.
    #[serde(rename = "sharing")]
    pub sharing: Option<String>,
    /// URL of the site that this organization exists at.
    #[serde(rename = "url")]
    pub url: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OrganizationAttributes {
    pub fn new() -> OrganizationAttributes {
        OrganizationAttributes {
            created_at: None,
            description: None,
            disabled: None,
            modified_at: None,
            name: None,
            public_id: None,
            sharing: None,
            url: None,
            _unparsed: false,
        }
    }

    pub fn created_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn disabled(mut self, value: bool) -> Self {
        self.disabled = Some(value);
        self
    }

    pub fn modified_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.modified_at = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn public_id(mut self, value: String) -> Self {
        self.public_id = Some(value);
        self
    }

    pub fn sharing(mut self, value: String) -> Self {
        self.sharing = Some(value);
        self
    }

    pub fn url(mut self, value: String) -> Self {
        self.url = Some(value);
        self
    }
}

impl Default for OrganizationAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for OrganizationAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OrganizationAttributesVisitor;
        impl<'a> Visitor<'a> for OrganizationAttributesVisitor {
            type Value = OrganizationAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut description: Option<String> = None;
                let mut disabled: Option<bool> = None;
                let mut modified_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut name: Option<String> = None;
                let mut public_id: Option<String> = None;
                let mut sharing: Option<String> = None;
                let mut url: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "disabled" => {
                            if v.is_null() {
                                continue;
                            }
                            disabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_at" => {
                            if v.is_null() {
                                continue;
                            }
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "public_id" => {
                            if v.is_null() {
                                continue;
                            }
                            public_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sharing" => {
                            if v.is_null() {
                                continue;
                            }
                            sharing = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "url" => {
                            if v.is_null() {
                                continue;
                            }
                            url = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = OrganizationAttributes {
                    created_at,
                    description,
                    disabled,
                    modified_at,
                    name,
                    public_id,
                    sharing,
                    url,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OrganizationAttributesVisitor)
    }
}
