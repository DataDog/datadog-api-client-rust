// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The data object associated with the SLO correction to be created.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SLOCorrectionCreateData {
    /// The attribute object associated with the SLO correction to be created.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV1::model::SLOCorrectionCreateRequestAttributes>,
    /// SLO correction resource type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::SLOCorrectionType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SLOCorrectionCreateData {
    pub fn new(type_: crate::datadogV1::model::SLOCorrectionType) -> SLOCorrectionCreateData {
        SLOCorrectionCreateData {
            attributes: None,
            type_,
            _unparsed: false,
        }
    }

    pub fn attributes(
        mut self,
        value: crate::datadogV1::model::SLOCorrectionCreateRequestAttributes,
    ) -> Self {
        self.attributes = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for SLOCorrectionCreateData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SLOCorrectionCreateDataVisitor;
        impl<'a> Visitor<'a> for SLOCorrectionCreateDataVisitor {
            type Value = SLOCorrectionCreateData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attributes: Option<
                    crate::datadogV1::model::SLOCorrectionCreateRequestAttributes,
                > = None;
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
                        "type" => {
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
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = SLOCorrectionCreateData {
                    attributes,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SLOCorrectionCreateDataVisitor)
    }
}
