// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// SAML assertion attribute.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SAMLAssertionAttribute {
    /// Key/Value pair of attributes used in SAML assertion attributes.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::SAMLAssertionAttributeAttributes>,
    /// The ID of the SAML assertion attribute.
    #[serde(rename = "id")]
    pub id: String,
    /// SAML assertion attributes resource type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::SAMLAssertionAttributesType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SAMLAssertionAttribute {
    pub fn new(
        id: String,
        type_: crate::datadogV2::model::SAMLAssertionAttributesType,
    ) -> SAMLAssertionAttribute {
        SAMLAssertionAttribute {
            attributes: None,
            id,
            type_,
            _unparsed: false,
        }
    }

    pub fn attributes(
        &mut self,
        value: crate::datadogV2::model::SAMLAssertionAttributeAttributes,
    ) -> &mut Self {
        self.attributes = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for SAMLAssertionAttribute {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SAMLAssertionAttributeVisitor;
        impl<'a> Visitor<'a> for SAMLAssertionAttributeVisitor {
            type Value = SAMLAssertionAttribute;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attributes: Option<
                    crate::datadogV2::model::SAMLAssertionAttributeAttributes,
                > = None;
                let mut id: Option<String> = None;
                let mut type_: Option<crate::datadogV2::model::SAMLAssertionAttributesType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "attributes" => {
                            if v.is_null() {
                                continue;
                            }
                            attributes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::SAMLAssertionAttributesType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = SAMLAssertionAttribute {
                    attributes,
                    id,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SAMLAssertionAttributeVisitor)
    }
}
