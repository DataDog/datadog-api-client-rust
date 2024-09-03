// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of Entity V3 Metadata Contacts Items object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EntityV3MetadataContactsItems {
    /// Contact value
    #[serde(rename = "contact")]
    pub contact: String,
    /// Contact name
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Contact type.
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl EntityV3MetadataContactsItems {
    pub fn new(contact: String, type_: String) -> EntityV3MetadataContactsItems {
        EntityV3MetadataContactsItems {
            contact,
            name: None,
            type_,
            _unparsed: false,
        }
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for EntityV3MetadataContactsItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EntityV3MetadataContactsItemsVisitor;
        impl<'a> Visitor<'a> for EntityV3MetadataContactsItemsVisitor {
            type Value = EntityV3MetadataContactsItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut contact: Option<String> = None;
                let mut name: Option<String> = None;
                let mut type_: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "contact" => {
                            contact = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let contact = contact.ok_or_else(|| M::Error::missing_field("contact"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = EntityV3MetadataContactsItems {
                    contact,
                    name,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(EntityV3MetadataContactsItemsVisitor)
    }
}
