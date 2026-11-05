// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Options that customize the content of the Slack message sent for a Page.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SendSlackMessageActionCustomizableContent {
    /// Whether to include the Page description in the Slack message.
    #[serde(rename = "include_description")]
    pub include_description: Option<bool>,
    /// Whether to include the source that triggered the Page in the Slack message.
    #[serde(rename = "include_source")]
    pub include_source: Option<bool>,
    /// Whether to mirror replies posted in the Slack message thread to the Page timeline.
    #[serde(rename = "mirror_replies")]
    pub mirror_replies: Option<bool>,
    /// Whether to post Page timeline updates as replies in the Slack message thread.
    #[serde(rename = "thread_timeline")]
    pub thread_timeline: Option<bool>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SendSlackMessageActionCustomizableContent {
    pub fn new() -> SendSlackMessageActionCustomizableContent {
        SendSlackMessageActionCustomizableContent {
            include_description: None,
            include_source: None,
            mirror_replies: None,
            thread_timeline: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn include_description(mut self, value: bool) -> Self {
        self.include_description = Some(value);
        self
    }

    pub fn include_source(mut self, value: bool) -> Self {
        self.include_source = Some(value);
        self
    }

    pub fn mirror_replies(mut self, value: bool) -> Self {
        self.mirror_replies = Some(value);
        self
    }

    pub fn thread_timeline(mut self, value: bool) -> Self {
        self.thread_timeline = Some(value);
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

impl Default for SendSlackMessageActionCustomizableContent {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SendSlackMessageActionCustomizableContent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SendSlackMessageActionCustomizableContentVisitor;
        impl<'a> Visitor<'a> for SendSlackMessageActionCustomizableContentVisitor {
            type Value = SendSlackMessageActionCustomizableContent;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut include_description: Option<bool> = None;
                let mut include_source: Option<bool> = None;
                let mut mirror_replies: Option<bool> = None;
                let mut thread_timeline: Option<bool> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "include_description" => {
                            if v.is_null() {
                                continue;
                            }
                            include_description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "include_source" => {
                            if v.is_null() {
                                continue;
                            }
                            include_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mirror_replies" => {
                            if v.is_null() {
                                continue;
                            }
                            mirror_replies =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "thread_timeline" => {
                            if v.is_null() {
                                continue;
                            }
                            thread_timeline =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SendSlackMessageActionCustomizableContent {
                    include_description,
                    include_source,
                    mirror_replies,
                    thread_timeline,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SendSlackMessageActionCustomizableContentVisitor)
    }
}
