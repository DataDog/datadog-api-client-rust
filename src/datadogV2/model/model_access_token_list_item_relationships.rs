// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Resources related to the access token entry in the mixed list response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AccessTokenListItemRelationships {
    /// Relationship to the access token's owner.
    #[serde(rename = "owned_by")]
    pub owned_by: Option<crate::datadogV2::model::RelationshipToAccessTokenOwner>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AccessTokenListItemRelationships {
    pub fn new() -> AccessTokenListItemRelationships {
        AccessTokenListItemRelationships {
            owned_by: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn owned_by(
        mut self,
        value: crate::datadogV2::model::RelationshipToAccessTokenOwner,
    ) -> Self {
        self.owned_by = Some(value);
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

impl Default for AccessTokenListItemRelationships {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AccessTokenListItemRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AccessTokenListItemRelationshipsVisitor;
        impl<'a> Visitor<'a> for AccessTokenListItemRelationshipsVisitor {
            type Value = AccessTokenListItemRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut owned_by: Option<crate::datadogV2::model::RelationshipToAccessTokenOwner> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "owned_by" => {
                            if v.is_null() {
                                continue;
                            }
                            owned_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = AccessTokenListItemRelationships {
                    owned_by,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AccessTokenListItemRelationshipsVisitor)
    }
}
