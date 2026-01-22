// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attachment's attributes.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AttachmentDataAttributes {
    /// The attachment object.
    #[serde(rename = "attachment")]
    pub attachment: Option<crate::datadogV2::model::AttachmentDataAttributesAttachment>,
    /// The type of the attachment.
    #[serde(rename = "attachment_type")]
    pub attachment_type: Option<crate::datadogV2::model::AttachmentDataAttributesAttachmentType>,
    /// Timestamp when the attachment was last modified.
    #[serde(rename = "modified")]
    pub modified: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AttachmentDataAttributes {
    pub fn new() -> AttachmentDataAttributes {
        AttachmentDataAttributes {
            attachment: None,
            attachment_type: None,
            modified: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn attachment(
        mut self,
        value: crate::datadogV2::model::AttachmentDataAttributesAttachment,
    ) -> Self {
        self.attachment = Some(value);
        self
    }

    pub fn attachment_type(
        mut self,
        value: crate::datadogV2::model::AttachmentDataAttributesAttachmentType,
    ) -> Self {
        self.attachment_type = Some(value);
        self
    }

    pub fn modified(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.modified = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl Default for AttachmentDataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AttachmentDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AttachmentDataAttributesVisitor;
        impl<'a> Visitor<'a> for AttachmentDataAttributesVisitor {
            type Value = AttachmentDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attachment: Option<
                    crate::datadogV2::model::AttachmentDataAttributesAttachment,
                > = None;
                let mut attachment_type: Option<
                    crate::datadogV2::model::AttachmentDataAttributesAttachmentType,
                > = None;
                let mut modified: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "attachment" => {
                            if v.is_null() {
                                continue;
                            }
                            attachment = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "attachment_type" => {
                            if v.is_null() {
                                continue;
                            }
                            attachment_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _attachment_type) = attachment_type {
                                match _attachment_type {
                                    crate::datadogV2::model::AttachmentDataAttributesAttachmentType::UnparsedObject(_attachment_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "modified" => {
                            if v.is_null() {
                                continue;
                            }
                            modified = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = AttachmentDataAttributes {
                    attachment,
                    attachment_type,
                    modified,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AttachmentDataAttributesVisitor)
    }
}
