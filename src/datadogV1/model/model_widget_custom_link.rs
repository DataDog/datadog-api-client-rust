// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Custom links help you connect a data value to a URL, like a Datadog page or your AWS console.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct WidgetCustomLink {
    /// The flag for toggling context menu link visibility.
    #[serde(rename = "is_hidden")]
    pub is_hidden: Option<bool>,
    /// The label for the custom link URL. Keep the label short and descriptive. Use metrics and tags as variables.
    #[serde(rename = "label")]
    pub label: Option<String>,
    /// The URL of the custom link. URL must include `http` or `https`. A relative URL must start with `/`.
    #[serde(rename = "link")]
    pub link: Option<String>,
    /// The label ID that refers to a context menu link. Can be `logs`, `hosts`, `traces`, `profiles`, `processes`, `containers`, or `rum`.
    #[serde(rename = "override_label")]
    pub override_label: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl WidgetCustomLink {
    pub fn new() -> WidgetCustomLink {
        WidgetCustomLink {
            is_hidden: None,
            label: None,
            link: None,
            override_label: None,
            _unparsed: false,
        }
    }

    pub fn is_hidden(mut self, value: bool) -> Self {
        self.is_hidden = Some(value);
        self
    }

    pub fn label(mut self, value: String) -> Self {
        self.label = Some(value);
        self
    }

    pub fn link(mut self, value: String) -> Self {
        self.link = Some(value);
        self
    }

    pub fn override_label(mut self, value: String) -> Self {
        self.override_label = Some(value);
        self
    }
}

impl Default for WidgetCustomLink {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for WidgetCustomLink {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct WidgetCustomLinkVisitor;
        impl<'a> Visitor<'a> for WidgetCustomLinkVisitor {
            type Value = WidgetCustomLink;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut is_hidden: Option<bool> = None;
                let mut label: Option<String> = None;
                let mut link: Option<String> = None;
                let mut override_label: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "is_hidden" => {
                            if v.is_null() {
                                continue;
                            }
                            is_hidden = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "label" => {
                            if v.is_null() {
                                continue;
                            }
                            label = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "link" => {
                            if v.is_null() {
                                continue;
                            }
                            link = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "override_label" => {
                            if v.is_null() {
                                continue;
                            }
                            override_label =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = WidgetCustomLink {
                    is_hidden,
                    label,
                    link,
                    override_label,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(WidgetCustomLinkVisitor)
    }
}
