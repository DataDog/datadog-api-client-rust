// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A uniquely identified resource.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ChangeEventCustomAttributesChangedResource {
    /// The name of the resource that was changed. Limited to 128 characters. Must contain at least one non-whitespace character.
    #[serde(rename = "name")]
    pub name: String,
    /// The type of the resource that was changed.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ChangeEventCustomAttributesChangedResourceType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ChangeEventCustomAttributesChangedResource {
    pub fn new(
        name: String,
        type_: crate::datadogV2::model::ChangeEventCustomAttributesChangedResourceType,
    ) -> ChangeEventCustomAttributesChangedResource {
        ChangeEventCustomAttributesChangedResource {
            name,
            type_,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for ChangeEventCustomAttributesChangedResource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ChangeEventCustomAttributesChangedResourceVisitor;
        impl<'a> Visitor<'a> for ChangeEventCustomAttributesChangedResourceVisitor {
            type Value = ChangeEventCustomAttributesChangedResource;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut name: Option<String> = None;
                let mut type_: Option<
                    crate::datadogV2::model::ChangeEventCustomAttributesChangedResourceType,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::ChangeEventCustomAttributesChangedResourceType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ChangeEventCustomAttributesChangedResource {
                    name,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ChangeEventCustomAttributesChangedResourceVisitor)
    }
}
