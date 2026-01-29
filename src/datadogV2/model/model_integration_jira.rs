// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Jira integration settings
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IntegrationJira {
    #[serde(rename = "auto_creation")]
    pub auto_creation: Option<crate::datadogV2::model::IntegrationJiraAutoCreation>,
    /// Whether Jira integration is enabled
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    #[serde(rename = "metadata")]
    pub metadata: Option<crate::datadogV2::model::IntegrationJiraMetadata>,
    #[serde(rename = "sync")]
    pub sync: Option<crate::datadogV2::model::IntegrationJiraSync>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IntegrationJira {
    pub fn new() -> IntegrationJira {
        IntegrationJira {
            auto_creation: None,
            enabled: None,
            metadata: None,
            sync: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn auto_creation(
        mut self,
        value: crate::datadogV2::model::IntegrationJiraAutoCreation,
    ) -> Self {
        self.auto_creation = Some(value);
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn metadata(mut self, value: crate::datadogV2::model::IntegrationJiraMetadata) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn sync(mut self, value: crate::datadogV2::model::IntegrationJiraSync) -> Self {
        self.sync = Some(value);
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

impl Default for IntegrationJira {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IntegrationJira {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IntegrationJiraVisitor;
        impl<'a> Visitor<'a> for IntegrationJiraVisitor {
            type Value = IntegrationJira;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut auto_creation: Option<
                    crate::datadogV2::model::IntegrationJiraAutoCreation,
                > = None;
                let mut enabled: Option<bool> = None;
                let mut metadata: Option<crate::datadogV2::model::IntegrationJiraMetadata> = None;
                let mut sync: Option<crate::datadogV2::model::IntegrationJiraSync> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "auto_creation" => {
                            if v.is_null() {
                                continue;
                            }
                            auto_creation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metadata" => {
                            if v.is_null() {
                                continue;
                            }
                            metadata = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sync" => {
                            if v.is_null() {
                                continue;
                            }
                            sync = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = IntegrationJira {
                    auto_creation,
                    enabled,
                    metadata,
                    sync,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IntegrationJiraVisitor)
    }
}
