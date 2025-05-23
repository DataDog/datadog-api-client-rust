// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Details about the On-Call Page you want to create.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CreatePageRequestDataAttributes {
    /// A short summary of the issue or context.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Tags to help categorize or filter the page.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// Information about the target to notify (such as a team or user).
    #[serde(rename = "target")]
    pub target: crate::datadogV2::model::CreatePageRequestDataAttributesTarget,
    /// The title of the page.
    #[serde(rename = "title")]
    pub title: String,
    /// On-Call Page urgency level.
    #[serde(rename = "urgency")]
    pub urgency: crate::datadogV2::model::PageUrgency,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CreatePageRequestDataAttributes {
    pub fn new(
        target: crate::datadogV2::model::CreatePageRequestDataAttributesTarget,
        title: String,
        urgency: crate::datadogV2::model::PageUrgency,
    ) -> CreatePageRequestDataAttributes {
        CreatePageRequestDataAttributes {
            description: None,
            tags: None,
            target,
            title,
            urgency,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
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

impl<'de> Deserialize<'de> for CreatePageRequestDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CreatePageRequestDataAttributesVisitor;
        impl<'a> Visitor<'a> for CreatePageRequestDataAttributesVisitor {
            type Value = CreatePageRequestDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut description: Option<String> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut target: Option<
                    crate::datadogV2::model::CreatePageRequestDataAttributesTarget,
                > = None;
                let mut title: Option<String> = None;
                let mut urgency: Option<crate::datadogV2::model::PageUrgency> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "target" => {
                            target = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "title" => {
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "urgency" => {
                            urgency = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _urgency) = urgency {
                                match _urgency {
                                    crate::datadogV2::model::PageUrgency::UnparsedObject(
                                        _urgency,
                                    ) => {
                                        _unparsed = true;
                                    }
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
                let target = target.ok_or_else(|| M::Error::missing_field("target"))?;
                let title = title.ok_or_else(|| M::Error::missing_field("title"))?;
                let urgency = urgency.ok_or_else(|| M::Error::missing_field("urgency"))?;

                let content = CreatePageRequestDataAttributes {
                    description,
                    tags,
                    target,
                    title,
                    urgency,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CreatePageRequestDataAttributesVisitor)
    }
}
