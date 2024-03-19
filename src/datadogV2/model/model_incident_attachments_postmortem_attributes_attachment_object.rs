// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The postmortem attachment.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentAttachmentsPostmortemAttributesAttachmentObject {
    /// The URL of this notebook attachment.
    #[serde(rename = "documentUrl")]
    pub document_url: String,
    /// The title of this postmortem attachment.
    #[serde(rename = "title")]
    pub title: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentAttachmentsPostmortemAttributesAttachmentObject {
    pub fn new(
        document_url: String,
        title: String,
    ) -> IncidentAttachmentsPostmortemAttributesAttachmentObject {
        IncidentAttachmentsPostmortemAttributesAttachmentObject {
            document_url,
            title,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for IncidentAttachmentsPostmortemAttributesAttachmentObject {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentAttachmentsPostmortemAttributesAttachmentObjectVisitor;
        impl<'a> Visitor<'a> for IncidentAttachmentsPostmortemAttributesAttachmentObjectVisitor {
            type Value = IncidentAttachmentsPostmortemAttributesAttachmentObject;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut document_url: Option<String> = None;
                let mut title: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "documentUrl" => {
                            document_url =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "title" => {
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let document_url =
                    document_url.ok_or_else(|| M::Error::missing_field("document_url"))?;
                let title = title.ok_or_else(|| M::Error::missing_field("title"))?;

                let content = IncidentAttachmentsPostmortemAttributesAttachmentObject {
                    document_url,
                    title,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentAttachmentsPostmortemAttributesAttachmentObjectVisitor)
    }
}
