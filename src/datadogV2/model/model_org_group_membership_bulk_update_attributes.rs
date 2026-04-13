// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for bulk updating org group memberships.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OrgGroupMembershipBulkUpdateAttributes {
    /// List of organizations to move. Maximum 100 per request.
    #[serde(rename = "orgs")]
    pub orgs: Vec<crate::datadogV2::model::GlobalOrgIdentifier>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OrgGroupMembershipBulkUpdateAttributes {
    pub fn new(
        orgs: Vec<crate::datadogV2::model::GlobalOrgIdentifier>,
    ) -> OrgGroupMembershipBulkUpdateAttributes {
        OrgGroupMembershipBulkUpdateAttributes {
            orgs,
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

impl<'de> Deserialize<'de> for OrgGroupMembershipBulkUpdateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OrgGroupMembershipBulkUpdateAttributesVisitor;
        impl<'a> Visitor<'a> for OrgGroupMembershipBulkUpdateAttributesVisitor {
            type Value = OrgGroupMembershipBulkUpdateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut orgs: Option<Vec<crate::datadogV2::model::GlobalOrgIdentifier>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "orgs" => {
                            orgs = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let orgs = orgs.ok_or_else(|| M::Error::missing_field("orgs"))?;

                let content = OrgGroupMembershipBulkUpdateAttributes {
                    orgs,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OrgGroupMembershipBulkUpdateAttributesVisitor)
    }
}
