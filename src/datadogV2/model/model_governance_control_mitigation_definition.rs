// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of a mitigation available for a control.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GovernanceControlMitigationDefinition {
    /// The verb describing the mitigation action, such as `revoke` or `delete`.
    #[serde(rename = "action_verb")]
    pub action_verb: String,
    /// A human-readable description of the mitigation.
    #[serde(rename = "description")]
    pub description: String,
    /// The execution modes the mitigation supports, such as `manual` or `automatic`.
    #[serde(rename = "execution_modes")]
    pub execution_modes: Option<Vec<String>>,
    /// The feature flags that gate the mitigation.
    #[serde(rename = "feature_flags")]
    pub feature_flags: Vec<String>,
    /// The unique identifier of the mitigation.
    #[serde(rename = "id")]
    pub id: String,
    /// A warning shown to the user before applying the mitigation manually.
    #[serde(rename = "manual_mitigation_warning")]
    pub manual_mitigation_warning: String,
    /// The permissions required to apply the mitigation.
    #[serde(rename = "permissions")]
    pub permissions: Vec<String>,
    /// Whether the mitigation requires AI to be enabled.
    #[serde(rename = "requires_ai")]
    pub requires_ai: bool,
    /// An array of parameter definitions.
    #[serde(rename = "supported_parameters")]
    pub supported_parameters: Vec<crate::datadogV2::model::GovernanceControlParameterDefinition>,
    /// A short, human-readable name for the mitigation.
    #[serde(rename = "title")]
    pub title: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GovernanceControlMitigationDefinition {
    pub fn new(
        action_verb: String,
        description: String,
        feature_flags: Vec<String>,
        id: String,
        manual_mitigation_warning: String,
        permissions: Vec<String>,
        requires_ai: bool,
        supported_parameters: Vec<crate::datadogV2::model::GovernanceControlParameterDefinition>,
        title: String,
    ) -> GovernanceControlMitigationDefinition {
        GovernanceControlMitigationDefinition {
            action_verb,
            description,
            execution_modes: None,
            feature_flags,
            id,
            manual_mitigation_warning,
            permissions,
            requires_ai,
            supported_parameters,
            title,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn execution_modes(mut self, value: Vec<String>) -> Self {
        self.execution_modes = Some(value);
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

impl<'de> Deserialize<'de> for GovernanceControlMitigationDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GovernanceControlMitigationDefinitionVisitor;
        impl<'a> Visitor<'a> for GovernanceControlMitigationDefinitionVisitor {
            type Value = GovernanceControlMitigationDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut action_verb: Option<String> = None;
                let mut description: Option<String> = None;
                let mut execution_modes: Option<Vec<String>> = None;
                let mut feature_flags: Option<Vec<String>> = None;
                let mut id: Option<String> = None;
                let mut manual_mitigation_warning: Option<String> = None;
                let mut permissions: Option<Vec<String>> = None;
                let mut requires_ai: Option<bool> = None;
                let mut supported_parameters: Option<
                    Vec<crate::datadogV2::model::GovernanceControlParameterDefinition>,
                > = None;
                let mut title: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "action_verb" => {
                            action_verb =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "execution_modes" => {
                            if v.is_null() {
                                continue;
                            }
                            execution_modes =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "feature_flags" => {
                            feature_flags =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "manual_mitigation_warning" => {
                            manual_mitigation_warning =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "permissions" => {
                            permissions =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "requires_ai" => {
                            requires_ai =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "supported_parameters" => {
                            supported_parameters =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "title" => {
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let action_verb =
                    action_verb.ok_or_else(|| M::Error::missing_field("action_verb"))?;
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let feature_flags =
                    feature_flags.ok_or_else(|| M::Error::missing_field("feature_flags"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let manual_mitigation_warning = manual_mitigation_warning
                    .ok_or_else(|| M::Error::missing_field("manual_mitigation_warning"))?;
                let permissions =
                    permissions.ok_or_else(|| M::Error::missing_field("permissions"))?;
                let requires_ai =
                    requires_ai.ok_or_else(|| M::Error::missing_field("requires_ai"))?;
                let supported_parameters = supported_parameters
                    .ok_or_else(|| M::Error::missing_field("supported_parameters"))?;
                let title = title.ok_or_else(|| M::Error::missing_field("title"))?;

                let content = GovernanceControlMitigationDefinition {
                    action_verb,
                    description,
                    execution_modes,
                    feature_flags,
                    id,
                    manual_mitigation_warning,
                    permissions,
                    requires_ai,
                    supported_parameters,
                    title,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GovernanceControlMitigationDefinitionVisitor)
    }
}
