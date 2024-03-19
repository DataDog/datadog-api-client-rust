// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Azure config Post data.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AzureUCConfigPostData {
    /// Attributes for Azure config Post Request.
    #[serde(rename = "attributes")]
    pub attributes: crate::datadogV2::model::AzureUCConfigPostRequestAttributes,
    /// Type of Azure config Post Request.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::AzureUCConfigPostRequestType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AzureUCConfigPostData {
    pub fn new(
        attributes: crate::datadogV2::model::AzureUCConfigPostRequestAttributes,
        type_: crate::datadogV2::model::AzureUCConfigPostRequestType,
    ) -> AzureUCConfigPostData {
        AzureUCConfigPostData {
            attributes,
            type_,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for AzureUCConfigPostData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AzureUCConfigPostDataVisitor;
        impl<'a> Visitor<'a> for AzureUCConfigPostDataVisitor {
            type Value = AzureUCConfigPostData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attributes: Option<
                    crate::datadogV2::model::AzureUCConfigPostRequestAttributes,
                > = None;
                let mut type_: Option<crate::datadogV2::model::AzureUCConfigPostRequestType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "attributes" => {
                            attributes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::AzureUCConfigPostRequestType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }
                let attributes = attributes.ok_or_else(|| M::Error::missing_field("attributes"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = AzureUCConfigPostData {
                    attributes,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AzureUCConfigPostDataVisitor)
    }
}
