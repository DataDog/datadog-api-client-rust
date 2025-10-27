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
pub struct GetMultipleRulesetsResponseDataAttributesRulesetsItems {
    #[serde(rename = "data")]
    pub data: crate::datadogV2::model::GetMultipleRulesetsResponseDataAttributesRulesetsItemsData,
    #[serde(rename = "description")]
    pub description: Option<String>,
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "rules")]
    pub rules: Option<Vec<crate::datadogV2::model::GetMultipleRulesetsResponseDataAttributesRulesetsItemsRulesItems>>,
    #[serde(rename = "short_description")]
    pub short_description: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl GetMultipleRulesetsResponseDataAttributesRulesetsItems {
    pub fn new(
        data: crate::datadogV2::model::GetMultipleRulesetsResponseDataAttributesRulesetsItemsData,
    ) -> GetMultipleRulesetsResponseDataAttributesRulesetsItems {
        GetMultipleRulesetsResponseDataAttributesRulesetsItems {
            data,
            description: None,
            name: None,
            rules: None,
            short_description: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn rules(
        mut self,
        value: Vec<crate::datadogV2::model::GetMultipleRulesetsResponseDataAttributesRulesetsItemsRulesItems>,
    ) -> Self {
        self.rules = Some(value);
        self
    }

    pub fn short_description(mut self, value: String) -> Self {
        self.short_description = Some(value);
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

impl<'de> Deserialize<'de> for GetMultipleRulesetsResponseDataAttributesRulesetsItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GetMultipleRulesetsResponseDataAttributesRulesetsItemsVisitor;
        impl<'a> Visitor<'a> for GetMultipleRulesetsResponseDataAttributesRulesetsItemsVisitor {
            type Value = GetMultipleRulesetsResponseDataAttributesRulesetsItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data: Option<crate::datadogV2::model::GetMultipleRulesetsResponseDataAttributesRulesetsItemsData> = None;
                let mut description: Option<String> = None;
                let mut name: Option<String> = None;
                let mut rules: Option<Vec<crate::datadogV2::model::GetMultipleRulesetsResponseDataAttributesRulesetsItemsRulesItems>> = None;
                let mut short_description: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
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
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rules" => {
                            if v.is_null() {
                                continue;
                            }
                            rules = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "short_description" => {
                            if v.is_null() {
                                continue;
                            }
                            short_description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let data = data.ok_or_else(|| M::Error::missing_field("data"))?;

                let content = GetMultipleRulesetsResponseDataAttributesRulesetsItems {
                    data,
                    description,
                    name,
                    rules,
                    short_description,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GetMultipleRulesetsResponseDataAttributesRulesetsItemsVisitor)
    }
}
