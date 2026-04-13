// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Relationships of an org group policy override.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OrgGroupPolicyOverrideRelationships {
    /// Relationship to a single org group.
    #[serde(rename = "org_group")]
    pub org_group: Option<crate::datadogV2::model::OrgGroupRelationshipToOne>,
    /// Relationship to a single org group policy.
    #[serde(rename = "org_group_policy")]
    pub org_group_policy: Option<crate::datadogV2::model::OrgGroupPolicyRelationshipToOne>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OrgGroupPolicyOverrideRelationships {
    pub fn new() -> OrgGroupPolicyOverrideRelationships {
        OrgGroupPolicyOverrideRelationships {
            org_group: None,
            org_group_policy: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn org_group(mut self, value: crate::datadogV2::model::OrgGroupRelationshipToOne) -> Self {
        self.org_group = Some(value);
        self
    }

    pub fn org_group_policy(
        mut self,
        value: crate::datadogV2::model::OrgGroupPolicyRelationshipToOne,
    ) -> Self {
        self.org_group_policy = Some(value);
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

impl Default for OrgGroupPolicyOverrideRelationships {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for OrgGroupPolicyOverrideRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OrgGroupPolicyOverrideRelationshipsVisitor;
        impl<'a> Visitor<'a> for OrgGroupPolicyOverrideRelationshipsVisitor {
            type Value = OrgGroupPolicyOverrideRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut org_group: Option<crate::datadogV2::model::OrgGroupRelationshipToOne> =
                    None;
                let mut org_group_policy: Option<
                    crate::datadogV2::model::OrgGroupPolicyRelationshipToOne,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "org_group" => {
                            if v.is_null() {
                                continue;
                            }
                            org_group = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "org_group_policy" => {
                            if v.is_null() {
                                continue;
                            }
                            org_group_policy =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = OrgGroupPolicyOverrideRelationships {
                    org_group,
                    org_group_policy,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OrgGroupPolicyOverrideRelationshipsVisitor)
    }
}
