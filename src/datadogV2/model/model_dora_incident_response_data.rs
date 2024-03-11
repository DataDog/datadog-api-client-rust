// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Response after receiving a DORA incident event.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DORAIncidentResponseData {
    /// The ID of the received DORA incident event.
    #[serde(rename = "id")]
    pub id: String,
    /// JSON:API type for DORA incident events.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::DORAIncidentType>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DORAIncidentResponseData {
    pub fn new(id: String) -> DORAIncidentResponseData {
        DORAIncidentResponseData {
            id,
            type_: None,
            _unparsed: false,
        }
    }

    pub fn type_(&mut self, value: crate::datadogV2::model::DORAIncidentType) -> &mut Self {
        self.type_ = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for DORAIncidentResponseData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DORAIncidentResponseDataVisitor;
        impl<'a> Visitor<'a> for DORAIncidentResponseDataVisitor {
            type Value = DORAIncidentResponseData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut id: Option<String> = None;
                let mut type_: Option<crate::datadogV2::model::DORAIncidentType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::DORAIncidentType::UnparsedObject(
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
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;

                let content = DORAIncidentResponseData {
                    id,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DORAIncidentResponseDataVisitor)
    }
}
