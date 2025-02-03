// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `EntityV3APISpecInterfaceFileRef` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EntityV3APISpecInterfaceFileRef {
    /// The reference to the API definition file.
    #[serde(rename = "fileRef")]
    pub file_ref: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl EntityV3APISpecInterfaceFileRef {
    pub fn new() -> EntityV3APISpecInterfaceFileRef {
        EntityV3APISpecInterfaceFileRef {
            file_ref: None,
            _unparsed: false,
        }
    }

    pub fn file_ref(mut self, value: String) -> Self {
        self.file_ref = Some(value);
        self
    }
}

impl Default for EntityV3APISpecInterfaceFileRef {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for EntityV3APISpecInterfaceFileRef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EntityV3APISpecInterfaceFileRefVisitor;
        impl<'a> Visitor<'a> for EntityV3APISpecInterfaceFileRefVisitor {
            type Value = EntityV3APISpecInterfaceFileRef;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut file_ref: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "fileRef" => {
                            if v.is_null() {
                                continue;
                            }
                            file_ref = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = EntityV3APISpecInterfaceFileRef {
                    file_ref,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(EntityV3APISpecInterfaceFileRefVisitor)
    }
}
