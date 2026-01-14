// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes for creating an attachment.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CreateAttachmentRequestDataAttributes {
    /// The attachment object for creating an attachment.
    #[serde(rename = "attachment")]
    pub attachment:
        Option<crate::datadogV2::model::CreateAttachmentRequestDataAttributesAttachment>,
    /// The type of the attachment.
    #[serde(rename = "attachment_type")]
    pub attachment_type: Option<crate::datadogV2::model::AttachmentDataAttributesAttachmentType>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CreateAttachmentRequestDataAttributes {
    pub fn new() -> CreateAttachmentRequestDataAttributes {
        CreateAttachmentRequestDataAttributes {
            attachment: None,
            attachment_type: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn attachment(
        mut self,
        value: crate::datadogV2::model::CreateAttachmentRequestDataAttributesAttachment,
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

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl Default for CreateAttachmentRequestDataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CreateAttachmentRequestDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CreateAttachmentRequestDataAttributesVisitor;
        impl<'a> Visitor<'a> for CreateAttachmentRequestDataAttributesVisitor {
            type Value = CreateAttachmentRequestDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attachment: Option<
                    crate::datadogV2::model::CreateAttachmentRequestDataAttributesAttachment,
                > = None;
                let mut attachment_type: Option<
                    crate::datadogV2::model::AttachmentDataAttributesAttachmentType,
                > = None;
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
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = CreateAttachmentRequestDataAttributes {
                    attachment,
                    attachment_type,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CreateAttachmentRequestDataAttributesVisitor)
    }
}
