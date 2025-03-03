// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The action query spec object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ActionQuerySpecObject {
    /// The connection group to use for an action query.
    #[serde(rename = "connectionGroup")]
    pub connection_group: Option<crate::datadogV2::model::ActionQuerySpecConnectionGroup>,
    /// The ID of the custom connection to use for this action query.
    #[serde(rename = "connectionId")]
    pub connection_id: Option<String>,
    /// The fully qualified name of the action type.
    #[serde(rename = "fqn")]
    pub fqn: String,
    /// The inputs to the action query. These are the values that are passed to the action when it is triggered.
    #[serde(rename = "inputs")]
    pub inputs: Option<crate::datadogV2::model::ActionQuerySpecInputs>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ActionQuerySpecObject {
    pub fn new(fqn: String) -> ActionQuerySpecObject {
        ActionQuerySpecObject {
            connection_group: None,
            connection_id: None,
            fqn,
            inputs: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn connection_group(
        mut self,
        value: crate::datadogV2::model::ActionQuerySpecConnectionGroup,
    ) -> Self {
        self.connection_group = Some(value);
        self
    }

    pub fn connection_id(mut self, value: String) -> Self {
        self.connection_id = Some(value);
        self
    }

    pub fn inputs(mut self, value: crate::datadogV2::model::ActionQuerySpecInputs) -> Self {
        self.inputs = Some(value);
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

impl<'de> Deserialize<'de> for ActionQuerySpecObject {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ActionQuerySpecObjectVisitor;
        impl<'a> Visitor<'a> for ActionQuerySpecObjectVisitor {
            type Value = ActionQuerySpecObject;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut connection_group: Option<
                    crate::datadogV2::model::ActionQuerySpecConnectionGroup,
                > = None;
                let mut connection_id: Option<String> = None;
                let mut fqn: Option<String> = None;
                let mut inputs: Option<crate::datadogV2::model::ActionQuerySpecInputs> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "connectionGroup" => {
                            if v.is_null() {
                                continue;
                            }
                            connection_group =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "connectionId" => {
                            if v.is_null() {
                                continue;
                            }
                            connection_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "fqn" => {
                            fqn = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "inputs" => {
                            if v.is_null() {
                                continue;
                            }
                            inputs = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _inputs) = inputs {
                                match _inputs {
                                    crate::datadogV2::model::ActionQuerySpecInputs::UnparsedObject(_inputs) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let fqn = fqn.ok_or_else(|| M::Error::missing_field("fqn"))?;

                let content = ActionQuerySpecObject {
                    connection_group,
                    connection_id,
                    fqn,
                    inputs,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ActionQuerySpecObjectVisitor)
    }
}
