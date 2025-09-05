// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes for creating a notification template.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentNotificationTemplateCreateAttributes {
    /// The category of the notification template.
    #[serde(rename = "category")]
    pub category: String,
    /// The content body of the notification template.
    #[serde(rename = "content")]
    pub content: String,
    /// The name of the notification template.
    #[serde(rename = "name")]
    pub name: String,
    /// The subject line of the notification template.
    #[serde(rename = "subject")]
    pub subject: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentNotificationTemplateCreateAttributes {
    pub fn new(
        category: String,
        content: String,
        name: String,
        subject: String,
    ) -> IncidentNotificationTemplateCreateAttributes {
        IncidentNotificationTemplateCreateAttributes {
            category,
            content,
            name,
            subject,
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

impl<'de> Deserialize<'de> for IncidentNotificationTemplateCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentNotificationTemplateCreateAttributesVisitor;
        impl<'a> Visitor<'a> for IncidentNotificationTemplateCreateAttributesVisitor {
            type Value = IncidentNotificationTemplateCreateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut category: Option<String> = None;
                let mut content: Option<String> = None;
                let mut name: Option<String> = None;
                let mut subject: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "category" => {
                            category = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "content" => {
                            content = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "subject" => {
                            subject = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let category = category.ok_or_else(|| M::Error::missing_field("category"))?;
                let content = content.ok_or_else(|| M::Error::missing_field("content"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let subject = subject.ok_or_else(|| M::Error::missing_field("subject"))?;

                let content = IncidentNotificationTemplateCreateAttributes {
                    category,
                    content,
                    name,
                    subject,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentNotificationTemplateCreateAttributesVisitor)
    }
}
