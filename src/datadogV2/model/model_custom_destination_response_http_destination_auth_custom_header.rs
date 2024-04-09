// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Custom header access authentication.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CustomDestinationResponseHttpDestinationAuthCustomHeader {
    /// The header name of the authentication.
    #[serde(rename = "header_name")]
    pub header_name: String,
    /// Type of the custom header access authentication.
    #[serde(rename = "type")]
    pub type_:
        crate::datadogV2::model::CustomDestinationResponseHttpDestinationAuthCustomHeaderType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CustomDestinationResponseHttpDestinationAuthCustomHeader {
    pub fn new(
        header_name: String,
        type_: crate::datadogV2::model::CustomDestinationResponseHttpDestinationAuthCustomHeaderType,
    ) -> CustomDestinationResponseHttpDestinationAuthCustomHeader {
        CustomDestinationResponseHttpDestinationAuthCustomHeader {
            header_name,
            type_,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for CustomDestinationResponseHttpDestinationAuthCustomHeader {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CustomDestinationResponseHttpDestinationAuthCustomHeaderVisitor;
        impl<'a> Visitor<'a> for CustomDestinationResponseHttpDestinationAuthCustomHeaderVisitor {
            type Value = CustomDestinationResponseHttpDestinationAuthCustomHeader;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut header_name: Option<String> = None;
                let mut type_: Option<crate::datadogV2::model::CustomDestinationResponseHttpDestinationAuthCustomHeaderType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "header_name" => {
                            header_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::CustomDestinationResponseHttpDestinationAuthCustomHeaderType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }
                let header_name =
                    header_name.ok_or_else(|| M::Error::missing_field("header_name"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = CustomDestinationResponseHttpDestinationAuthCustomHeader {
                    header_name,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(CustomDestinationResponseHttpDestinationAuthCustomHeaderVisitor)
    }
}
