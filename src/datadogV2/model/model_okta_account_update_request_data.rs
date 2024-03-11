// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Data object for updating an Okta account.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OktaAccountUpdateRequestData {
    /// Attributes object for updating an Okta account.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::OktaAccountUpdateRequestAttributes>,
    /// Account type for an Okta account.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::OktaAccountType>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OktaAccountUpdateRequestData {
    pub fn new() -> OktaAccountUpdateRequestData {
        OktaAccountUpdateRequestData {
            attributes: None,
            type_: None,
            _unparsed: false,
        }
    }

    pub fn attributes(
        &mut self,
        value: crate::datadogV2::model::OktaAccountUpdateRequestAttributes,
    ) -> &mut Self {
        self.attributes = Some(value);
        self
    }

    pub fn type_(&mut self, value: crate::datadogV2::model::OktaAccountType) -> &mut Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for OktaAccountUpdateRequestData {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for OktaAccountUpdateRequestData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OktaAccountUpdateRequestDataVisitor;
        impl<'a> Visitor<'a> for OktaAccountUpdateRequestDataVisitor {
            type Value = OktaAccountUpdateRequestData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attributes: Option<
                    crate::datadogV2::model::OktaAccountUpdateRequestAttributes,
                > = None;
                let mut type_: Option<crate::datadogV2::model::OktaAccountType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "attributes" => {
                            if v.is_null() {
                                continue;
                            }
                            attributes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::OktaAccountType::UnparsedObject(
                                        _type_,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }

                let content = OktaAccountUpdateRequestData {
                    attributes,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OktaAccountUpdateRequestDataVisitor)
    }
}
