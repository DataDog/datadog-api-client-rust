// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of Entity V3 Datastore Spec object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EntityV3DatastoreSpec {
    /// The lifecycle state of the datastore.
    #[serde(rename = "lifecycle")]
    pub lifecycle: Option<String>,
    /// The importance of the datastore
    #[serde(rename = "tier")]
    pub tier: Option<String>,
    /// The type of datastore
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl EntityV3DatastoreSpec {
    pub fn new() -> EntityV3DatastoreSpec {
        EntityV3DatastoreSpec {
            lifecycle: None,
            tier: None,
            type_: None,
            _unparsed: false,
        }
    }

    pub fn lifecycle(mut self, value: String) -> Self {
        self.lifecycle = Some(value);
        self
    }

    pub fn tier(mut self, value: String) -> Self {
        self.tier = Some(value);
        self
    }

    pub fn type_(mut self, value: String) -> Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for EntityV3DatastoreSpec {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for EntityV3DatastoreSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EntityV3DatastoreSpecVisitor;
        impl<'a> Visitor<'a> for EntityV3DatastoreSpecVisitor {
            type Value = EntityV3DatastoreSpec;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut lifecycle: Option<String> = None;
                let mut tier: Option<String> = None;
                let mut type_: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "lifecycle" => {
                            if v.is_null() {
                                continue;
                            }
                            lifecycle = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tier" => {
                            if v.is_null() {
                                continue;
                            }
                            tier = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = EntityV3DatastoreSpec {
                    lifecycle,
                    tier,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(EntityV3DatastoreSpecVisitor)
    }
}
