// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Specifies which principals are associated with a relation.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RestrictionPolicyBinding {
    /// An array of principals. A principal is a subject or group of subjects.
    /// Each principal is formatted as `type:id`. Supported types: `role`, `team`, `user`, and `org`.
    /// The org ID can be obtained through the api/v2/current_user API.
    /// The user principal type accepts service account IDs.
    #[serde(rename = "principals")]
    pub principals: Vec<String>,
    /// The role/level of access.
    #[serde(rename = "relation")]
    pub relation: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RestrictionPolicyBinding {
    pub fn new(principals: Vec<String>, relation: String) -> RestrictionPolicyBinding {
        RestrictionPolicyBinding {
            principals,
            relation,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for RestrictionPolicyBinding {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RestrictionPolicyBindingVisitor;
        impl<'a> Visitor<'a> for RestrictionPolicyBindingVisitor {
            type Value = RestrictionPolicyBinding;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut principals: Option<Vec<String>> = None;
                let mut relation: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "principals" => {
                            principals = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "relation" => {
                            relation = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let principals = principals.ok_or_else(|| M::Error::missing_field("principals"))?;
                let relation = relation.ok_or_else(|| M::Error::missing_field("relation"))?;

                let content = RestrictionPolicyBinding {
                    principals,
                    relation,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RestrictionPolicyBindingVisitor)
    }
}
