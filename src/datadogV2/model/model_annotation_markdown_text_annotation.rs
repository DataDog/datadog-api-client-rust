// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `AnnotationMarkdownTextAnnotation` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AnnotationMarkdownTextAnnotation {
    /// The `markdownTextAnnotation` `text`.
    #[serde(rename = "text")]
    pub text: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AnnotationMarkdownTextAnnotation {
    pub fn new() -> AnnotationMarkdownTextAnnotation {
        AnnotationMarkdownTextAnnotation {
            text: None,
            _unparsed: false,
        }
    }

    pub fn text(mut self, value: String) -> Self {
        self.text = Some(value);
        self
    }
}

impl Default for AnnotationMarkdownTextAnnotation {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AnnotationMarkdownTextAnnotation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AnnotationMarkdownTextAnnotationVisitor;
        impl<'a> Visitor<'a> for AnnotationMarkdownTextAnnotationVisitor {
            type Value = AnnotationMarkdownTextAnnotation;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut text: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "text" => {
                            if v.is_null() {
                                continue;
                            }
                            text = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = AnnotationMarkdownTextAnnotation { text, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AnnotationMarkdownTextAnnotationVisitor)
    }
}
