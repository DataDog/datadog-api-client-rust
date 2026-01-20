// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Metadata about the entity from cloud providers
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityEntityMetadata {
    /// Cloud account ID (AWS)
    #[serde(rename = "accountID")]
    pub account_id: Option<String>,
    /// Environment tags associated with the entity
    #[serde(rename = "environments")]
    pub environments: Vec<String>,
    /// MITRE ATT&CK tactics detected
    #[serde(rename = "mitreTactics")]
    pub mitre_tactics: Vec<String>,
    /// MITRE ATT&CK techniques detected
    #[serde(rename = "mitreTechniques")]
    pub mitre_techniques: Vec<String>,
    /// Cloud project ID (GCP)
    #[serde(rename = "projectID")]
    pub project_id: Option<String>,
    /// Services associated with the entity
    #[serde(rename = "services")]
    pub services: Vec<String>,
    /// Data sources that detected this entity
    #[serde(rename = "sources")]
    pub sources: Vec<String>,
    /// Cloud subscription ID (Azure)
    #[serde(rename = "subscriptionID")]
    pub subscription_id: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityEntityMetadata {
    pub fn new(
        environments: Vec<String>,
        mitre_tactics: Vec<String>,
        mitre_techniques: Vec<String>,
        services: Vec<String>,
        sources: Vec<String>,
    ) -> SecurityEntityMetadata {
        SecurityEntityMetadata {
            account_id: None,
            environments,
            mitre_tactics,
            mitre_techniques,
            project_id: None,
            services,
            sources,
            subscription_id: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn account_id(mut self, value: String) -> Self {
        self.account_id = Some(value);
        self
    }

    pub fn project_id(mut self, value: String) -> Self {
        self.project_id = Some(value);
        self
    }

    pub fn subscription_id(mut self, value: String) -> Self {
        self.subscription_id = Some(value);
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

impl<'de> Deserialize<'de> for SecurityEntityMetadata {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityEntityMetadataVisitor;
        impl<'a> Visitor<'a> for SecurityEntityMetadataVisitor {
            type Value = SecurityEntityMetadata;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account_id: Option<String> = None;
                let mut environments: Option<Vec<String>> = None;
                let mut mitre_tactics: Option<Vec<String>> = None;
                let mut mitre_techniques: Option<Vec<String>> = None;
                let mut project_id: Option<String> = None;
                let mut services: Option<Vec<String>> = None;
                let mut sources: Option<Vec<String>> = None;
                let mut subscription_id: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "accountID" => {
                            if v.is_null() {
                                continue;
                            }
                            account_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "environments" => {
                            environments =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mitreTactics" => {
                            mitre_tactics =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mitreTechniques" => {
                            mitre_techniques =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "projectID" => {
                            if v.is_null() {
                                continue;
                            }
                            project_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "services" => {
                            services = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sources" => {
                            sources = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "subscriptionID" => {
                            if v.is_null() {
                                continue;
                            }
                            subscription_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let environments =
                    environments.ok_or_else(|| M::Error::missing_field("environments"))?;
                let mitre_tactics =
                    mitre_tactics.ok_or_else(|| M::Error::missing_field("mitre_tactics"))?;
                let mitre_techniques =
                    mitre_techniques.ok_or_else(|| M::Error::missing_field("mitre_techniques"))?;
                let services = services.ok_or_else(|| M::Error::missing_field("services"))?;
                let sources = sources.ok_or_else(|| M::Error::missing_field("sources"))?;

                let content = SecurityEntityMetadata {
                    account_id,
                    environments,
                    mitre_tactics,
                    mitre_techniques,
                    project_id,
                    services,
                    sources,
                    subscription_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityEntityMetadataVisitor)
    }
}
