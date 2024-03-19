// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Modifier of the object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SLOCorrectionResponseAttributesModifier {
    /// Email of the Modifier.
    #[serde(rename = "email")]
    pub email: Option<String>,
    /// Handle of the Modifier.
    #[serde(rename = "handle")]
    pub handle: Option<String>,
    /// Name of the Modifier.
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SLOCorrectionResponseAttributesModifier {
    pub fn new() -> SLOCorrectionResponseAttributesModifier {
        SLOCorrectionResponseAttributesModifier {
            email: None,
            handle: None,
            name: None,
            _unparsed: false,
        }
    }

    pub fn email(mut self, value: String) -> Self {
        self.email = Some(value);
        self
    }

    pub fn handle(mut self, value: String) -> Self {
        self.handle = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }
}

impl Default for SLOCorrectionResponseAttributesModifier {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SLOCorrectionResponseAttributesModifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SLOCorrectionResponseAttributesModifierVisitor;
        impl<'a> Visitor<'a> for SLOCorrectionResponseAttributesModifierVisitor {
            type Value = SLOCorrectionResponseAttributesModifier;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut email: Option<String> = None;
                let mut handle: Option<String> = None;
                let mut name: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "email" => {
                            if v.is_null() {
                                continue;
                            }
                            email = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "handle" => {
                            if v.is_null() {
                                continue;
                            }
                            handle = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SLOCorrectionResponseAttributesModifier {
                    email,
                    handle,
                    name,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SLOCorrectionResponseAttributesModifierVisitor)
    }
}
