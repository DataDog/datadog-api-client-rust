// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes object for a postmortem attachment.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentAttachmentPostmortemAttributes {
    /// The postmortem attachment.
    #[serde(rename = "attachment")]
    pub attachment:
        crate::datadogV2::model::IncidentAttachmentsPostmortemAttributesAttachmentObject,
    /// The type of postmortem attachment attributes.
    #[serde(rename = "attachment_type")]
    pub attachment_type: crate::datadogV2::model::IncidentAttachmentPostmortemAttachmentType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentAttachmentPostmortemAttributes {
    pub fn new(
        attachment: crate::datadogV2::model::IncidentAttachmentsPostmortemAttributesAttachmentObject,
        attachment_type: crate::datadogV2::model::IncidentAttachmentPostmortemAttachmentType,
    ) -> IncidentAttachmentPostmortemAttributes {
        IncidentAttachmentPostmortemAttributes {
            attachment,
            attachment_type,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for IncidentAttachmentPostmortemAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentAttachmentPostmortemAttributesVisitor;
        impl<'a> Visitor<'a> for IncidentAttachmentPostmortemAttributesVisitor {
            type Value = IncidentAttachmentPostmortemAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attachment: Option<crate::datadogV2::model::IncidentAttachmentsPostmortemAttributesAttachmentObject> = None;
                let mut attachment_type: Option<
                    crate::datadogV2::model::IncidentAttachmentPostmortemAttachmentType,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "attachment" => {
                            attachment = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "attachment_type" => {
                            attachment_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _attachment_type) = attachment_type {
                                match _attachment_type {
                                    crate::datadogV2::model::IncidentAttachmentPostmortemAttachmentType::UnparsedObject(_attachment_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }
                let attachment = attachment.ok_or_else(|| M::Error::missing_field("attachment"))?;
                let attachment_type =
                    attachment_type.ok_or_else(|| M::Error::missing_field("attachment_type"))?;

                let content = IncidentAttachmentPostmortemAttributes {
                    attachment,
                    attachment_type,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentAttachmentPostmortemAttributesVisitor)
    }
}
