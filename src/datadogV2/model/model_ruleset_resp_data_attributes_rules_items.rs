// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `RulesetRespDataAttributesRulesItems` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RulesetRespDataAttributesRulesItems {
    /// The `items` `enabled`.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// The definition of `RulesetRespDataAttributesRulesItemsMapping` object.
    #[serde(
        rename = "mapping",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub mapping:
        Option<Option<crate::datadogV2::model::RulesetRespDataAttributesRulesItemsMapping>>,
    /// The `items` `metadata`.
    #[serde(
        rename = "metadata",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub metadata: Option<Option<std::collections::BTreeMap<String, String>>>,
    /// The `items` `name`.
    #[serde(rename = "name")]
    pub name: String,
    /// The definition of `RulesetRespDataAttributesRulesItemsQuery` object.
    #[serde(rename = "query", default, with = "::serde_with::rust::double_option")]
    pub query: Option<Option<crate::datadogV2::model::RulesetRespDataAttributesRulesItemsQuery>>,
    /// The definition of `RulesetRespDataAttributesRulesItemsReferenceTable` object.
    #[serde(
        rename = "reference_table",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub reference_table:
        Option<Option<crate::datadogV2::model::RulesetRespDataAttributesRulesItemsReferenceTable>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RulesetRespDataAttributesRulesItems {
    pub fn new(enabled: bool, name: String) -> RulesetRespDataAttributesRulesItems {
        RulesetRespDataAttributesRulesItems {
            enabled,
            mapping: None,
            metadata: None,
            name,
            query: None,
            reference_table: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn mapping(
        mut self,
        value: Option<crate::datadogV2::model::RulesetRespDataAttributesRulesItemsMapping>,
    ) -> Self {
        self.mapping = Some(value);
        self
    }

    pub fn metadata(mut self, value: Option<std::collections::BTreeMap<String, String>>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn query(
        mut self,
        value: Option<crate::datadogV2::model::RulesetRespDataAttributesRulesItemsQuery>,
    ) -> Self {
        self.query = Some(value);
        self
    }

    pub fn reference_table(
        mut self,
        value: Option<crate::datadogV2::model::RulesetRespDataAttributesRulesItemsReferenceTable>,
    ) -> Self {
        self.reference_table = Some(value);
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

impl<'de> Deserialize<'de> for RulesetRespDataAttributesRulesItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RulesetRespDataAttributesRulesItemsVisitor;
        impl<'a> Visitor<'a> for RulesetRespDataAttributesRulesItemsVisitor {
            type Value = RulesetRespDataAttributesRulesItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut enabled: Option<bool> = None;
                let mut mapping: Option<
                    Option<crate::datadogV2::model::RulesetRespDataAttributesRulesItemsMapping>,
                > = None;
                let mut metadata: Option<Option<std::collections::BTreeMap<String, String>>> = None;
                let mut name: Option<String> = None;
                let mut query: Option<
                    Option<crate::datadogV2::model::RulesetRespDataAttributesRulesItemsQuery>,
                > = None;
                let mut reference_table: Option<
                    Option<
                        crate::datadogV2::model::RulesetRespDataAttributesRulesItemsReferenceTable,
                    >,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "enabled" => {
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mapping" => {
                            mapping = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metadata" => {
                            metadata = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "reference_table" => {
                            reference_table =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let enabled = enabled.ok_or_else(|| M::Error::missing_field("enabled"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = RulesetRespDataAttributesRulesItems {
                    enabled,
                    mapping,
                    metadata,
                    name,
                    query,
                    reference_table,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RulesetRespDataAttributesRulesItemsVisitor)
    }
}
