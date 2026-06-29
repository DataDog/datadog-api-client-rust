// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Targeting condition details. A condition is either an inline
/// predicate with `operator`, `attribute`, and `value`, or a reference to a
/// saved filter with `saved_filter_id`. The inline fields are omitted for saved-filter
/// references.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Condition {
    /// The user or request attribute to evaluate. Omitted for saved-filter references.
    #[serde(rename = "attribute")]
    pub attribute: Option<String>,
    /// The timestamp when the condition was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// The unique identifier of the condition.
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    /// The operator used in a targeting condition.
    #[serde(rename = "operator")]
    pub operator: Option<crate::datadogV2::model::ConditionOperator>,
    /// The ID of the saved filter referenced by this condition, or null for inline conditions.
    #[serde(
        rename = "saved_filter_id",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub saved_filter_id: Option<Option<uuid::Uuid>>,
    /// The timestamp when the condition was last updated.
    #[serde(rename = "updated_at")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// Values used by the selected operator. Omitted for saved-filter references.
    #[serde(rename = "value")]
    pub value: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl Condition {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        id: uuid::Uuid,
        updated_at: chrono::DateTime<chrono::Utc>,
    ) -> Condition {
        Condition {
            attribute: None,
            created_at,
            id,
            operator: None,
            saved_filter_id: None,
            updated_at,
            value: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn attribute(mut self, value: String) -> Self {
        self.attribute = Some(value);
        self
    }

    pub fn operator(mut self, value: crate::datadogV2::model::ConditionOperator) -> Self {
        self.operator = Some(value);
        self
    }

    pub fn saved_filter_id(mut self, value: Option<uuid::Uuid>) -> Self {
        self.saved_filter_id = Some(value);
        self
    }

    pub fn value(mut self, value: Vec<String>) -> Self {
        self.value = Some(value);
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

impl<'de> Deserialize<'de> for Condition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ConditionVisitor;
        impl<'a> Visitor<'a> for ConditionVisitor {
            type Value = Condition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attribute: Option<String> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut id: Option<uuid::Uuid> = None;
                let mut operator: Option<crate::datadogV2::model::ConditionOperator> = None;
                let mut saved_filter_id: Option<Option<uuid::Uuid>> = None;
                let mut updated_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut value: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "attribute" => {
                            if v.is_null() {
                                continue;
                            }
                            attribute = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "operator" => {
                            if v.is_null() {
                                continue;
                            }
                            operator = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _operator) = operator {
                                match _operator {
                                    crate::datadogV2::model::ConditionOperator::UnparsedObject(
                                        _operator,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "saved_filter_id" => {
                            saved_filter_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_at" => {
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "value" => {
                            if v.is_null() {
                                continue;
                            }
                            value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let updated_at = updated_at.ok_or_else(|| M::Error::missing_field("updated_at"))?;

                let content = Condition {
                    attribute,
                    created_at,
                    id,
                    operator,
                    saved_filter_id,
                    updated_at,
                    value,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ConditionVisitor)
    }
}
