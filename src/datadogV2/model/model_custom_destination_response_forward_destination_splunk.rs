// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The Splunk HTTP Event Collector (HEC) destination.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CustomDestinationResponseForwardDestinationSplunk {
    /// The destination for which logs will be forwarded to.
    /// Must have HTTPS scheme and forwarding back to Datadog is not allowed.
    #[serde(rename = "endpoint")]
    pub endpoint: String,
    /// Type of the Splunk HTTP Event Collector (HEC) destination.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::CustomDestinationResponseForwardDestinationSplunkType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CustomDestinationResponseForwardDestinationSplunk {
    pub fn new(
        endpoint: String,
        type_: crate::datadogV2::model::CustomDestinationResponseForwardDestinationSplunkType,
    ) -> CustomDestinationResponseForwardDestinationSplunk {
        CustomDestinationResponseForwardDestinationSplunk {
            endpoint,
            type_,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for CustomDestinationResponseForwardDestinationSplunk {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CustomDestinationResponseForwardDestinationSplunkVisitor;
        impl<'a> Visitor<'a> for CustomDestinationResponseForwardDestinationSplunkVisitor {
            type Value = CustomDestinationResponseForwardDestinationSplunk;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut endpoint: Option<String> = None;
                let mut type_: Option<
                    crate::datadogV2::model::CustomDestinationResponseForwardDestinationSplunkType,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "endpoint" => {
                            endpoint = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::CustomDestinationResponseForwardDestinationSplunkType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }
                let endpoint = endpoint.ok_or_else(|| M::Error::missing_field("endpoint"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = CustomDestinationResponseForwardDestinationSplunk {
                    endpoint,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CustomDestinationResponseForwardDestinationSplunkVisitor)
    }
}
