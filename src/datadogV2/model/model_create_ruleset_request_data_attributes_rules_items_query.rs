// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `CreateRulesetRequestDataAttributesRulesItemsQuery` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CreateRulesetRequestDataAttributesRulesItemsQuery {
    /// The definition of `CreateRulesetRequestDataAttributesRulesItemsQueryAddition` object.
    #[serialize_always]
    #[serde(rename = "addition")]
    pub addition:
        Option<crate::datadogV2::model::CreateRulesetRequestDataAttributesRulesItemsQueryAddition>,
    /// The `query` `case_insensitivity`.
    #[serde(rename = "case_insensitivity")]
    pub case_insensitivity: Option<bool>,
    /// Deprecated. Use `if_tag_exists` instead. The `query` `if_not_exists`.
    #[deprecated]
    #[serde(rename = "if_not_exists")]
    pub if_not_exists: Option<bool>,
    /// The behavior when the tag already exists.
    #[serde(rename = "if_tag_exists")]
    pub if_tag_exists: Option<crate::datadogV2::model::DataAttributesRulesItemsIfTagExists>,
    /// The `query` `query`.
    #[serde(rename = "query")]
    pub query: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CreateRulesetRequestDataAttributesRulesItemsQuery {
    pub fn new(
        addition: Option<
            crate::datadogV2::model::CreateRulesetRequestDataAttributesRulesItemsQueryAddition,
        >,
        query: String,
    ) -> CreateRulesetRequestDataAttributesRulesItemsQuery {
        #[allow(deprecated)]
        CreateRulesetRequestDataAttributesRulesItemsQuery {
            addition,
            case_insensitivity: None,
            if_not_exists: None,
            if_tag_exists: None,
            query,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    #[allow(deprecated)]
    pub fn case_insensitivity(mut self, value: bool) -> Self {
        self.case_insensitivity = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn if_not_exists(mut self, value: bool) -> Self {
        self.if_not_exists = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn if_tag_exists(
        mut self,
        value: crate::datadogV2::model::DataAttributesRulesItemsIfTagExists,
    ) -> Self {
        self.if_tag_exists = Some(value);
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

impl<'de> Deserialize<'de> for CreateRulesetRequestDataAttributesRulesItemsQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CreateRulesetRequestDataAttributesRulesItemsQueryVisitor;
        impl<'a> Visitor<'a> for CreateRulesetRequestDataAttributesRulesItemsQueryVisitor {
            type Value = CreateRulesetRequestDataAttributesRulesItemsQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut addition: Option<Option<crate::datadogV2::model::CreateRulesetRequestDataAttributesRulesItemsQueryAddition>> = None;
                let mut case_insensitivity: Option<bool> = None;
                let mut if_not_exists: Option<bool> = None;
                let mut if_tag_exists: Option<
                    crate::datadogV2::model::DataAttributesRulesItemsIfTagExists,
                > = None;
                let mut query: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "addition" => {
                            addition = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "case_insensitivity" => {
                            if v.is_null() {
                                continue;
                            }
                            case_insensitivity =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "if_not_exists" => {
                            if v.is_null() {
                                continue;
                            }
                            if_not_exists =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "if_tag_exists" => {
                            if v.is_null() {
                                continue;
                            }
                            if_tag_exists =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _if_tag_exists) = if_tag_exists {
                                match _if_tag_exists {
                                    crate::datadogV2::model::DataAttributesRulesItemsIfTagExists::UnparsedObject(_if_tag_exists) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "query" => {
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let addition = addition.ok_or_else(|| M::Error::missing_field("addition"))?;
                let query = query.ok_or_else(|| M::Error::missing_field("query"))?;

                #[allow(deprecated)]
                let content = CreateRulesetRequestDataAttributesRulesItemsQuery {
                    addition,
                    case_insensitivity,
                    if_not_exists,
                    if_tag_exists,
                    query,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CreateRulesetRequestDataAttributesRulesItemsQueryVisitor)
    }
}
