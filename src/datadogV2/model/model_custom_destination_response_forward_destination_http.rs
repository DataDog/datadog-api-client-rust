// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The HTTP destination.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CustomDestinationResponseForwardDestinationHttp {
    /// Authentication method of the HTTP requests.
    #[serde(rename = "auth")]
    pub auth: crate::datadogV2::model::CustomDestinationResponseHttpDestinationAuth,
    /// The destination for which logs will be forwarded to.
    /// Must have HTTPS scheme and forwarding back to Datadog is not allowed.
    #[serde(rename = "endpoint")]
    pub endpoint: String,
    /// Type of the HTTP destination.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::CustomDestinationResponseForwardDestinationHttpType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CustomDestinationResponseForwardDestinationHttp {
    pub fn new(
        auth: crate::datadogV2::model::CustomDestinationResponseHttpDestinationAuth,
        endpoint: String,
        type_: crate::datadogV2::model::CustomDestinationResponseForwardDestinationHttpType,
    ) -> CustomDestinationResponseForwardDestinationHttp {
        CustomDestinationResponseForwardDestinationHttp {
            auth,
            endpoint,
            type_,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for CustomDestinationResponseForwardDestinationHttp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CustomDestinationResponseForwardDestinationHttpVisitor;
        impl<'a> Visitor<'a> for CustomDestinationResponseForwardDestinationHttpVisitor {
            type Value = CustomDestinationResponseForwardDestinationHttp;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut auth: Option<
                    crate::datadogV2::model::CustomDestinationResponseHttpDestinationAuth,
                > = None;
                let mut endpoint: Option<String> = None;
                let mut type_: Option<
                    crate::datadogV2::model::CustomDestinationResponseForwardDestinationHttpType,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "auth" => {
                            auth = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _auth) = auth {
                                match _auth {
                                    crate::datadogV2::model::CustomDestinationResponseHttpDestinationAuth::UnparsedObject(_auth) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "endpoint" => {
                            endpoint = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::CustomDestinationResponseForwardDestinationHttpType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }
                let auth = auth.ok_or_else(|| M::Error::missing_field("auth"))?;
                let endpoint = endpoint.ok_or_else(|| M::Error::missing_field("endpoint"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = CustomDestinationResponseForwardDestinationHttp {
                    auth,
                    endpoint,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CustomDestinationResponseForwardDestinationHttpVisitor)
    }
}
