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
pub struct GetCohortUsersRequestDataAttributesDefinitionAudienceFilters {
    #[serde(rename = "accounts")]
    pub accounts: Option<Vec<crate::datadogV2::model::GetCohortUsersRequestDataAttributesDefinitionAudienceFiltersAccountsItems>>,
    #[serde(rename = "formula")]
    pub formula: Option<String>,
    #[serde(rename = "segments")]
    pub segments: Option<Vec<crate::datadogV2::model::GetCohortUsersRequestDataAttributesDefinitionAudienceFiltersSegmentsItems>>,
    #[serde(rename = "users")]
    pub users: Option<Vec<crate::datadogV2::model::GetCohortUsersRequestDataAttributesDefinitionAudienceFiltersUsersItems>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl GetCohortUsersRequestDataAttributesDefinitionAudienceFilters {
    pub fn new() -> GetCohortUsersRequestDataAttributesDefinitionAudienceFilters {
        GetCohortUsersRequestDataAttributesDefinitionAudienceFilters {
            accounts: None,
            formula: None,
            segments: None,
            users: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn accounts(
        mut self,
        value: Vec<crate::datadogV2::model::GetCohortUsersRequestDataAttributesDefinitionAudienceFiltersAccountsItems>,
    ) -> Self {
        self.accounts = Some(value);
        self
    }

    pub fn formula(mut self, value: String) -> Self {
        self.formula = Some(value);
        self
    }

    pub fn segments(
        mut self,
        value: Vec<crate::datadogV2::model::GetCohortUsersRequestDataAttributesDefinitionAudienceFiltersSegmentsItems>,
    ) -> Self {
        self.segments = Some(value);
        self
    }

    pub fn users(
        mut self,
        value: Vec<crate::datadogV2::model::GetCohortUsersRequestDataAttributesDefinitionAudienceFiltersUsersItems>,
    ) -> Self {
        self.users = Some(value);
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

impl Default for GetCohortUsersRequestDataAttributesDefinitionAudienceFilters {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for GetCohortUsersRequestDataAttributesDefinitionAudienceFilters {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GetCohortUsersRequestDataAttributesDefinitionAudienceFiltersVisitor;
        impl<'a> Visitor<'a> for GetCohortUsersRequestDataAttributesDefinitionAudienceFiltersVisitor {
            type Value = GetCohortUsersRequestDataAttributesDefinitionAudienceFilters;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut accounts: Option<Vec<crate::datadogV2::model::GetCohortUsersRequestDataAttributesDefinitionAudienceFiltersAccountsItems>> = None;
                let mut formula: Option<String> = None;
                let mut segments: Option<Vec<crate::datadogV2::model::GetCohortUsersRequestDataAttributesDefinitionAudienceFiltersSegmentsItems>> = None;
                let mut users: Option<Vec<crate::datadogV2::model::GetCohortUsersRequestDataAttributesDefinitionAudienceFiltersUsersItems>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "accounts" => {
                            if v.is_null() {
                                continue;
                            }
                            accounts = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "formula" => {
                            if v.is_null() {
                                continue;
                            }
                            formula = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "segments" => {
                            if v.is_null() {
                                continue;
                            }
                            segments = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "users" => {
                            if v.is_null() {
                                continue;
                            }
                            users = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = GetCohortUsersRequestDataAttributesDefinitionAudienceFilters {
                    accounts,
                    formula,
                    segments,
                    users,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(GetCohortUsersRequestDataAttributesDefinitionAudienceFiltersVisitor)
    }
}
