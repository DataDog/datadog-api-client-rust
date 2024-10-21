// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Incident integration metadata for the Microsoft Teams integration.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MSTeamsIntegrationMetadata {
    /// Array of Microsoft Teams in this integration metadata.
    #[serde(rename = "teams")]
    pub teams: Vec<crate::datadogV2::model::MSTeamsIntegrationMetadataTeamsItem>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MSTeamsIntegrationMetadata {
    pub fn new(
        teams: Vec<crate::datadogV2::model::MSTeamsIntegrationMetadataTeamsItem>,
    ) -> MSTeamsIntegrationMetadata {
        MSTeamsIntegrationMetadata {
            teams,
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

impl<'de> Deserialize<'de> for MSTeamsIntegrationMetadata {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MSTeamsIntegrationMetadataVisitor;
        impl<'a> Visitor<'a> for MSTeamsIntegrationMetadataVisitor {
            type Value = MSTeamsIntegrationMetadata;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut teams: Option<
                    Vec<crate::datadogV2::model::MSTeamsIntegrationMetadataTeamsItem>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "teams" => {
                            teams = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let teams = teams.ok_or_else(|| M::Error::missing_field("teams"))?;

                let content = MSTeamsIntegrationMetadata {
                    teams,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MSTeamsIntegrationMetadataVisitor)
    }
}
