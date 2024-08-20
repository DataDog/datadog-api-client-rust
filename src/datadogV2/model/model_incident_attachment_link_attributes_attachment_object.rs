// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The link attachment.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentAttachmentLinkAttributesAttachmentObject {
    /// The URL of this link attachment.
    #[serde(rename = "documentUrl")]
    pub document_url: String,
    /// The title of this link attachment.
    #[serde(rename = "title")]
    pub title: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentAttachmentLinkAttributesAttachmentObject {
    pub fn new(
        document_url: String,
        title: String,
    ) -> IncidentAttachmentLinkAttributesAttachmentObject {
        IncidentAttachmentLinkAttributesAttachmentObject {
            document_url,
            title,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for IncidentAttachmentLinkAttributesAttachmentObject {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentAttachmentLinkAttributesAttachmentObjectVisitor;
        impl<'a> Visitor<'a> for IncidentAttachmentLinkAttributesAttachmentObjectVisitor {
            type Value = IncidentAttachmentLinkAttributesAttachmentObject;

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
                            document_url =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "title" => {
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let document_url =
                    document_url.ok_or_else(|| M::Error::missing_field("document_url"))?;
                let title = title.ok_or_else(|| M::Error::missing_field("title"))?;

                let content = IncidentAttachmentLinkAttributesAttachmentObject {
                    document_url,
                    title,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentAttachmentLinkAttributesAttachmentObjectVisitor)
    }
}
