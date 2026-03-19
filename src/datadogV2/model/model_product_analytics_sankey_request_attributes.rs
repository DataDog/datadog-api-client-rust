// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for a Sankey request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsSankeyRequestAttributes {
    /// The data source for the Sankey query.
    #[serde(rename = "data_source")]
    pub data_source: String,
    /// Sankey visualization definition. Set either `source` or `target`, not both.
    /// Use `source` for forward flow (where do users go after this page?) or
    /// `target` for backward flow (where did users come from?).
    #[serde(rename = "definition")]
    pub definition: crate::datadogV2::model::ProductAnalyticsSankeyDefinition,
    /// Override the query execution strategy.
    #[serde(rename = "enforced_execution_type")]
    pub enforced_execution_type: Option<crate::datadogV2::model::ProductAnalyticsExecutionType>,
    #[serde(rename = "request_id")]
    pub request_id: Option<String>,
    /// Sampling configuration.
    #[serde(rename = "sampling")]
    pub sampling: Option<crate::datadogV2::model::ProductAnalyticsSampling>,
    /// Search parameters for a Sankey query.
    #[serde(rename = "search")]
    pub search: crate::datadogV2::model::ProductAnalyticsSankeySearch,
    /// Time range for the Sankey query.
    #[serde(rename = "time")]
    pub time: crate::datadogV2::model::ProductAnalyticsSankeyTime,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsSankeyRequestAttributes {
    pub fn new(
        data_source: String,
        definition: crate::datadogV2::model::ProductAnalyticsSankeyDefinition,
        search: crate::datadogV2::model::ProductAnalyticsSankeySearch,
        time: crate::datadogV2::model::ProductAnalyticsSankeyTime,
    ) -> ProductAnalyticsSankeyRequestAttributes {
        ProductAnalyticsSankeyRequestAttributes {
            data_source,
            definition,
            enforced_execution_type: None,
            request_id: None,
            sampling: None,
            search,
            time,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn enforced_execution_type(
        mut self,
        value: crate::datadogV2::model::ProductAnalyticsExecutionType,
    ) -> Self {
        self.enforced_execution_type = Some(value);
        self
    }

    pub fn request_id(mut self, value: String) -> Self {
        self.request_id = Some(value);
        self
    }

    pub fn sampling(mut self, value: crate::datadogV2::model::ProductAnalyticsSampling) -> Self {
        self.sampling = Some(value);
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

impl<'de> Deserialize<'de> for ProductAnalyticsSankeyRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsSankeyRequestAttributesVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsSankeyRequestAttributesVisitor {
            type Value = ProductAnalyticsSankeyRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data_source: Option<String> = None;
                let mut definition: Option<
                    crate::datadogV2::model::ProductAnalyticsSankeyDefinition,
                > = None;
                let mut enforced_execution_type: Option<
                    crate::datadogV2::model::ProductAnalyticsExecutionType,
                > = None;
                let mut request_id: Option<String> = None;
                let mut sampling: Option<crate::datadogV2::model::ProductAnalyticsSampling> = None;
                let mut search: Option<crate::datadogV2::model::ProductAnalyticsSankeySearch> =
                    None;
                let mut time: Option<crate::datadogV2::model::ProductAnalyticsSankeyTime> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data_source" => {
                            data_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "definition" => {
                            definition = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enforced_execution_type" => {
                            if v.is_null() {
                                continue;
                            }
                            enforced_execution_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _enforced_execution_type) = enforced_execution_type {
                                match _enforced_execution_type {
                                    crate::datadogV2::model::ProductAnalyticsExecutionType::UnparsedObject(_enforced_execution_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "request_id" => {
                            if v.is_null() {
                                continue;
                            }
                            request_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sampling" => {
                            if v.is_null() {
                                continue;
                            }
                            sampling = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "search" => {
                            search = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "time" => {
                            time = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let data_source =
                    data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;
                let definition = definition.ok_or_else(|| M::Error::missing_field("definition"))?;
                let search = search.ok_or_else(|| M::Error::missing_field("search"))?;
                let time = time.ok_or_else(|| M::Error::missing_field("time"))?;

                let content = ProductAnalyticsSankeyRequestAttributes {
                    data_source,
                    definition,
                    enforced_execution_type,
                    request_id,
                    sampling,
                    search,
                    time,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsSankeyRequestAttributesVisitor)
    }
}
