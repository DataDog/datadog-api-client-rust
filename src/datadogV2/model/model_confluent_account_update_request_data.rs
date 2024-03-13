// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Data object for updating a Confluent account.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ConfluentAccountUpdateRequestData {
    /// Attributes object for updating a Confluent account.
    #[serde(rename = "attributes")]
    pub attributes: crate::datadogV2::model::ConfluentAccountUpdateRequestAttributes,
    /// The JSON:API type for this API. Should always be `confluent-cloud-accounts`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ConfluentAccountType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ConfluentAccountUpdateRequestData {
    pub fn new(
        attributes: crate::datadogV2::model::ConfluentAccountUpdateRequestAttributes,
        type_: crate::datadogV2::model::ConfluentAccountType,
    ) -> ConfluentAccountUpdateRequestData {
        ConfluentAccountUpdateRequestData {
            attributes,
            type_,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for ConfluentAccountUpdateRequestData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ConfluentAccountUpdateRequestDataVisitor;
        impl<'a> Visitor<'a> for ConfluentAccountUpdateRequestDataVisitor {
            type Value = ConfluentAccountUpdateRequestData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attributes: Option<
                    crate::datadogV2::model::ConfluentAccountUpdateRequestAttributes,
                > = None;
                let mut type_: Option<crate::datadogV2::model::ConfluentAccountType> = None;
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
                                    crate::datadogV2::model::ConfluentAccountType::UnparsedObject(_type_) => {
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

                let content = ConfluentAccountUpdateRequestData {
                    attributes,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ConfluentAccountUpdateRequestDataVisitor)
    }
}
