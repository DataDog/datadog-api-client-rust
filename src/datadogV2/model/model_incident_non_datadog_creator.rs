// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Incident's non Datadog creator.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentNonDatadogCreator {
    /// Non Datadog creator `48px` image.
    #[serde(rename = "image_48_px")]
    pub image_48_px: Option<String>,
    /// Non Datadog creator name.
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentNonDatadogCreator {
    pub fn new() -> IncidentNonDatadogCreator {
        IncidentNonDatadogCreator {
            image_48_px: None,
            name: None,
            _unparsed: false,
        }
    }

    pub fn image_48_px(&mut self, value: String) -> &mut Self {
        self.image_48_px = Some(value);
        self
    }

    pub fn name(&mut self, value: String) -> &mut Self {
        self.name = Some(value);
        self
    }
}

impl Default for IncidentNonDatadogCreator {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IncidentNonDatadogCreator {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentNonDatadogCreatorVisitor;
        impl<'a> Visitor<'a> for IncidentNonDatadogCreatorVisitor {
            type Value = IncidentNonDatadogCreator;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut image_48_px: Option<String> = None;
                let mut name: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "image_48_px" => {
                            if v.is_null() {
                                continue;
                            }
                            image_48_px =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = IncidentNonDatadogCreator {
                    image_48_px,
                    name,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentNonDatadogCreatorVisitor)
    }
}
