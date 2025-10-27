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
pub struct GetMultipleRulesetsResponseDataAttributesRulesetsItemsRulesItems {
    #[serde(rename = "arguments")]
    pub arguments: Option<Vec<crate::datadogV2::model::GetMultipleRulesetsResponseDataAttributesRulesetsItemsRulesItemsArgumentsItems>>,
    #[serde(rename = "category")]
    pub category: Option<String>,
    #[serde(rename = "checksum")]
    pub checksum: Option<String>,
    #[serde(rename = "code")]
    pub code: Option<String>,
    #[serde(rename = "created_at")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "created_by")]
    pub created_by: Option<String>,
    #[serde(rename = "cve")]
    pub cve: Option<String>,
    #[serde(rename = "cwe")]
    pub cwe: Option<String>,
    #[serde(rename = "data")]
    pub data: crate::datadogV2::model::GetMultipleRulesetsResponseDataAttributesRulesetsItemsRulesItemsData,
    #[serde(rename = "description")]
    pub description: Option<String>,
    #[serde(rename = "documentation_url")]
    pub documentation_url: Option<String>,
    #[serde(rename = "entity_checked")]
    pub entity_checked: Option<String>,
    #[serde(rename = "is_published")]
    pub is_published: Option<bool>,
    #[serde(rename = "is_testing")]
    pub is_testing: Option<bool>,
    #[serde(rename = "language")]
    pub language: Option<String>,
    #[serde(rename = "last_updated_at")]
    pub last_updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "last_updated_by")]
    pub last_updated_by: Option<String>,
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "regex")]
    pub regex: Option<String>,
    #[serde(rename = "severity")]
    pub severity: Option<String>,
    #[serde(rename = "short_description")]
    pub short_description: Option<String>,
    #[serde(rename = "should_use_ai_fix")]
    pub should_use_ai_fix: Option<bool>,
    #[serde(rename = "tests")]
    pub tests: Option<Vec<crate::datadogV2::model::GetMultipleRulesetsResponseDataAttributesRulesetsItemsRulesItemsTestsItems>>,
    #[serde(rename = "tree_sitter_query")]
    pub tree_sitter_query: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl GetMultipleRulesetsResponseDataAttributesRulesetsItemsRulesItems {
    pub fn new(
        data: crate::datadogV2::model::GetMultipleRulesetsResponseDataAttributesRulesetsItemsRulesItemsData,
    ) -> GetMultipleRulesetsResponseDataAttributesRulesetsItemsRulesItems {
        GetMultipleRulesetsResponseDataAttributesRulesetsItemsRulesItems {
            arguments: None,
            category: None,
            checksum: None,
            code: None,
            created_at: None,
            created_by: None,
            cve: None,
            cwe: None,
            data,
            description: None,
            documentation_url: None,
            entity_checked: None,
            is_published: None,
            is_testing: None,
            language: None,
            last_updated_at: None,
            last_updated_by: None,
            name: None,
            regex: None,
            severity: None,
            short_description: None,
            should_use_ai_fix: None,
            tests: None,
            tree_sitter_query: None,
            type_: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn arguments(
        mut self,
        value: Vec<crate::datadogV2::model::GetMultipleRulesetsResponseDataAttributesRulesetsItemsRulesItemsArgumentsItems>,
    ) -> Self {
        self.arguments = Some(value);
        self
    }

    pub fn category(mut self, value: String) -> Self {
        self.category = Some(value);
        self
    }

    pub fn checksum(mut self, value: String) -> Self {
        self.checksum = Some(value);
        self
    }

    pub fn code(mut self, value: String) -> Self {
        self.code = Some(value);
        self
    }

    pub fn created_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn created_by(mut self, value: String) -> Self {
        self.created_by = Some(value);
        self
    }

    pub fn cve(mut self, value: String) -> Self {
        self.cve = Some(value);
        self
    }

    pub fn cwe(mut self, value: String) -> Self {
        self.cwe = Some(value);
        self
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn documentation_url(mut self, value: String) -> Self {
        self.documentation_url = Some(value);
        self
    }

    pub fn entity_checked(mut self, value: String) -> Self {
        self.entity_checked = Some(value);
        self
    }

    pub fn is_published(mut self, value: bool) -> Self {
        self.is_published = Some(value);
        self
    }

    pub fn is_testing(mut self, value: bool) -> Self {
        self.is_testing = Some(value);
        self
    }

    pub fn language(mut self, value: String) -> Self {
        self.language = Some(value);
        self
    }

    pub fn last_updated_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.last_updated_at = Some(value);
        self
    }

    pub fn last_updated_by(mut self, value: String) -> Self {
        self.last_updated_by = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn regex(mut self, value: String) -> Self {
        self.regex = Some(value);
        self
    }

    pub fn severity(mut self, value: String) -> Self {
        self.severity = Some(value);
        self
    }

    pub fn short_description(mut self, value: String) -> Self {
        self.short_description = Some(value);
        self
    }

    pub fn should_use_ai_fix(mut self, value: bool) -> Self {
        self.should_use_ai_fix = Some(value);
        self
    }

    pub fn tests(
        mut self,
        value: Vec<crate::datadogV2::model::GetMultipleRulesetsResponseDataAttributesRulesetsItemsRulesItemsTestsItems>,
    ) -> Self {
        self.tests = Some(value);
        self
    }

    pub fn tree_sitter_query(mut self, value: String) -> Self {
        self.tree_sitter_query = Some(value);
        self
    }

    pub fn type_(mut self, value: String) -> Self {
        self.type_ = Some(value);
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

impl<'de> Deserialize<'de> for GetMultipleRulesetsResponseDataAttributesRulesetsItemsRulesItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GetMultipleRulesetsResponseDataAttributesRulesetsItemsRulesItemsVisitor;
        impl<'a> Visitor<'a> for GetMultipleRulesetsResponseDataAttributesRulesetsItemsRulesItemsVisitor {
            type Value = GetMultipleRulesetsResponseDataAttributesRulesetsItemsRulesItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut arguments: Option<Vec<crate::datadogV2::model::GetMultipleRulesetsResponseDataAttributesRulesetsItemsRulesItemsArgumentsItems>> = None;
                let mut category: Option<String> = None;
                let mut checksum: Option<String> = None;
                let mut code: Option<String> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut created_by: Option<String> = None;
                let mut cve: Option<String> = None;
                let mut cwe: Option<String> = None;
                let mut data: Option<crate::datadogV2::model::GetMultipleRulesetsResponseDataAttributesRulesetsItemsRulesItemsData> = None;
                let mut description: Option<String> = None;
                let mut documentation_url: Option<String> = None;
                let mut entity_checked: Option<String> = None;
                let mut is_published: Option<bool> = None;
                let mut is_testing: Option<bool> = None;
                let mut language: Option<String> = None;
                let mut last_updated_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut last_updated_by: Option<String> = None;
                let mut name: Option<String> = None;
                let mut regex: Option<String> = None;
                let mut severity: Option<String> = None;
                let mut short_description: Option<String> = None;
                let mut should_use_ai_fix: Option<bool> = None;
                let mut tests: Option<Vec<crate::datadogV2::model::GetMultipleRulesetsResponseDataAttributesRulesetsItemsRulesItemsTestsItems>> = None;
                let mut tree_sitter_query: Option<String> = None;
                let mut type_: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "arguments" => {
                            if v.is_null() {
                                continue;
                            }
                            arguments = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "category" => {
                            if v.is_null() {
                                continue;
                            }
                            category = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "checksum" => {
                            if v.is_null() {
                                continue;
                            }
                            checksum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "code" => {
                            if v.is_null() {
                                continue;
                            }
                            code = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_by" => {
                            if v.is_null() {
                                continue;
                            }
                            created_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cve" => {
                            if v.is_null() {
                                continue;
                            }
                            cve = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cwe" => {
                            if v.is_null() {
                                continue;
                            }
                            cwe = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "data" => {
                            data = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "documentation_url" => {
                            if v.is_null() {
                                continue;
                            }
                            documentation_url =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "entity_checked" => {
                            if v.is_null() {
                                continue;
                            }
                            entity_checked =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_published" => {
                            if v.is_null() {
                                continue;
                            }
                            is_published =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_testing" => {
                            if v.is_null() {
                                continue;
                            }
                            is_testing = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "language" => {
                            if v.is_null() {
                                continue;
                            }
                            language = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_updated_at" => {
                            if v.is_null() {
                                continue;
                            }
                            last_updated_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_updated_by" => {
                            if v.is_null() {
                                continue;
                            }
                            last_updated_by =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "regex" => {
                            if v.is_null() {
                                continue;
                            }
                            regex = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "severity" => {
                            if v.is_null() {
                                continue;
                            }
                            severity = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "short_description" => {
                            if v.is_null() {
                                continue;
                            }
                            short_description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "should_use_ai_fix" => {
                            if v.is_null() {
                                continue;
                            }
                            should_use_ai_fix =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tests" => {
                            if v.is_null() {
                                continue;
                            }
                            tests = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tree_sitter_query" => {
                            if v.is_null() {
                                continue;
                            }
                            tree_sitter_query =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let data = data.ok_or_else(|| M::Error::missing_field("data"))?;

                let content = GetMultipleRulesetsResponseDataAttributesRulesetsItemsRulesItems {
                    arguments,
                    category,
                    checksum,
                    code,
                    created_at,
                    created_by,
                    cve,
                    cwe,
                    data,
                    description,
                    documentation_url,
                    entity_checked,
                    is_published,
                    is_testing,
                    language,
                    last_updated_at,
                    last_updated_by,
                    name,
                    regex,
                    severity,
                    short_description,
                    should_use_ai_fix,
                    tests,
                    tree_sitter_query,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(
            GetMultipleRulesetsResponseDataAttributesRulesetsItemsRulesItemsVisitor,
        )
    }
}
