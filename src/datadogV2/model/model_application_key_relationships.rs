// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Resources related to the application key.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ApplicationKeyRelationships {
    /// Relationship to user.
    #[serde(rename = "owned_by")]
    pub owned_by: Option<crate::datadogV2::model::RelationshipToUser>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ApplicationKeyRelationships {
    pub fn new() -> ApplicationKeyRelationships {
        ApplicationKeyRelationships {
            owned_by: None,
            _unparsed: false,
        }
    }

    pub fn owned_by(mut self, value: crate::datadogV2::model::RelationshipToUser) -> Self {
        self.owned_by = Some(value);
        self
    }
}

impl Default for ApplicationKeyRelationships {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ApplicationKeyRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ApplicationKeyRelationshipsVisitor;
        impl<'a> Visitor<'a> for ApplicationKeyRelationshipsVisitor {
            type Value = ApplicationKeyRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut owned_by: Option<crate::datadogV2::model::RelationshipToUser> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "owned_by" => {
                            if v.is_null() {
                                continue;
                            }
                            owned_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = ApplicationKeyRelationships {
                    owned_by,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ApplicationKeyRelationshipsVisitor)
    }
}
