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
pub struct GetCohortUsersRequestDataAttributes {
    #[serde(rename = "data_source")]
    pub data_source: Option<String>,
    #[serde(rename = "definition")]
    pub definition: Option<crate::datadogV2::model::GetCohortUsersRequestDataAttributesDefinition>,
    #[serde(rename = "execution")]
    pub execution: Option<i64>,
    #[serde(rename = "time")]
    pub time: Option<crate::datadogV2::model::GetCohortUsersRequestDataAttributesTime>,
    #[serde(rename = "user_selection")]
    pub user_selection: Option<String>,
    #[serde(rename = "window_size")]
    pub window_size: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GetCohortUsersRequestDataAttributes {
    pub fn new() -> GetCohortUsersRequestDataAttributes {
        GetCohortUsersRequestDataAttributes {
            data_source: None,
            definition: None,
            execution: None,
            time: None,
            user_selection: None,
            window_size: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn data_source(mut self, value: String) -> Self {
        self.data_source = Some(value);
        self
    }

    pub fn definition(
        mut self,
        value: crate::datadogV2::model::GetCohortUsersRequestDataAttributesDefinition,
    ) -> Self {
        self.definition = Some(value);
        self
    }

    pub fn execution(mut self, value: i64) -> Self {
        self.execution = Some(value);
        self
    }

    pub fn time(
        mut self,
        value: crate::datadogV2::model::GetCohortUsersRequestDataAttributesTime,
    ) -> Self {
        self.time = Some(value);
        self
    }

    pub fn user_selection(mut self, value: String) -> Self {
        self.user_selection = Some(value);
        self
    }

    pub fn window_size(mut self, value: String) -> Self {
        self.window_size = Some(value);
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

impl Default for GetCohortUsersRequestDataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for GetCohortUsersRequestDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GetCohortUsersRequestDataAttributesVisitor;
        impl<'a> Visitor<'a> for GetCohortUsersRequestDataAttributesVisitor {
            type Value = GetCohortUsersRequestDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data_source: Option<String> = None;
                let mut definition: Option<
                    crate::datadogV2::model::GetCohortUsersRequestDataAttributesDefinition,
                > = None;
                let mut execution: Option<i64> = None;
                let mut time: Option<
                    crate::datadogV2::model::GetCohortUsersRequestDataAttributesTime,
                > = None;
                let mut user_selection: Option<String> = None;
                let mut window_size: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data_source" => {
                            if v.is_null() {
                                continue;
                            }
                            data_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "definition" => {
                            if v.is_null() {
                                continue;
                            }
                            definition = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "execution" => {
                            if v.is_null() {
                                continue;
                            }
                            execution = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "time" => {
                            if v.is_null() {
                                continue;
                            }
                            time = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "user_selection" => {
                            if v.is_null() {
                                continue;
                            }
                            user_selection =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "window_size" => {
                            if v.is_null() {
                                continue;
                            }
                            window_size =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = GetCohortUsersRequestDataAttributes {
                    data_source,
                    definition,
                    execution,
                    time,
                    user_selection,
                    window_size,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GetCohortUsersRequestDataAttributesVisitor)
    }
}
