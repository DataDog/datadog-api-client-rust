// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The Statuspage URL setting attributes for a create request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct StatuspageUrlSettingCreateAttributes {
    /// Comma-separated list of custom tags to apply to events generated from this Statuspage URL.
    #[serde(rename = "custom_tags")]
    pub custom_tags: String,
    /// The Statuspage URL to monitor. Must be a `status.io` or `statuspage.com` URL.
    #[serde(rename = "url")]
    pub url: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl StatuspageUrlSettingCreateAttributes {
    pub fn new(custom_tags: String, url: String) -> StatuspageUrlSettingCreateAttributes {
        StatuspageUrlSettingCreateAttributes {
            custom_tags,
            url,
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

impl<'de> Deserialize<'de> for StatuspageUrlSettingCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StatuspageUrlSettingCreateAttributesVisitor;
        impl<'a> Visitor<'a> for StatuspageUrlSettingCreateAttributesVisitor {
            type Value = StatuspageUrlSettingCreateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut custom_tags: Option<String> = None;
                let mut url: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "custom_tags" => {
                            custom_tags =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "url" => {
                            url = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let custom_tags =
                    custom_tags.ok_or_else(|| M::Error::missing_field("custom_tags"))?;
                let url = url.ok_or_else(|| M::Error::missing_field("url"))?;

                let content = StatuspageUrlSettingCreateAttributes {
                    custom_tags,
                    url,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(StatuspageUrlSettingCreateAttributesVisitor)
    }
}
