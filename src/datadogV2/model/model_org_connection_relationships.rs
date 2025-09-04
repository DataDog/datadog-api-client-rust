// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Related organizations and user.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OrgConnectionRelationships {
    /// User relationship.
    #[serde(rename = "created_by")]
    pub created_by: Option<crate::datadogV2::model::OrgConnectionUserRelationship>,
    /// Org relationship.
    #[serde(rename = "sink_org")]
    pub sink_org: Option<crate::datadogV2::model::OrgConnectionOrgRelationship>,
    /// Org relationship.
    #[serde(rename = "source_org")]
    pub source_org: Option<crate::datadogV2::model::OrgConnectionOrgRelationship>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OrgConnectionRelationships {
    pub fn new() -> OrgConnectionRelationships {
        OrgConnectionRelationships {
            created_by: None,
            sink_org: None,
            source_org: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn created_by(
        mut self,
        value: crate::datadogV2::model::OrgConnectionUserRelationship,
    ) -> Self {
        self.created_by = Some(value);
        self
    }

    pub fn sink_org(
        mut self,
        value: crate::datadogV2::model::OrgConnectionOrgRelationship,
    ) -> Self {
        self.sink_org = Some(value);
        self
    }

    pub fn source_org(
        mut self,
        value: crate::datadogV2::model::OrgConnectionOrgRelationship,
    ) -> Self {
        self.source_org = Some(value);
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

impl Default for OrgConnectionRelationships {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for OrgConnectionRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OrgConnectionRelationshipsVisitor;
        impl<'a> Visitor<'a> for OrgConnectionRelationshipsVisitor {
            type Value = OrgConnectionRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_by: Option<crate::datadogV2::model::OrgConnectionUserRelationship> =
                    None;
                let mut sink_org: Option<crate::datadogV2::model::OrgConnectionOrgRelationship> =
                    None;
                let mut source_org: Option<crate::datadogV2::model::OrgConnectionOrgRelationship> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_by" => {
                            if v.is_null() {
                                continue;
                            }
                            created_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sink_org" => {
                            if v.is_null() {
                                continue;
                            }
                            sink_org = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source_org" => {
                            if v.is_null() {
                                continue;
                            }
                            source_org = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = OrgConnectionRelationships {
                    created_by,
                    sink_org,
                    source_org,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OrgConnectionRelationshipsVisitor)
    }
}
