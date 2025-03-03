// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The mocked outputs of the action query.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ActionQueryMockedOutputsObject {
    /// Whether to enable the mocked outputs for testing.
    #[serde(rename = "enabled")]
    pub enabled: crate::datadogV2::model::ActionQueryMockedOutputsEnabled,
    /// The mocked outputs of the action query, serialized as JSON.
    #[serde(rename = "outputs")]
    pub outputs: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ActionQueryMockedOutputsObject {
    pub fn new(
        enabled: crate::datadogV2::model::ActionQueryMockedOutputsEnabled,
    ) -> ActionQueryMockedOutputsObject {
        ActionQueryMockedOutputsObject {
            enabled,
            outputs: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn outputs(mut self, value: String) -> Self {
        self.outputs = Some(value);
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

impl<'de> Deserialize<'de> for ActionQueryMockedOutputsObject {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ActionQueryMockedOutputsObjectVisitor;
        impl<'a> Visitor<'a> for ActionQueryMockedOutputsObjectVisitor {
            type Value = ActionQueryMockedOutputsObject;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut enabled: Option<crate::datadogV2::model::ActionQueryMockedOutputsEnabled> =
                    None;
                let mut outputs: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "enabled" => {
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _enabled) = enabled {
                                match _enabled {
                                    crate::datadogV2::model::ActionQueryMockedOutputsEnabled::UnparsedObject(_enabled) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "outputs" => {
                            if v.is_null() {
                                continue;
                            }
                            outputs = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let enabled = enabled.ok_or_else(|| M::Error::missing_field("enabled"))?;

                let content = ActionQueryMockedOutputsObject {
                    enabled,
                    outputs,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ActionQueryMockedOutputsObjectVisitor)
    }
}
