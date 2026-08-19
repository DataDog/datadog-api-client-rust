// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A rule restricting remote shell access to specific paths.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExecutionPolicyRemoteActionRshellScopeRule {
    /// The level of remote shell access granted for the target paths.
    #[serde(rename = "access")]
    pub access: crate::datadogV2::model::ExecutionPolicyRemoteActionRshellAccess,
    /// The file system paths this rule applies to.
    #[serde(rename = "target_paths")]
    pub target_paths: Vec<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExecutionPolicyRemoteActionRshellScopeRule {
    pub fn new(
        access: crate::datadogV2::model::ExecutionPolicyRemoteActionRshellAccess,
        target_paths: Vec<String>,
    ) -> ExecutionPolicyRemoteActionRshellScopeRule {
        ExecutionPolicyRemoteActionRshellScopeRule {
            access,
            target_paths,
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

impl<'de> Deserialize<'de> for ExecutionPolicyRemoteActionRshellScopeRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExecutionPolicyRemoteActionRshellScopeRuleVisitor;
        impl<'a> Visitor<'a> for ExecutionPolicyRemoteActionRshellScopeRuleVisitor {
            type Value = ExecutionPolicyRemoteActionRshellScopeRule;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut access: Option<
                    crate::datadogV2::model::ExecutionPolicyRemoteActionRshellAccess,
                > = None;
                let mut target_paths: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "access" => {
                            access = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _access) = access {
                                match _access {
                                    crate::datadogV2::model::ExecutionPolicyRemoteActionRshellAccess::UnparsedObject(_access) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "target_paths" => {
                            target_paths =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let access = access.ok_or_else(|| M::Error::missing_field("access"))?;
                let target_paths =
                    target_paths.ok_or_else(|| M::Error::missing_field("target_paths"))?;

                let content = ExecutionPolicyRemoteActionRshellScopeRule {
                    access,
                    target_paths,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExecutionPolicyRemoteActionRshellScopeRuleVisitor)
    }
}
