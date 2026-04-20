// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of an organization.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OrgAttributes {
    /// The creation timestamp of the organization.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// A description of the organization.
    #[serde(rename = "description")]
    pub description: String,
    /// Whether the organization is disabled.
    #[serde(rename = "disabled")]
    pub disabled: bool,
    /// The last modification timestamp of the organization.
    #[serde(rename = "modified_at")]
    pub modified_at: chrono::DateTime<chrono::Utc>,
    /// The name of the organization.
    #[serde(rename = "name")]
    pub name: String,
    /// The public identifier of the organization.
    #[serde(rename = "public_id")]
    pub public_id: String,
    /// The sharing setting of the organization.
    #[serde(rename = "sharing")]
    pub sharing: String,
    /// The URL of the organization.
    #[serde(rename = "url")]
    pub url: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OrgAttributes {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        description: String,
        disabled: bool,
        modified_at: chrono::DateTime<chrono::Utc>,
        name: String,
        public_id: String,
        sharing: String,
        url: String,
    ) -> OrgAttributes {
        OrgAttributes {
            created_at,
            description,
            disabled,
            modified_at,
            name,
            public_id,
            sharing,
            url,
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

impl<'de> Deserialize<'de> for OrgAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OrgAttributesVisitor;
        impl<'a> Visitor<'a> for OrgAttributesVisitor {
            type Value = OrgAttributes;

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
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "disabled" => {
                            disabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_at" => {
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "public_id" => {
                            public_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sharing" => {
                            sharing = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "url" => {
                            url = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let disabled = disabled.ok_or_else(|| M::Error::missing_field("disabled"))?;
                let modified_at =
                    modified_at.ok_or_else(|| M::Error::missing_field("modified_at"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let public_id = public_id.ok_or_else(|| M::Error::missing_field("public_id"))?;
                let sharing = sharing.ok_or_else(|| M::Error::missing_field("sharing"))?;
                let url = url.ok_or_else(|| M::Error::missing_field("url"))?;

                let content = OrgAttributes {
                    created_at,
                    description,
                    disabled,
                    modified_at,
                    name,
                    public_id,
                    sharing,
                    url,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OrgAttributesVisitor)
    }
}
