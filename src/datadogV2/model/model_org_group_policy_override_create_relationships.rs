// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Relationships for creating a policy override.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OrgGroupPolicyOverrideCreateRelationships {
    /// Relationship to a single org group.
    #[serde(rename = "org_group")]
    pub org_group: crate::datadogV2::model::OrgGroupRelationshipToOne,
    /// Relationship to a single org group policy.
    #[serde(rename = "org_group_policy")]
    pub org_group_policy: crate::datadogV2::model::OrgGroupPolicyRelationshipToOne,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OrgGroupPolicyOverrideCreateRelationships {
    pub fn new(
        org_group: crate::datadogV2::model::OrgGroupRelationshipToOne,
        org_group_policy: crate::datadogV2::model::OrgGroupPolicyRelationshipToOne,
    ) -> OrgGroupPolicyOverrideCreateRelationships {
        OrgGroupPolicyOverrideCreateRelationships {
            org_group,
            org_group_policy,
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

impl<'de> Deserialize<'de> for OrgGroupPolicyOverrideCreateRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OrgGroupPolicyOverrideCreateRelationshipsVisitor;
        impl<'a> Visitor<'a> for OrgGroupPolicyOverrideCreateRelationshipsVisitor {
            type Value = OrgGroupPolicyOverrideCreateRelationships;

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
                            org_group = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "org_group_policy" => {
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
                let org_group = org_group.ok_or_else(|| M::Error::missing_field("org_group"))?;
                let org_group_policy =
                    org_group_policy.ok_or_else(|| M::Error::missing_field("org_group_policy"))?;

                let content = OrgGroupPolicyOverrideCreateRelationships {
                    org_group,
                    org_group_policy,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OrgGroupPolicyOverrideCreateRelationshipsVisitor)
    }
}
