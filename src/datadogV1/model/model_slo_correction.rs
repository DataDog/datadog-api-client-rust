// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The response object of a list of SLO corrections.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SLOCorrection {
    /// The attribute object associated with the SLO correction.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV1::model::SLOCorrectionResponseAttributes>,
    /// The ID of the SLO correction.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// SLO correction resource type.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV1::model::SLOCorrectionType>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SLOCorrection {
    pub fn new() -> SLOCorrection {
        SLOCorrection {
            attributes: None,
            id: None,
            type_: None,
            _unparsed: false,
        }
    }

    pub fn attributes(
        mut self,
        value: crate::datadogV1::model::SLOCorrectionResponseAttributes,
    ) -> Self {
        self.attributes = Some(value);
        self
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    pub fn type_(mut self, value: crate::datadogV1::model::SLOCorrectionType) -> Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for SLOCorrection {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SLOCorrection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SLOCorrectionVisitor;
        impl<'a> Visitor<'a> for SLOCorrectionVisitor {
            type Value = SLOCorrection;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attributes: Option<
                    crate::datadogV1::model::SLOCorrectionResponseAttributes,
                > = None;
                let mut id: Option<String> = None;
                let mut type_: Option<crate::datadogV1::model::SLOCorrectionType> = None;
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
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::SLOCorrectionType::UnparsedObject(
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

                let content = SLOCorrection {
                    attributes,
                    id,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SLOCorrectionVisitor)
    }
}
