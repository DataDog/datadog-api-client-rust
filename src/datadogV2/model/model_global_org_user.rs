// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// User information for a global organization association.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GlobalOrgUser {
    /// The handle of the user.
    #[serde(rename = "handle")]
    pub handle: String,
    /// The UUID of the user.
    #[serde(rename = "uuid")]
    pub uuid: uuid::Uuid,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GlobalOrgUser {
    pub fn new(handle: String, uuid: uuid::Uuid) -> GlobalOrgUser {
        GlobalOrgUser {
            handle,
            uuid,
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

impl<'de> Deserialize<'de> for GlobalOrgUser {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GlobalOrgUserVisitor;
        impl<'a> Visitor<'a> for GlobalOrgUserVisitor {
            type Value = GlobalOrgUser;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut handle: Option<String> = None;
                let mut uuid: Option<uuid::Uuid> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "handle" => {
                            handle = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "uuid" => {
                            uuid = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let handle = handle.ok_or_else(|| M::Error::missing_field("handle"))?;
                let uuid = uuid.ok_or_else(|| M::Error::missing_field("uuid"))?;

                let content = GlobalOrgUser {
                    handle,
                    uuid,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GlobalOrgUserVisitor)
    }
}
