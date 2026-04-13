// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for creating a policy override.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OrgGroupPolicyOverrideCreateAttributes {
    /// The site of the organization.
    #[serde(rename = "org_site")]
    pub org_site: String,
    /// The UUID of the organization to grant the override.
    #[serde(rename = "org_uuid")]
    pub org_uuid: uuid::Uuid,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OrgGroupPolicyOverrideCreateAttributes {
    pub fn new(org_site: String, org_uuid: uuid::Uuid) -> OrgGroupPolicyOverrideCreateAttributes {
        OrgGroupPolicyOverrideCreateAttributes {
            org_site,
            org_uuid,
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

impl<'de> Deserialize<'de> for OrgGroupPolicyOverrideCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OrgGroupPolicyOverrideCreateAttributesVisitor;
        impl<'a> Visitor<'a> for OrgGroupPolicyOverrideCreateAttributesVisitor {
            type Value = OrgGroupPolicyOverrideCreateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut org_site: Option<String> = None;
                let mut org_uuid: Option<uuid::Uuid> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "org_site" => {
                            org_site = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "org_uuid" => {
                            org_uuid = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let org_site = org_site.ok_or_else(|| M::Error::missing_field("org_site"))?;
                let org_uuid = org_uuid.ok_or_else(|| M::Error::missing_field("org_uuid"))?;

                let content = OrgGroupPolicyOverrideCreateAttributes {
                    org_site,
                    org_uuid,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OrgGroupPolicyOverrideCreateAttributesVisitor)
    }
}
