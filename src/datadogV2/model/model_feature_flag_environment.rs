// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Environment-specific settings for a feature flag.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FeatureFlagEnvironment {
    /// Allocation metadata for this environment.
    #[serde(
        rename = "allocations",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub allocations: Option<Option<std::collections::BTreeMap<String, serde_json::Value>>>,
    /// The allocation key used for the default variant.
    #[serde(rename = "default_allocation_key")]
    pub default_allocation_key: Option<String>,
    /// The ID of the default variant for this environment.
    #[serde(
        rename = "default_variant_id",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub default_variant_id: Option<Option<String>>,
    /// The ID of the environment.
    #[serde(rename = "environment_id")]
    pub environment_id: uuid::Uuid,
    /// The name of the environment.
    #[serde(rename = "environment_name")]
    pub environment_name: Option<String>,
    /// Queries that target this environment.
    #[serde(rename = "environment_queries")]
    pub environment_queries: Option<Vec<String>>,
    /// Indicates whether the environment is production.
    #[serde(rename = "is_production")]
    pub is_production: Option<bool>,
    /// The allocation key used for the override variant.
    #[serde(rename = "override_allocation_key")]
    pub override_allocation_key: Option<String>,
    /// The ID of the override variant for this environment.
    #[serde(
        rename = "override_variant_id",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub override_variant_id: Option<Option<String>>,
    /// Pending suggestion identifier, if approval is required.
    #[serde(
        rename = "pending_suggestion_id",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub pending_suggestion_id: Option<Option<String>>,
    /// Indicates whether feature flag changes require approval in this environment.
    #[serde(rename = "require_feature_flag_approval")]
    pub require_feature_flag_approval: Option<bool>,
    /// The status of a feature flag in an environment.
    #[serde(rename = "status")]
    pub status: crate::datadogV2::model::FeatureFlagStatus,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FeatureFlagEnvironment {
    pub fn new(
        environment_id: uuid::Uuid,
        status: crate::datadogV2::model::FeatureFlagStatus,
    ) -> FeatureFlagEnvironment {
        FeatureFlagEnvironment {
            allocations: None,
            default_allocation_key: None,
            default_variant_id: None,
            environment_id,
            environment_name: None,
            environment_queries: None,
            is_production: None,
            override_allocation_key: None,
            override_variant_id: None,
            pending_suggestion_id: None,
            require_feature_flag_approval: None,
            status,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn allocations(
        mut self,
        value: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    ) -> Self {
        self.allocations = Some(value);
        self
    }

    pub fn default_allocation_key(mut self, value: String) -> Self {
        self.default_allocation_key = Some(value);
        self
    }

    pub fn default_variant_id(mut self, value: Option<String>) -> Self {
        self.default_variant_id = Some(value);
        self
    }

    pub fn environment_name(mut self, value: String) -> Self {
        self.environment_name = Some(value);
        self
    }

    pub fn environment_queries(mut self, value: Vec<String>) -> Self {
        self.environment_queries = Some(value);
        self
    }

    pub fn is_production(mut self, value: bool) -> Self {
        self.is_production = Some(value);
        self
    }

    pub fn override_allocation_key(mut self, value: String) -> Self {
        self.override_allocation_key = Some(value);
        self
    }

    pub fn override_variant_id(mut self, value: Option<String>) -> Self {
        self.override_variant_id = Some(value);
        self
    }

    pub fn pending_suggestion_id(mut self, value: Option<String>) -> Self {
        self.pending_suggestion_id = Some(value);
        self
    }

    pub fn require_feature_flag_approval(mut self, value: bool) -> Self {
        self.require_feature_flag_approval = Some(value);
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

impl<'de> Deserialize<'de> for FeatureFlagEnvironment {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FeatureFlagEnvironmentVisitor;
        impl<'a> Visitor<'a> for FeatureFlagEnvironmentVisitor {
            type Value = FeatureFlagEnvironment;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut allocations: Option<
                    Option<std::collections::BTreeMap<String, serde_json::Value>>,
                > = None;
                let mut default_allocation_key: Option<String> = None;
                let mut default_variant_id: Option<Option<String>> = None;
                let mut environment_id: Option<uuid::Uuid> = None;
                let mut environment_name: Option<String> = None;
                let mut environment_queries: Option<Vec<String>> = None;
                let mut is_production: Option<bool> = None;
                let mut override_allocation_key: Option<String> = None;
                let mut override_variant_id: Option<Option<String>> = None;
                let mut pending_suggestion_id: Option<Option<String>> = None;
                let mut require_feature_flag_approval: Option<bool> = None;
                let mut status: Option<crate::datadogV2::model::FeatureFlagStatus> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "allocations" => {
                            allocations =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "default_allocation_key" => {
                            if v.is_null() {
                                continue;
                            }
                            default_allocation_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "default_variant_id" => {
                            default_variant_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "environment_id" => {
                            environment_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "environment_name" => {
                            if v.is_null() {
                                continue;
                            }
                            environment_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "environment_queries" => {
                            if v.is_null() {
                                continue;
                            }
                            environment_queries =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_production" => {
                            if v.is_null() {
                                continue;
                            }
                            is_production =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "override_allocation_key" => {
                            if v.is_null() {
                                continue;
                            }
                            override_allocation_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "override_variant_id" => {
                            override_variant_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "pending_suggestion_id" => {
                            pending_suggestion_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "require_feature_flag_approval" => {
                            if v.is_null() {
                                continue;
                            }
                            require_feature_flag_approval =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::FeatureFlagStatus::UnparsedObject(
                                        _status,
                                    ) => {
                                        _unparsed = true;
                                    }
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
                let environment_id =
                    environment_id.ok_or_else(|| M::Error::missing_field("environment_id"))?;
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;

                let content = FeatureFlagEnvironment {
                    allocations,
                    default_allocation_key,
                    default_variant_id,
                    environment_id,
                    environment_name,
                    environment_queries,
                    is_production,
                    override_allocation_key,
                    override_variant_id,
                    pending_suggestion_id,
                    require_feature_flag_approval,
                    status,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FeatureFlagEnvironmentVisitor)
    }
}
