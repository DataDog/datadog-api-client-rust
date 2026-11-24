// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for creating a new environment.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CreateEnvironmentAttributes {
    /// Indicates whether this is a production environment.
    #[serde(rename = "is_production")]
    pub is_production: Option<bool>,
    /// The name of the environment. Required on every request and unique within the organization. Must contain 1 to 200 ASCII characters, start with a letter or digit, and contain only letters, digits, spaces, underscores, hyphens, and periods.
    #[serde(rename = "name")]
    pub name: String,
    /// Controls whether feature flag evaluation events may include original targeting identifiers and full evaluation context. When true, SDKs may include this data. When false, SDKs hash targeting identifiers and omit evaluation context. If omitted, defaults to false.
    #[serde(rename = "observe_full_evaluation_data")]
    pub observe_full_evaluation_data: Option<bool>,
    /// Required list of at least one exact DD_ENV value associated with this environment. Values cannot be used by another environment in the same organization. Wildcards and search expressions are not supported.
    #[serde(rename = "queries")]
    pub queries: Vec<String>,
    /// Indicates whether feature flag changes require approval in this environment.
    #[serde(rename = "require_feature_flag_approval")]
    pub require_feature_flag_approval: Option<bool>,
    /// The ID of a Custom OIDC provider, formatted as a nonzero UUID. Required when `subject_authorization_state` is `enabled`. If omitted or `null`, no provider is associated. A non-null ID must be registered in the current organization and site, even when subject authorization is disabled. An unregistered ID returns `400 Bad Request`, and an unavailable registry returns `503 Service Unavailable`. Saving does not verify provider eligibility or key publication.
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
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CreateEnvironmentAttributes {
    pub fn new(name: String, queries: Vec<String>) -> CreateEnvironmentAttributes {
        CreateEnvironmentAttributes {
            is_production: None,
            name,
            observe_full_evaluation_data: None,
            queries,
            require_feature_flag_approval: None,
            subject_authorization_oidc_provider_id: None,
            subject_authorization_state: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn is_production(mut self, value: bool) -> Self {
        self.is_production = Some(value);
        self
    }

    pub fn observe_full_evaluation_data(mut self, value: bool) -> Self {
        self.observe_full_evaluation_data = Some(value);
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

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for CreateEnvironmentAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CreateEnvironmentAttributesVisitor;
        impl<'a> Visitor<'a> for CreateEnvironmentAttributesVisitor {
            type Value = CreateEnvironmentAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut is_production: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut observe_full_evaluation_data: Option<bool> = None;
                let mut queries: Option<Vec<String>> = None;
                let mut require_feature_flag_approval: Option<bool> = None;
                let mut subject_authorization_oidc_provider_id: Option<Option<uuid::Uuid>> = None;
                let mut subject_authorization_state: Option<
                    crate::datadogV2::model::EnvironmentSubjectAuthorizationState,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
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
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let queries = queries.ok_or_else(|| M::Error::missing_field("queries"))?;

                let content = CreateEnvironmentAttributes {
                    is_production,
                    name,
                    observe_full_evaluation_data,
                    queries,
                    require_feature_flag_approval,
                    subject_authorization_oidc_provider_id,
                    subject_authorization_state,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CreateEnvironmentAttributesVisitor)
    }
}
