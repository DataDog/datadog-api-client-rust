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
pub struct DataTransformationContext {
    /// Available context variables for the transformation.
    #[serde(rename = "contextVariables")]
    pub context_variables: String,
    /// The current script to modify or enhance.
    #[serde(rename = "currentScript")]
    pub current_script: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DataTransformationContext {
    pub fn new(context_variables: String, current_script: String) -> DataTransformationContext {
        DataTransformationContext {
            context_variables,
            current_script,
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

impl<'de> Deserialize<'de> for DataTransformationContext {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DataTransformationContextVisitor;
        impl<'a> Visitor<'a> for DataTransformationContextVisitor {
            type Value = DataTransformationContext;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut context_variables: Option<String> = None;
                let mut current_script: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "contextVariables" => {
                            context_variables =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "currentScript" => {
                            current_script =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let context_variables = context_variables
                    .ok_or_else(|| M::Error::missing_field("context_variables"))?;
                let current_script =
                    current_script.ok_or_else(|| M::Error::missing_field("current_script"))?;

                let content = DataTransformationContext {
                    context_variables,
                    current_script,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DataTransformationContextVisitor)
    }
}
