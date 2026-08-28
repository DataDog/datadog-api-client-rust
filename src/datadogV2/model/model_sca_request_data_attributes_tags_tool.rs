// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Tool metadata included in SCA tags.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ScaRequestDataAttributesTagsTool {
    /// Metadata about the tool that generated the SCA tags.
    #[serde(rename = "generator")]
    pub generator: Option<crate::datadogV2::model::ScaRequestDataAttributesTagsToolGenerator>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ScaRequestDataAttributesTagsTool {
    pub fn new() -> ScaRequestDataAttributesTagsTool {
        ScaRequestDataAttributesTagsTool {
            generator: None,
            _unparsed: false,
        }
    }

    pub fn generator(
        mut self,
        value: crate::datadogV2::model::ScaRequestDataAttributesTagsToolGenerator,
    ) -> Self {
        self.generator = Some(value);
        self
    }
}

impl Default for ScaRequestDataAttributesTagsTool {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ScaRequestDataAttributesTagsTool {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ScaRequestDataAttributesTagsToolVisitor;
        impl<'a> Visitor<'a> for ScaRequestDataAttributesTagsToolVisitor {
            type Value = ScaRequestDataAttributesTagsTool;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut generator: Option<
                    crate::datadogV2::model::ScaRequestDataAttributesTagsToolGenerator,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "generator" => {
                            if v.is_null() {
                                continue;
                            }
                            generator = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = ScaRequestDataAttributesTagsTool {
                    generator,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ScaRequestDataAttributesTagsToolVisitor)
    }
}
