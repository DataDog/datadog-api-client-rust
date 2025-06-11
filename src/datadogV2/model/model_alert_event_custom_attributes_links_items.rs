// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A link.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AlertEventCustomAttributesLinksItems {
    /// The category of the link.
    #[serde(rename = "category")]
    pub category: crate::datadogV2::model::AlertEventCustomAttributesLinksItemsCategory,
    /// The title of the link. Limited to 300 characters.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// The URL of the link. Limited to 2048 characters.
    #[serde(rename = "url")]
    pub url: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AlertEventCustomAttributesLinksItems {
    pub fn new(
        category: crate::datadogV2::model::AlertEventCustomAttributesLinksItemsCategory,
        url: String,
    ) -> AlertEventCustomAttributesLinksItems {
        AlertEventCustomAttributesLinksItems {
            category,
            title: None,
            url,
            _unparsed: false,
        }
    }

    pub fn title(mut self, value: String) -> Self {
        self.title = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for AlertEventCustomAttributesLinksItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AlertEventCustomAttributesLinksItemsVisitor;
        impl<'a> Visitor<'a> for AlertEventCustomAttributesLinksItemsVisitor {
            type Value = AlertEventCustomAttributesLinksItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut category: Option<
                    crate::datadogV2::model::AlertEventCustomAttributesLinksItemsCategory,
                > = None;
                let mut title: Option<String> = None;
                let mut url: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "category" => {
                            category = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _category) = category {
                                match _category {
                                    crate::datadogV2::model::AlertEventCustomAttributesLinksItemsCategory::UnparsedObject(_category) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "title" => {
                            if v.is_null() {
                                continue;
                            }
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "url" => {
                            url = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let category = category.ok_or_else(|| M::Error::missing_field("category"))?;
                let url = url.ok_or_else(|| M::Error::missing_field("url"))?;

                let content = AlertEventCustomAttributesLinksItems {
                    category,
                    title,
                    url,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AlertEventCustomAttributesLinksItemsVisitor)
    }
}
