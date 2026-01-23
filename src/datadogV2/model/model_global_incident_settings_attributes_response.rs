// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Global incident settings attributes
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GlobalIncidentSettingsAttributesResponse {
    /// The analytics dashboard ID
    #[serde(rename = "analytics_dashboard_id")]
    pub analytics_dashboard_id: String,
    /// Timestamp when the settings were created
    #[serde(rename = "created")]
    pub created: chrono::DateTime<chrono::Utc>,
    /// Timestamp when the settings were last modified
    #[serde(rename = "modified")]
    pub modified: chrono::DateTime<chrono::Utc>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GlobalIncidentSettingsAttributesResponse {
    pub fn new(
        analytics_dashboard_id: String,
        created: chrono::DateTime<chrono::Utc>,
        modified: chrono::DateTime<chrono::Utc>,
    ) -> GlobalIncidentSettingsAttributesResponse {
        GlobalIncidentSettingsAttributesResponse {
            analytics_dashboard_id,
            created,
            modified,
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

impl<'de> Deserialize<'de> for GlobalIncidentSettingsAttributesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GlobalIncidentSettingsAttributesResponseVisitor;
        impl<'a> Visitor<'a> for GlobalIncidentSettingsAttributesResponseVisitor {
            type Value = GlobalIncidentSettingsAttributesResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut analytics_dashboard_id: Option<String> = None;
                let mut created: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut modified: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "analytics_dashboard_id" => {
                            analytics_dashboard_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created" => {
                            created = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified" => {
                            modified = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let analytics_dashboard_id = analytics_dashboard_id
                    .ok_or_else(|| M::Error::missing_field("analytics_dashboard_id"))?;
                let created = created.ok_or_else(|| M::Error::missing_field("created"))?;
                let modified = modified.ok_or_else(|| M::Error::missing_field("modified"))?;

                let content = GlobalIncidentSettingsAttributesResponse {
                    analytics_dashboard_id,
                    created,
                    modified,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GlobalIncidentSettingsAttributesResponseVisitor)
    }
}
