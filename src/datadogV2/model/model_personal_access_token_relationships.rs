// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Resources related to the access token.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PersonalAccessTokenRelationships {
    /// Relationship to the leak the access token was found in. `data` is null when the access token has not been detected as leaked.
    #[serde(rename = "leak_information")]
    pub leak_information: Option<crate::datadogV2::model::RelationshipToLeakedKey>,
    /// Relationship to user.
    #[serde(rename = "owned_by")]
    pub owned_by: Option<crate::datadogV2::model::RelationshipToUser>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl PersonalAccessTokenRelationships {
    pub fn new() -> PersonalAccessTokenRelationships {
        PersonalAccessTokenRelationships {
            leak_information: None,
            owned_by: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn leak_information(
        mut self,
        value: crate::datadogV2::model::RelationshipToLeakedKey,
    ) -> Self {
        self.leak_information = Some(value);
        self
    }

    pub fn owned_by(mut self, value: crate::datadogV2::model::RelationshipToUser) -> Self {
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

impl Default for PersonalAccessTokenRelationships {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for PersonalAccessTokenRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PersonalAccessTokenRelationshipsVisitor;
        impl<'a> Visitor<'a> for PersonalAccessTokenRelationshipsVisitor {
            type Value = PersonalAccessTokenRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut leak_information: Option<crate::datadogV2::model::RelationshipToLeakedKey> =
                    None;
                let mut owned_by: Option<crate::datadogV2::model::RelationshipToUser> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "leak_information" => {
                            if v.is_null() {
                                continue;
                            }
                            leak_information =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
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

                let content = PersonalAccessTokenRelationships {
                    leak_information,
                    owned_by,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(PersonalAccessTokenRelationshipsVisitor)
    }
}
