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
pub struct GetCohortRequestDataAttributesDefinition {
    #[serde(rename = "audience_filters")]
    pub audience_filters:
        Option<crate::datadogV2::model::GetCohortRequestDataAttributesDefinitionAudienceFilters>,
    #[serde(rename = "inclusion_search")]
    pub inclusion_search: Option<String>,
    #[serde(rename = "return_search")]
    pub return_search: Option<String>,
    #[serde(rename = "segment_id")]
    pub segment_id: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GetCohortRequestDataAttributesDefinition {
    pub fn new() -> GetCohortRequestDataAttributesDefinition {
        GetCohortRequestDataAttributesDefinition {
            audience_filters: None,
            inclusion_search: None,
            return_search: None,
            segment_id: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn audience_filters(
        mut self,
        value: crate::datadogV2::model::GetCohortRequestDataAttributesDefinitionAudienceFilters,
    ) -> Self {
        self.audience_filters = Some(value);
        self
    }

    pub fn inclusion_search(mut self, value: String) -> Self {
        self.inclusion_search = Some(value);
        self
    }

    pub fn return_search(mut self, value: String) -> Self {
        self.return_search = Some(value);
        self
    }

    pub fn segment_id(mut self, value: String) -> Self {
        self.segment_id = Some(value);
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

impl Default for GetCohortRequestDataAttributesDefinition {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for GetCohortRequestDataAttributesDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GetCohortRequestDataAttributesDefinitionVisitor;
        impl<'a> Visitor<'a> for GetCohortRequestDataAttributesDefinitionVisitor {
            type Value = GetCohortRequestDataAttributesDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut audience_filters: Option<crate::datadogV2::model::GetCohortRequestDataAttributesDefinitionAudienceFilters> = None;
                let mut inclusion_search: Option<String> = None;
                let mut return_search: Option<String> = None;
                let mut segment_id: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "audience_filters" => {
                            if v.is_null() {
                                continue;
                            }
                            audience_filters =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "inclusion_search" => {
                            if v.is_null() {
                                continue;
                            }
                            inclusion_search =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "return_search" => {
                            if v.is_null() {
                                continue;
                            }
                            return_search =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "segment_id" => {
                            if v.is_null() {
                                continue;
                            }
                            segment_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = GetCohortRequestDataAttributesDefinition {
                    audience_filters,
                    inclusion_search,
                    return_search,
                    segment_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GetCohortRequestDataAttributesDefinitionVisitor)
    }
}
