// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Service owner's email.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ServiceDefinitionV2Email {
    /// Contact value.
    #[serde(rename = "contact")]
    pub contact: String,
    /// Contact email.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Contact type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ServiceDefinitionV2EmailType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ServiceDefinitionV2Email {
    pub fn new(
        contact: String,
        type_: crate::datadogV2::model::ServiceDefinitionV2EmailType,
    ) -> ServiceDefinitionV2Email {
        ServiceDefinitionV2Email {
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

impl<'de> Deserialize<'de> for ServiceDefinitionV2Email {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ServiceDefinitionV2EmailVisitor;
        impl<'a> Visitor<'a> for ServiceDefinitionV2EmailVisitor {
            type Value = ServiceDefinitionV2Email;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut contact: Option<String> = None;
                let mut name: Option<String> = None;
                let mut type_: Option<crate::datadogV2::model::ServiceDefinitionV2EmailType> = None;
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
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::ServiceDefinitionV2EmailType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }
                let contact = contact.ok_or_else(|| M::Error::missing_field("contact"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ServiceDefinitionV2Email {
                    contact,
                    name,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ServiceDefinitionV2EmailVisitor)
    }
}
