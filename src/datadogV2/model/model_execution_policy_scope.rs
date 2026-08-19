// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Restricts where the policy applies. At most one of `kubernetes`, `scripts`,
/// or `remote_action_rshell` can be set. An empty object means the policy has
/// no scope restriction.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExecutionPolicyScope {
    /// Restricts the policy to specific Kubernetes namespaces.
    #[serde(rename = "kubernetes")]
    pub kubernetes: Option<crate::datadogV2::model::ExecutionPolicyKubernetesScope>,
    /// Restricts the policy to specific remote shell paths.
    #[serde(rename = "remote_action_rshell")]
    pub remote_action_rshell:
        Option<crate::datadogV2::model::ExecutionPolicyRemoteActionRshellScope>,
    /// Restricts the policy to specific scripts.
    #[serde(rename = "scripts")]
    pub scripts: Option<crate::datadogV2::model::ExecutionPolicyScriptScope>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExecutionPolicyScope {
    pub fn new() -> ExecutionPolicyScope {
        ExecutionPolicyScope {
            kubernetes: None,
            remote_action_rshell: None,
            scripts: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn kubernetes(
        mut self,
        value: crate::datadogV2::model::ExecutionPolicyKubernetesScope,
    ) -> Self {
        self.kubernetes = Some(value);
        self
    }

    pub fn remote_action_rshell(
        mut self,
        value: crate::datadogV2::model::ExecutionPolicyRemoteActionRshellScope,
    ) -> Self {
        self.remote_action_rshell = Some(value);
        self
    }

    pub fn scripts(mut self, value: crate::datadogV2::model::ExecutionPolicyScriptScope) -> Self {
        self.scripts = Some(value);
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

impl Default for ExecutionPolicyScope {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ExecutionPolicyScope {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExecutionPolicyScopeVisitor;
        impl<'a> Visitor<'a> for ExecutionPolicyScopeVisitor {
            type Value = ExecutionPolicyScope;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut kubernetes: Option<
                    crate::datadogV2::model::ExecutionPolicyKubernetesScope,
                > = None;
                let mut remote_action_rshell: Option<
                    crate::datadogV2::model::ExecutionPolicyRemoteActionRshellScope,
                > = None;
                let mut scripts: Option<crate::datadogV2::model::ExecutionPolicyScriptScope> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "kubernetes" => {
                            if v.is_null() {
                                continue;
                            }
                            kubernetes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "remote_action_rshell" => {
                            if v.is_null() {
                                continue;
                            }
                            remote_action_rshell =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scripts" => {
                            if v.is_null() {
                                continue;
                            }
                            scripts = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ExecutionPolicyScope {
                    kubernetes,
                    remote_action_rshell,
                    scripts,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExecutionPolicyScopeVisitor)
    }
}
