// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Writable attributes of an Org Config.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OrgConfigWriteAttributes {
    /// The value of an Org Config.
    #[serde(rename = "value")]
    pub value: serde_json::Value,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OrgConfigWriteAttributes {
    pub fn new(value: serde_json::Value) -> OrgConfigWriteAttributes {
        OrgConfigWriteAttributes {
            value,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for OrgConfigWriteAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OrgConfigWriteAttributesVisitor;
        impl<'a> Visitor<'a> for OrgConfigWriteAttributesVisitor {
            type Value = OrgConfigWriteAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut value: Option<serde_json::Value> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "value" => {
                            value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let value = value.ok_or_else(|| M::Error::missing_field("value"))?;

                let content = OrgConfigWriteAttributes { value, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OrgConfigWriteAttributesVisitor)
    }
}
