// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GlobalVariableJsonPatchRequestDataAttributes {
    /// JSON Patch operations following RFC 6902.
    #[serde(rename = "json_patch")]
    pub json_patch: Option<Vec<crate::datadogV2::model::JsonPatchOperation>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GlobalVariableJsonPatchRequestDataAttributes {
    pub fn new() -> GlobalVariableJsonPatchRequestDataAttributes {
        GlobalVariableJsonPatchRequestDataAttributes {
            json_patch: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn json_patch(mut self, value: Vec<crate::datadogV2::model::JsonPatchOperation>) -> Self {
        self.json_patch = Some(value);
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

impl Default for GlobalVariableJsonPatchRequestDataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for GlobalVariableJsonPatchRequestDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GlobalVariableJsonPatchRequestDataAttributesVisitor;
        impl<'a> Visitor<'a> for GlobalVariableJsonPatchRequestDataAttributesVisitor {
            type Value = GlobalVariableJsonPatchRequestDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut json_patch: Option<Vec<crate::datadogV2::model::JsonPatchOperation>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "json_patch" => {
                            if v.is_null() {
                                continue;
                            }
                            json_patch = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = GlobalVariableJsonPatchRequestDataAttributes {
                    json_patch,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GlobalVariableJsonPatchRequestDataAttributesVisitor)
    }
}
