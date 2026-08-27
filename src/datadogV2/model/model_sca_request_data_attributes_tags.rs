// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A map of tags providing additional metadata for the SCA scan.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ScaRequestDataAttributesTags {
    /// Tool metadata included in SCA tags.
    #[serde(rename = "tool")]
    pub tool: Option<crate::datadogV2::model::ScaRequestDataAttributesTagsTool>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ScaRequestDataAttributesTags {
    pub fn new() -> ScaRequestDataAttributesTags {
        ScaRequestDataAttributesTags {
            tool: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn tool(
        mut self,
        value: crate::datadogV2::model::ScaRequestDataAttributesTagsTool,
    ) -> Self {
        self.tool = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, String>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl Default for ScaRequestDataAttributesTags {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ScaRequestDataAttributesTags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ScaRequestDataAttributesTagsVisitor;
        impl<'a> Visitor<'a> for ScaRequestDataAttributesTagsVisitor {
            type Value = ScaRequestDataAttributesTags;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut tool: Option<crate::datadogV2::model::ScaRequestDataAttributesTagsTool> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<String, String> =
                    std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "tool" => {
                            if v.is_null() {
                                continue;
                            }
                            tool = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ScaRequestDataAttributesTags {
                    tool,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ScaRequestDataAttributesTagsVisitor)
    }
}
