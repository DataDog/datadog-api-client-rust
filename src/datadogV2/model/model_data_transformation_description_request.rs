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
pub struct DataTransformationDescriptionRequest {
    /// The fully qualified name (FQN) of the action.
    #[serde(rename = "actionId")]
    pub action_id: String,
    /// The transformation script to describe.
    #[serde(rename = "script")]
    pub script: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DataTransformationDescriptionRequest {
    pub fn new(action_id: String, script: String) -> DataTransformationDescriptionRequest {
        DataTransformationDescriptionRequest {
            action_id,
            script,
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

impl<'de> Deserialize<'de> for DataTransformationDescriptionRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DataTransformationDescriptionRequestVisitor;
        impl<'a> Visitor<'a> for DataTransformationDescriptionRequestVisitor {
            type Value = DataTransformationDescriptionRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut action_id: Option<String> = None;
                let mut script: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "actionId" => {
                            action_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "script" => {
                            script = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let action_id = action_id.ok_or_else(|| M::Error::missing_field("action_id"))?;
                let script = script.ok_or_else(|| M::Error::missing_field("script"))?;

                let content = DataTransformationDescriptionRequest {
                    action_id,
                    script,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DataTransformationDescriptionRequestVisitor)
    }
}
