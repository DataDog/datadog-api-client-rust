// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Relationships of a status page email access policy.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct StatusPageEmailAccessPolicyDataRelationships {
    /// The Datadog user who created the email access policy.
    #[serde(rename = "created_by_user")]
    pub created_by_user:
        crate::datadogV2::model::StatusPageEmailAccessPolicyDataRelationshipsCreatedByUser,
    /// The status page associated with the email access policy.
    #[serde(rename = "status_page")]
    pub status_page:
        crate::datadogV2::model::StatusPageEmailAccessPolicyDataRelationshipsStatusPage,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl StatusPageEmailAccessPolicyDataRelationships {
    pub fn new(
        created_by_user: crate::datadogV2::model::StatusPageEmailAccessPolicyDataRelationshipsCreatedByUser,
        status_page: crate::datadogV2::model::StatusPageEmailAccessPolicyDataRelationshipsStatusPage,
    ) -> StatusPageEmailAccessPolicyDataRelationships {
        StatusPageEmailAccessPolicyDataRelationships {
            created_by_user,
            status_page,
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

impl<'de> Deserialize<'de> for StatusPageEmailAccessPolicyDataRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StatusPageEmailAccessPolicyDataRelationshipsVisitor;
        impl<'a> Visitor<'a> for StatusPageEmailAccessPolicyDataRelationshipsVisitor {
            type Value = StatusPageEmailAccessPolicyDataRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_by_user: Option<crate::datadogV2::model::StatusPageEmailAccessPolicyDataRelationshipsCreatedByUser> = None;
                let mut status_page: Option<
                    crate::datadogV2::model::StatusPageEmailAccessPolicyDataRelationshipsStatusPage,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_by_user" => {
                            created_by_user =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status_page" => {
                            status_page =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let created_by_user =
                    created_by_user.ok_or_else(|| M::Error::missing_field("created_by_user"))?;
                let status_page =
                    status_page.ok_or_else(|| M::Error::missing_field("status_page"))?;

                let content = StatusPageEmailAccessPolicyDataRelationships {
                    created_by_user,
                    status_page,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(StatusPageEmailAccessPolicyDataRelationshipsVisitor)
    }
}
