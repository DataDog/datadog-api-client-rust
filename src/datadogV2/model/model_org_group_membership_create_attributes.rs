// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for adding organizations to an org group.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OrgGroupMembershipCreateAttributes {
    /// List of organizations to add. Between 1 and 100 per request. Each `org_uuid` and `org_site` pair must be unique.
    #[serde(rename = "orgs")]
    pub orgs: Vec<crate::datadogV2::model::GlobalOrgIdentifier>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OrgGroupMembershipCreateAttributes {
    pub fn new(
        orgs: Vec<crate::datadogV2::model::GlobalOrgIdentifier>,
    ) -> OrgGroupMembershipCreateAttributes {
        OrgGroupMembershipCreateAttributes {
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

impl<'de> Deserialize<'de> for OrgGroupMembershipCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OrgGroupMembershipCreateAttributesVisitor;
        impl<'a> Visitor<'a> for OrgGroupMembershipCreateAttributesVisitor {
            type Value = OrgGroupMembershipCreateAttributes;

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

                let content = OrgGroupMembershipCreateAttributes {
                    orgs,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OrgGroupMembershipCreateAttributesVisitor)
    }
}
