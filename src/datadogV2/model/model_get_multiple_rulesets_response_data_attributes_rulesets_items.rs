// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A ruleset returned in the response, containing its metadata and associated rules.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GetMultipleRulesetsResponseDataAttributesRulesetsItems {
    /// A detailed description of the ruleset's purpose and the types of issues it targets.
    #[serde(rename = "description")]
    pub description: String,
    /// The unique identifier of the ruleset, which is the same as its name.
    #[serde(rename = "id")]
    pub id: String,
    /// The unique name of the ruleset.
    #[serde(rename = "name")]
    pub name: String,
    /// The list of static analysis rules included in this ruleset.
    #[serde(rename = "rules")]
    pub rules: Vec<
        crate::datadogV2::model::GetMultipleRulesetsResponseDataAttributesRulesetsItemsRulesItems,
    >,
    /// A brief summary of the ruleset, suitable for display in listings.
    #[serde(rename = "short_description")]
    pub short_description: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GetMultipleRulesetsResponseDataAttributesRulesetsItems {
    pub fn new(
        description: String,
        id: String,
        name: String,
        rules: Vec<crate::datadogV2::model::GetMultipleRulesetsResponseDataAttributesRulesetsItemsRulesItems>,
        short_description: String,
    ) -> GetMultipleRulesetsResponseDataAttributesRulesetsItems {
        GetMultipleRulesetsResponseDataAttributesRulesetsItems {
            description,
            id,
            name,
            rules,
            short_description,
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
                let mut description: Option<String> = None;
                let mut id: Option<String> = None;
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
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rules" => {
                            rules = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "short_description" => {
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
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let rules = rules.ok_or_else(|| M::Error::missing_field("rules"))?;
                let short_description = short_description
                    .ok_or_else(|| M::Error::missing_field("short_description"))?;

                let content = GetMultipleRulesetsResponseDataAttributesRulesetsItems {
                    description,
                    id,
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
