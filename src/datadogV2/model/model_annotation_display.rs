// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `AnnotationDisplay` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AnnotationDisplay {
    /// The definition of `AnnotationDisplayBounds` object.
    #[serde(rename = "bounds")]
    pub bounds: Option<crate::datadogV2::model::AnnotationDisplayBounds>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AnnotationDisplay {
    pub fn new() -> AnnotationDisplay {
        AnnotationDisplay {
            bounds: None,
            _unparsed: false,
        }
    }

    pub fn bounds(mut self, value: crate::datadogV2::model::AnnotationDisplayBounds) -> Self {
        self.bounds = Some(value);
        self
    }
}

impl Default for AnnotationDisplay {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AnnotationDisplay {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AnnotationDisplayVisitor;
        impl<'a> Visitor<'a> for AnnotationDisplayVisitor {
            type Value = AnnotationDisplay;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut bounds: Option<crate::datadogV2::model::AnnotationDisplayBounds> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "bounds" => {
                            if v.is_null() {
                                continue;
                            }
                            bounds = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = AnnotationDisplay { bounds, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AnnotationDisplayVisitor)
    }
}
