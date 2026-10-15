// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An inline condition in a saved filter targeting rule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SavedFilterCondition {
    /// The attribute evaluated by the condition.
    #[serde(rename = "attribute")]
    pub attribute: String,
    /// The ID of the condition.
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    /// The comparison operator used by a saved filter condition.
    #[serde(rename = "operator")]
    pub operator: crate::datadogV2::model::SavedFilterConditionOperator,
    /// Values used by the condition operator.
    #[serde(rename = "value")]
    pub value: Vec<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SavedFilterCondition {
    pub fn new(
        attribute: String,
        id: uuid::Uuid,
        operator: crate::datadogV2::model::SavedFilterConditionOperator,
        value: Vec<String>,
    ) -> SavedFilterCondition {
        SavedFilterCondition {
            attribute,
            id,
            operator,
            value,
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

impl<'de> Deserialize<'de> for SavedFilterCondition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SavedFilterConditionVisitor;
        impl<'a> Visitor<'a> for SavedFilterConditionVisitor {
            type Value = SavedFilterCondition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attribute: Option<String> = None;
                let mut id: Option<uuid::Uuid> = None;
                let mut operator: Option<crate::datadogV2::model::SavedFilterConditionOperator> =
                    None;
                let mut value: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "attribute" => {
                            attribute = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "operator" => {
                            operator = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _operator) = operator {
                                match _operator {
                                    crate::datadogV2::model::SavedFilterConditionOperator::UnparsedObject(_operator) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "value" => {
                            value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let attribute = attribute.ok_or_else(|| M::Error::missing_field("attribute"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let operator = operator.ok_or_else(|| M::Error::missing_field("operator"))?;
                let value = value.ok_or_else(|| M::Error::missing_field("value"))?;

                let content = SavedFilterCondition {
                    attribute,
                    id,
                    operator,
                    value,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SavedFilterConditionVisitor)
    }
}
