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
pub struct GetCohortResponseDataAttributesCohortsItems {
    #[serde(rename = "cohort")]
    pub cohort: Option<String>,
    #[serde(rename = "cohort_size")]
    pub cohort_size: Option<i64>,
    #[serde(rename = "start_time")]
    pub start_time: Option<i64>,
    #[serde(rename = "values")]
    pub values: Option<
        Vec<crate::datadogV2::model::GetCohortResponseDataAttributesCohortsItemsValuesItems>,
    >,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GetCohortResponseDataAttributesCohortsItems {
    pub fn new() -> GetCohortResponseDataAttributesCohortsItems {
        GetCohortResponseDataAttributesCohortsItems {
            cohort: None,
            cohort_size: None,
            start_time: None,
            values: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn cohort(mut self, value: String) -> Self {
        self.cohort = Some(value);
        self
    }

    pub fn cohort_size(mut self, value: i64) -> Self {
        self.cohort_size = Some(value);
        self
    }

    pub fn start_time(mut self, value: i64) -> Self {
        self.start_time = Some(value);
        self
    }

    pub fn values(
        mut self,
        value: Vec<crate::datadogV2::model::GetCohortResponseDataAttributesCohortsItemsValuesItems>,
    ) -> Self {
        self.values = Some(value);
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

impl Default for GetCohortResponseDataAttributesCohortsItems {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for GetCohortResponseDataAttributesCohortsItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GetCohortResponseDataAttributesCohortsItemsVisitor;
        impl<'a> Visitor<'a> for GetCohortResponseDataAttributesCohortsItemsVisitor {
            type Value = GetCohortResponseDataAttributesCohortsItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut cohort: Option<String> = None;
                let mut cohort_size: Option<i64> = None;
                let mut start_time: Option<i64> = None;
                let mut values: Option<Vec<crate::datadogV2::model::GetCohortResponseDataAttributesCohortsItemsValuesItems>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "cohort" => {
                            if v.is_null() {
                                continue;
                            }
                            cohort = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cohort_size" => {
                            if v.is_null() {
                                continue;
                            }
                            cohort_size =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "start_time" => {
                            if v.is_null() {
                                continue;
                            }
                            start_time = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "values" => {
                            if v.is_null() {
                                continue;
                            }
                            values = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = GetCohortResponseDataAttributesCohortsItems {
                    cohort,
                    cohort_size,
                    start_time,
                    values,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GetCohortResponseDataAttributesCohortsItemsVisitor)
    }
}
