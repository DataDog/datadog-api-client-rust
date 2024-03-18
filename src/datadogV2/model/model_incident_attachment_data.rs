// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A single incident attachment.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentAttachmentData {
    /// The attributes object for an attachment.
    #[serde(rename = "attributes")]
    pub attributes: crate::datadogV2::model::IncidentAttachmentAttributes,
    /// A unique identifier that represents the incident attachment.
    #[serde(rename = "id")]
    pub id: String,
    /// The incident attachment's relationships.
    #[serde(rename = "relationships")]
    pub relationships: crate::datadogV2::model::IncidentAttachmentRelationships,
    /// The incident attachment resource type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::IncidentAttachmentType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentAttachmentData {
    pub fn new(
        attributes: crate::datadogV2::model::IncidentAttachmentAttributes,
        id: String,
        relationships: crate::datadogV2::model::IncidentAttachmentRelationships,
        type_: crate::datadogV2::model::IncidentAttachmentType,
    ) -> IncidentAttachmentData {
        IncidentAttachmentData {
            attributes,
            id,
            relationships,
            type_,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for IncidentAttachmentData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentAttachmentDataVisitor;
        impl<'a> Visitor<'a> for IncidentAttachmentDataVisitor {
            type Value = IncidentAttachmentData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attributes: Option<crate::datadogV2::model::IncidentAttachmentAttributes> =
                    None;
                let mut id: Option<String> = None;
                let mut relationships: Option<
                    crate::datadogV2::model::IncidentAttachmentRelationships,
                > = None;
                let mut type_: Option<crate::datadogV2::model::IncidentAttachmentType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "attributes" => {
                            attributes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _attributes) = attributes {
                                match _attributes {
                                    crate::datadogV2::model::IncidentAttachmentAttributes::UnparsedObject(_attributes) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "relationships" => {
                            relationships =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::IncidentAttachmentType::UnparsedObject(_type_) => {
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
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let relationships =
                    relationships.ok_or_else(|| M::Error::missing_field("relationships"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = IncidentAttachmentData {
                    attributes,
                    id,
                    relationships,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentAttachmentDataVisitor)
    }
}
