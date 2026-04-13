// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Relationships for bulk updating memberships.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OrgGroupMembershipBulkUpdateRelationships {
    /// Relationship to a single org group.
    #[serde(rename = "source_org_group")]
    pub source_org_group: crate::datadogV2::model::OrgGroupRelationshipToOne,
    /// Relationship to a single org group.
    #[serde(rename = "target_org_group")]
    pub target_org_group: crate::datadogV2::model::OrgGroupRelationshipToOne,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OrgGroupMembershipBulkUpdateRelationships {
    pub fn new(
        source_org_group: crate::datadogV2::model::OrgGroupRelationshipToOne,
        target_org_group: crate::datadogV2::model::OrgGroupRelationshipToOne,
    ) -> OrgGroupMembershipBulkUpdateRelationships {
        OrgGroupMembershipBulkUpdateRelationships {
            source_org_group,
            target_org_group,
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

impl<'de> Deserialize<'de> for OrgGroupMembershipBulkUpdateRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OrgGroupMembershipBulkUpdateRelationshipsVisitor;
        impl<'a> Visitor<'a> for OrgGroupMembershipBulkUpdateRelationshipsVisitor {
            type Value = OrgGroupMembershipBulkUpdateRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut source_org_group: Option<
                    crate::datadogV2::model::OrgGroupRelationshipToOne,
                > = None;
                let mut target_org_group: Option<
                    crate::datadogV2::model::OrgGroupRelationshipToOne,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "source_org_group" => {
                            source_org_group =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "target_org_group" => {
                            target_org_group =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let source_org_group =
                    source_org_group.ok_or_else(|| M::Error::missing_field("source_org_group"))?;
                let target_org_group =
                    target_org_group.ok_or_else(|| M::Error::missing_field("target_org_group"))?;

                let content = OrgGroupMembershipBulkUpdateRelationships {
                    source_org_group,
                    target_org_group,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OrgGroupMembershipBulkUpdateRelationshipsVisitor)
    }
}
