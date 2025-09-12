// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `UpdateRulesetRequestDataAttributesRulesItemsReferenceTable` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UpdateRulesetRequestDataAttributesRulesItemsReferenceTable {
    /// The `reference_table` `case_insensitivity`.
    #[serde(rename = "case_insensitivity")]
    pub case_insensitivity: Option<bool>,
    /// The `reference_table` `field_pairs`.
    #[serde(rename = "field_pairs")]
    pub field_pairs: Vec<crate::datadogV2::model::UpdateRulesetRequestDataAttributesRulesItemsReferenceTableFieldPairsItems>,
    /// The `reference_table` `if_not_exists`.
    #[serde(rename = "if_not_exists")]
    pub if_not_exists: Option<bool>,
    /// The `reference_table` `source_keys`.
    #[serde(rename = "source_keys")]
    pub source_keys: Vec<String>,
    /// The `reference_table` `table_name`.
    #[serde(rename = "table_name")]
    pub table_name: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl UpdateRulesetRequestDataAttributesRulesItemsReferenceTable {
    pub fn new(
        field_pairs: Vec<crate::datadogV2::model::UpdateRulesetRequestDataAttributesRulesItemsReferenceTableFieldPairsItems>,
        source_keys: Vec<String>,
        table_name: String,
    ) -> UpdateRulesetRequestDataAttributesRulesItemsReferenceTable {
        UpdateRulesetRequestDataAttributesRulesItemsReferenceTable {
            case_insensitivity: None,
            field_pairs,
            if_not_exists: None,
            source_keys,
            table_name,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn case_insensitivity(mut self, value: bool) -> Self {
        self.case_insensitivity = Some(value);
        self
    }

    pub fn if_not_exists(mut self, value: bool) -> Self {
        self.if_not_exists = Some(value);
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

impl<'de> Deserialize<'de> for UpdateRulesetRequestDataAttributesRulesItemsReferenceTable {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UpdateRulesetRequestDataAttributesRulesItemsReferenceTableVisitor;
        impl<'a> Visitor<'a> for UpdateRulesetRequestDataAttributesRulesItemsReferenceTableVisitor {
            type Value = UpdateRulesetRequestDataAttributesRulesItemsReferenceTable;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut case_insensitivity: Option<bool> = None;
                let mut field_pairs: Option<Vec<crate::datadogV2::model::UpdateRulesetRequestDataAttributesRulesItemsReferenceTableFieldPairsItems>> = None;
                let mut if_not_exists: Option<bool> = None;
                let mut source_keys: Option<Vec<String>> = None;
                let mut table_name: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "case_insensitivity" => {
                            if v.is_null() {
                                continue;
                            }
                            case_insensitivity =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "field_pairs" => {
                            field_pairs =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "if_not_exists" => {
                            if v.is_null() {
                                continue;
                            }
                            if_not_exists =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source_keys" => {
                            source_keys =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "table_name" => {
                            table_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let field_pairs =
                    field_pairs.ok_or_else(|| M::Error::missing_field("field_pairs"))?;
                let source_keys =
                    source_keys.ok_or_else(|| M::Error::missing_field("source_keys"))?;
                let table_name = table_name.ok_or_else(|| M::Error::missing_field("table_name"))?;

                let content = UpdateRulesetRequestDataAttributesRulesItemsReferenceTable {
                    case_insensitivity,
                    field_pairs,
                    if_not_exists,
                    source_keys,
                    table_name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(UpdateRulesetRequestDataAttributesRulesItemsReferenceTableVisitor)
    }
}
