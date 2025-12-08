// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CreateAttachmentRequestDataAttributesAttachment {
    #[serde(rename = "documentUrl")]
    pub document_url: Option<String>,
    #[serde(rename = "title")]
    pub title: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CreateAttachmentRequestDataAttributesAttachment {
    pub fn new() -> CreateAttachmentRequestDataAttributesAttachment {
        CreateAttachmentRequestDataAttributesAttachment {
            document_url: None,
            title: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn document_url(mut self, value: String) -> Self {
        self.document_url = Some(value);
        self
    }

    pub fn title(mut self, value: String) -> Self {
        self.title = Some(value);
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

impl Default for CreateAttachmentRequestDataAttributesAttachment {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CreateAttachmentRequestDataAttributesAttachment {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CreateAttachmentRequestDataAttributesAttachmentVisitor;
        impl<'a> Visitor<'a> for CreateAttachmentRequestDataAttributesAttachmentVisitor {
            type Value = CreateAttachmentRequestDataAttributesAttachment;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut document_url: Option<String> = None;
                let mut title: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "documentUrl" => {
                            if v.is_null() {
                                continue;
                            }
                            document_url =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "title" => {
                            if v.is_null() {
                                continue;
                            }
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = CreateAttachmentRequestDataAttributesAttachment {
                    document_url,
                    title,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CreateAttachmentRequestDataAttributesAttachmentVisitor)
    }
}
