// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Relationships of the managed organizations resource.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ManagedOrgsRelationships {
    /// Relationship to the current organization.
    #[serde(rename = "current_org")]
    pub current_org: crate::datadogV2::model::ManagedOrgsRelationshipToOrg,
    /// Relationship to the managed organizations.
    #[serde(rename = "managed_orgs")]
    pub managed_orgs: crate::datadogV2::model::ManagedOrgsRelationshipToOrgs,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ManagedOrgsRelationships {
    pub fn new(
        current_org: crate::datadogV2::model::ManagedOrgsRelationshipToOrg,
        managed_orgs: crate::datadogV2::model::ManagedOrgsRelationshipToOrgs,
    ) -> ManagedOrgsRelationships {
        ManagedOrgsRelationships {
            current_org,
            managed_orgs,
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

impl<'de> Deserialize<'de> for ManagedOrgsRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ManagedOrgsRelationshipsVisitor;
        impl<'a> Visitor<'a> for ManagedOrgsRelationshipsVisitor {
            type Value = ManagedOrgsRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut current_org: Option<crate::datadogV2::model::ManagedOrgsRelationshipToOrg> =
                    None;
                let mut managed_orgs: Option<
                    crate::datadogV2::model::ManagedOrgsRelationshipToOrgs,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "current_org" => {
                            current_org =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "managed_orgs" => {
                            managed_orgs =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let current_org =
                    current_org.ok_or_else(|| M::Error::missing_field("current_org"))?;
                let managed_orgs =
                    managed_orgs.ok_or_else(|| M::Error::missing_field("managed_orgs"))?;

                let content = ManagedOrgsRelationships {
                    current_org,
                    managed_orgs,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ManagedOrgsRelationshipsVisitor)
    }
}
