// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Powerpack relationship object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PowerpackRelationships {
    /// Relationship to user.
    #[serde(rename = "author")]
    pub author: Option<crate::datadogV2::model::RelationshipToUser>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl PowerpackRelationships {
    pub fn new() -> PowerpackRelationships {
        PowerpackRelationships {
            author: None,
            _unparsed: false,
        }
    }

    pub fn author(mut self, value: crate::datadogV2::model::RelationshipToUser) -> Self {
        self.author = Some(value);
        self
    }
}

impl Default for PowerpackRelationships {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for PowerpackRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PowerpackRelationshipsVisitor;
        impl<'a> Visitor<'a> for PowerpackRelationshipsVisitor {
            type Value = PowerpackRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut author: Option<crate::datadogV2::model::RelationshipToUser> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "author" => {
                            if v.is_null() {
                                continue;
                            }
                            author = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = PowerpackRelationships { author, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(PowerpackRelationshipsVisitor)
    }
}
