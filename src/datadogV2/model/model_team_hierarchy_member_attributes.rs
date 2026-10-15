// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Membership attributes relative to the requested team.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TeamHierarchyMemberAttributes {
    /// Role on the requested team: admin or member. Null for inherited members.
    #[serialize_always]
    #[serde(rename = "direct_role")]
    pub direct_role: Option<String>,
    /// Whether the user belongs directly to the requested team.
    #[serde(rename = "is_direct")]
    pub is_direct: bool,
    /// Membership provisioner. Null for inherited members.
    #[serialize_always]
    #[serde(rename = "provisioned_by")]
    pub provisioned_by: Option<String>,
    /// Identifier supplied by the membership provisioner. Null when unavailable or for inherited members.
    #[serialize_always]
    #[serde(rename = "provisioned_by_id")]
    pub provisioned_by_id: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TeamHierarchyMemberAttributes {
    pub fn new(
        direct_role: Option<String>,
        is_direct: bool,
        provisioned_by: Option<String>,
        provisioned_by_id: Option<String>,
    ) -> TeamHierarchyMemberAttributes {
        TeamHierarchyMemberAttributes {
            direct_role,
            is_direct,
            provisioned_by,
            provisioned_by_id,
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

impl<'de> Deserialize<'de> for TeamHierarchyMemberAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TeamHierarchyMemberAttributesVisitor;
        impl<'a> Visitor<'a> for TeamHierarchyMemberAttributesVisitor {
            type Value = TeamHierarchyMemberAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut direct_role: Option<Option<String>> = None;
                let mut is_direct: Option<bool> = None;
                let mut provisioned_by: Option<Option<String>> = None;
                let mut provisioned_by_id: Option<Option<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "direct_role" => {
                            direct_role =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_direct" => {
                            is_direct = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "provisioned_by" => {
                            provisioned_by =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "provisioned_by_id" => {
                            provisioned_by_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let direct_role =
                    direct_role.ok_or_else(|| M::Error::missing_field("direct_role"))?;
                let is_direct = is_direct.ok_or_else(|| M::Error::missing_field("is_direct"))?;
                let provisioned_by =
                    provisioned_by.ok_or_else(|| M::Error::missing_field("provisioned_by"))?;
                let provisioned_by_id = provisioned_by_id
                    .ok_or_else(|| M::Error::missing_field("provisioned_by_id"))?;

                let content = TeamHierarchyMemberAttributes {
                    direct_role,
                    is_direct,
                    provisioned_by,
                    provisioned_by_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TeamHierarchyMemberAttributesVisitor)
    }
}
