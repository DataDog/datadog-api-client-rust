// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of an environment.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EnvironmentAttributes {
    /// The timestamp when the environment was created.
    #[serde(rename = "created_at")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Indicates whether this is a production environment.
    #[serde(rename = "is_production")]
    pub is_production: Option<bool>,
    /// The name of the environment.
    #[serde(rename = "name")]
    pub name: String,
    /// Indicates whether feature flag evaluation events include original targeting identifiers and full evaluation context.
    #[serde(rename = "observe_full_evaluation_data")]
    pub observe_full_evaluation_data: Option<bool>,
    /// List of queries to define the environment scope.
    #[serde(rename = "queries")]
    pub queries: Option<Vec<String>>,
    /// Indicates whether feature flag changes require approval in this environment.
    #[serde(rename = "require_feature_flag_approval")]
    pub require_feature_flag_approval: Option<bool>,
    /// The associated Custom OIDC provider ID, or `null` when no provider is associated. Disabled environments may retain a provider association.
    #[serde(
        rename = "subject_authorization_oidc_provider_id",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub subject_authorization_oidc_provider_id: Option<Option<uuid::Uuid>>,
    /// Whether assignment requests for this environment require subject authorization. Allowed values are `disabled` and `enabled`. Enabling requires a provider ID formatted as a nonzero UUID. If omitted on create, defaults to `disabled`. If omitted on update, the stored state is preserved. `null` is not accepted. This setting applies only to the current Datadog site and is not copied to other sites. Before you enable it, configure an eligible Custom OIDC provider in the same organization and site, and verify that the provider's keys are published. Assignment requests fail when the required verification configuration is unavailable.
    #[serde(rename = "subject_authorization_state")]
    pub subject_authorization_state:
        Option<crate::datadogV2::model::EnvironmentSubjectAuthorizationState>,
    /// The timestamp when the environment was last updated.
    #[serde(rename = "updated_at")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl EnvironmentAttributes {
    pub fn new(name: String) -> EnvironmentAttributes {
        EnvironmentAttributes {
            created_at: None,
            is_production: None,
            name,
            observe_full_evaluation_data: None,
            queries: None,
            require_feature_flag_approval: None,
            subject_authorization_oidc_provider_id: None,
            subject_authorization_state: None,
            updated_at: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn created_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn is_production(mut self, value: bool) -> Self {
        self.is_production = Some(value);
        self
    }

    pub fn observe_full_evaluation_data(mut self, value: bool) -> Self {
        self.observe_full_evaluation_data = Some(value);
        self
    }

    pub fn queries(mut self, value: Vec<String>) -> Self {
        self.queries = Some(value);
        self
    }

    pub fn require_feature_flag_approval(mut self, value: bool) -> Self {
        self.require_feature_flag_approval = Some(value);
        self
    }

    pub fn subject_authorization_oidc_provider_id(mut self, value: Option<uuid::Uuid>) -> Self {
        self.subject_authorization_oidc_provider_id = Some(value);
        self
    }

    pub fn subject_authorization_state(
        mut self,
        value: crate::datadogV2::model::EnvironmentSubjectAuthorizationState,
    ) -> Self {
        self.subject_authorization_state = Some(value);
        self
    }

    pub fn updated_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.updated_at = Some(value);
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

impl<'de> Deserialize<'de> for EnvironmentAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EnvironmentAttributesVisitor;
        impl<'a> Visitor<'a> for EnvironmentAttributesVisitor {
            type Value = EnvironmentAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut is_production: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut observe_full_evaluation_data: Option<bool> = None;
                let mut queries: Option<Vec<String>> = None;
                let mut require_feature_flag_approval: Option<bool> = None;
                let mut subject_authorization_oidc_provider_id: Option<Option<uuid::Uuid>> = None;
                let mut subject_authorization_state: Option<
                    crate::datadogV2::model::EnvironmentSubjectAuthorizationState,
                > = None;
                let mut updated_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_production" => {
                            if v.is_null() {
                                continue;
                            }
                            is_production =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "observe_full_evaluation_data" => {
                            if v.is_null() {
                                continue;
                            }
                            observe_full_evaluation_data =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "queries" => {
                            if v.is_null() {
                                continue;
                            }
                            queries = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "require_feature_flag_approval" => {
                            if v.is_null() {
                                continue;
                            }
                            require_feature_flag_approval =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "subject_authorization_oidc_provider_id" => {
                            subject_authorization_oidc_provider_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "subject_authorization_state" => {
                            if v.is_null() {
                                continue;
                            }
                            subject_authorization_state =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _subject_authorization_state) =
                                subject_authorization_state
                            {
                                match _subject_authorization_state {
                                    crate::datadogV2::model::EnvironmentSubjectAuthorizationState::UnparsedObject(_subject_authorization_state) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "updated_at" => {
                            if v.is_null() {
                                continue;
                            }
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = EnvironmentAttributes {
                    created_at,
                    is_production,
                    name,
                    observe_full_evaluation_data,
                    queries,
                    require_feature_flag_approval,
                    subject_authorization_oidc_provider_id,
                    subject_authorization_state,
                    updated_at,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(EnvironmentAttributesVisitor)
    }
}
