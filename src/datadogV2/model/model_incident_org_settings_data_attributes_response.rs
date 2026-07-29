// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of an incident org settings resource in a response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentOrgSettingsDataAttributesResponse {
    /// Timestamp when the settings were created.
    #[serde(rename = "created")]
    pub created: chrono::DateTime<chrono::Utc>,
    /// Timestamp when the settings were last modified.
    #[serde(rename = "modified")]
    pub modified: chrono::DateTime<chrono::Utc>,
    /// The settings configuration for an incident org settings resource.
    #[serde(rename = "settings")]
    pub settings: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentOrgSettingsDataAttributesResponse {
    pub fn new(
        created: chrono::DateTime<chrono::Utc>,
        modified: chrono::DateTime<chrono::Utc>,
        settings: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> IncidentOrgSettingsDataAttributesResponse {
        IncidentOrgSettingsDataAttributesResponse {
            created,
            modified,
            settings,
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

impl<'de> Deserialize<'de> for IncidentOrgSettingsDataAttributesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentOrgSettingsDataAttributesResponseVisitor;
        impl<'a> Visitor<'a> for IncidentOrgSettingsDataAttributesResponseVisitor {
            type Value = IncidentOrgSettingsDataAttributesResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut modified: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut settings: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created" => {
                            created = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified" => {
                            modified = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "settings" => {
                            settings = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let created = created.ok_or_else(|| M::Error::missing_field("created"))?;
                let modified = modified.ok_or_else(|| M::Error::missing_field("modified"))?;
                let settings = settings.ok_or_else(|| M::Error::missing_field("settings"))?;

                let content = IncidentOrgSettingsDataAttributesResponse {
                    created,
                    modified,
                    settings,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentOrgSettingsDataAttributesResponseVisitor)
    }
}
