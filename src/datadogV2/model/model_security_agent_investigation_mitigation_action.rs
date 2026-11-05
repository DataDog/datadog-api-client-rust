// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An action that applies a mitigation.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityAgentInvestigationMitigationAction {
    /// Whether the action has run.
    #[serde(rename = "applied")]
    pub applied: bool,
    /// When the action ran.
    #[serde(rename = "applied_at")]
    pub applied_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Who ran the action.
    #[serde(rename = "applied_by")]
    pub applied_by: Option<String>,
    /// ID of the action in the Action Catalog.
    #[serde(rename = "fqn")]
    pub fqn: Option<String>,
    /// ID of the action.
    #[serde(rename = "id")]
    pub id: String,
    /// Name of the action in the Action Catalog.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The resource the action changes.
    #[serde(rename = "resource")]
    pub resource: Option<String>,
    /// Integration that runs the action, such as `dd`, `aws`, or `okta`.
    #[serde(rename = "source")]
    pub source: String,
    /// Whether the organization has a connection that can run the action.
    #[serde(rename = "source_connected")]
    pub source_connected: bool,
    /// What the action targets.
    #[serde(rename = "target")]
    pub target: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityAgentInvestigationMitigationAction {
    pub fn new(
        applied: bool,
        id: String,
        source: String,
        source_connected: bool,
    ) -> SecurityAgentInvestigationMitigationAction {
        SecurityAgentInvestigationMitigationAction {
            applied,
            applied_at: None,
            applied_by: None,
            fqn: None,
            id,
            name: None,
            resource: None,
            source,
            source_connected,
            target: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn applied_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.applied_at = Some(value);
        self
    }

    pub fn applied_by(mut self, value: String) -> Self {
        self.applied_by = Some(value);
        self
    }

    pub fn fqn(mut self, value: String) -> Self {
        self.fqn = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn resource(mut self, value: String) -> Self {
        self.resource = Some(value);
        self
    }

    pub fn target(mut self, value: String) -> Self {
        self.target = Some(value);
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

impl<'de> Deserialize<'de> for SecurityAgentInvestigationMitigationAction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityAgentInvestigationMitigationActionVisitor;
        impl<'a> Visitor<'a> for SecurityAgentInvestigationMitigationActionVisitor {
            type Value = SecurityAgentInvestigationMitigationAction;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut applied: Option<bool> = None;
                let mut applied_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut applied_by: Option<String> = None;
                let mut fqn: Option<String> = None;
                let mut id: Option<String> = None;
                let mut name: Option<String> = None;
                let mut resource: Option<String> = None;
                let mut source: Option<String> = None;
                let mut source_connected: Option<bool> = None;
                let mut target: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "applied" => {
                            applied = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "applied_at" => {
                            if v.is_null() {
                                continue;
                            }
                            applied_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "applied_by" => {
                            if v.is_null() {
                                continue;
                            }
                            applied_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "fqn" => {
                            if v.is_null() {
                                continue;
                            }
                            fqn = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resource" => {
                            if v.is_null() {
                                continue;
                            }
                            resource = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source" => {
                            source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source_connected" => {
                            source_connected =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "target" => {
                            if v.is_null() {
                                continue;
                            }
                            target = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let applied = applied.ok_or_else(|| M::Error::missing_field("applied"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let source = source.ok_or_else(|| M::Error::missing_field("source"))?;
                let source_connected =
                    source_connected.ok_or_else(|| M::Error::missing_field("source_connected"))?;

                let content = SecurityAgentInvestigationMitigationAction {
                    applied,
                    applied_at,
                    applied_by,
                    fqn,
                    id,
                    name,
                    resource,
                    source,
                    source_connected,
                    target,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityAgentInvestigationMitigationActionVisitor)
    }
}
