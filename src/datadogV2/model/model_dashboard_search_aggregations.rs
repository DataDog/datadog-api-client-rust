// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Aggregations of dashboard search results.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DashboardSearchAggregations {
    /// Aggregation by author.
    #[serde(rename = "author")]
    pub author: Option<Vec<crate::datadogV2::model::DashboardSearchAggregationBucketMultiKey>>,
    /// Aggregation by share status.
    #[serde(rename = "is_shared")]
    pub is_shared: Option<Vec<crate::datadogV2::model::DashboardSearchAggregationBucketKey>>,
    /// Aggregation by share type.
    #[serde(rename = "share_type")]
    pub share_type: Option<Vec<crate::datadogV2::model::DashboardSearchAggregationBucketKey>>,
    /// Aggregation by who shared the dashboard.
    #[serde(rename = "shared_by.handle")]
    pub shared_by_handle: Option<Vec<crate::datadogV2::model::DashboardSearchAggregationBucketKey>>,
    /// Aggregation by status.
    #[serde(rename = "status")]
    pub status: Option<Vec<crate::datadogV2::model::DashboardSearchAggregationBucketKey>>,
    /// Aggregation by tags.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<crate::datadogV2::model::DashboardSearchAggregationBucketKey>>,
    /// Aggregation by template variable names.
    #[serde(rename = "template_variables.name")]
    pub template_variables_name:
        Option<Vec<crate::datadogV2::model::DashboardSearchAggregationBucketKey>>,
    /// Aggregation by dashboard type.
    #[serde(rename = "type")]
    pub type_: Option<Vec<crate::datadogV2::model::DashboardSearchAggregationBucketKey>>,
    /// Aggregation by widget metrics.
    #[serde(rename = "widgets.metrics")]
    pub widgets_metrics: Option<Vec<crate::datadogV2::model::DashboardSearchAggregationBucketKey>>,
    /// Aggregation by widget type.
    #[serde(rename = "widgets.type")]
    pub widgets_type: Option<Vec<crate::datadogV2::model::DashboardSearchAggregationBucketKey>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DashboardSearchAggregations {
    pub fn new() -> DashboardSearchAggregations {
        DashboardSearchAggregations {
            author: None,
            is_shared: None,
            share_type: None,
            shared_by_handle: None,
            status: None,
            tags: None,
            template_variables_name: None,
            type_: None,
            widgets_metrics: None,
            widgets_type: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn author(
        mut self,
        value: Vec<crate::datadogV2::model::DashboardSearchAggregationBucketMultiKey>,
    ) -> Self {
        self.author = Some(value);
        self
    }

    pub fn is_shared(
        mut self,
        value: Vec<crate::datadogV2::model::DashboardSearchAggregationBucketKey>,
    ) -> Self {
        self.is_shared = Some(value);
        self
    }

    pub fn share_type(
        mut self,
        value: Vec<crate::datadogV2::model::DashboardSearchAggregationBucketKey>,
    ) -> Self {
        self.share_type = Some(value);
        self
    }

    pub fn shared_by_handle(
        mut self,
        value: Vec<crate::datadogV2::model::DashboardSearchAggregationBucketKey>,
    ) -> Self {
        self.shared_by_handle = Some(value);
        self
    }

    pub fn status(
        mut self,
        value: Vec<crate::datadogV2::model::DashboardSearchAggregationBucketKey>,
    ) -> Self {
        self.status = Some(value);
        self
    }

    pub fn tags(
        mut self,
        value: Vec<crate::datadogV2::model::DashboardSearchAggregationBucketKey>,
    ) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn template_variables_name(
        mut self,
        value: Vec<crate::datadogV2::model::DashboardSearchAggregationBucketKey>,
    ) -> Self {
        self.template_variables_name = Some(value);
        self
    }

    pub fn type_(
        mut self,
        value: Vec<crate::datadogV2::model::DashboardSearchAggregationBucketKey>,
    ) -> Self {
        self.type_ = Some(value);
        self
    }

    pub fn widgets_metrics(
        mut self,
        value: Vec<crate::datadogV2::model::DashboardSearchAggregationBucketKey>,
    ) -> Self {
        self.widgets_metrics = Some(value);
        self
    }

    pub fn widgets_type(
        mut self,
        value: Vec<crate::datadogV2::model::DashboardSearchAggregationBucketKey>,
    ) -> Self {
        self.widgets_type = Some(value);
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

impl Default for DashboardSearchAggregations {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for DashboardSearchAggregations {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DashboardSearchAggregationsVisitor;
        impl<'a> Visitor<'a> for DashboardSearchAggregationsVisitor {
            type Value = DashboardSearchAggregations;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut author: Option<
                    Vec<crate::datadogV2::model::DashboardSearchAggregationBucketMultiKey>,
                > = None;
                let mut is_shared: Option<
                    Vec<crate::datadogV2::model::DashboardSearchAggregationBucketKey>,
                > = None;
                let mut share_type: Option<
                    Vec<crate::datadogV2::model::DashboardSearchAggregationBucketKey>,
                > = None;
                let mut shared_by_handle: Option<
                    Vec<crate::datadogV2::model::DashboardSearchAggregationBucketKey>,
                > = None;
                let mut status: Option<
                    Vec<crate::datadogV2::model::DashboardSearchAggregationBucketKey>,
                > = None;
                let mut tags: Option<
                    Vec<crate::datadogV2::model::DashboardSearchAggregationBucketKey>,
                > = None;
                let mut template_variables_name: Option<
                    Vec<crate::datadogV2::model::DashboardSearchAggregationBucketKey>,
                > = None;
                let mut type_: Option<
                    Vec<crate::datadogV2::model::DashboardSearchAggregationBucketKey>,
                > = None;
                let mut widgets_metrics: Option<
                    Vec<crate::datadogV2::model::DashboardSearchAggregationBucketKey>,
                > = None;
                let mut widgets_type: Option<
                    Vec<crate::datadogV2::model::DashboardSearchAggregationBucketKey>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "author" => {
                            if v.is_null() {
                                continue;
                            }
                            author = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_shared" => {
                            if v.is_null() {
                                continue;
                            }
                            is_shared = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "share_type" => {
                            if v.is_null() {
                                continue;
                            }
                            share_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "shared_by.handle" => {
                            if v.is_null() {
                                continue;
                            }
                            shared_by_handle =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "template_variables.name" => {
                            if v.is_null() {
                                continue;
                            }
                            template_variables_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "widgets.metrics" => {
                            if v.is_null() {
                                continue;
                            }
                            widgets_metrics =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "widgets.type" => {
                            if v.is_null() {
                                continue;
                            }
                            widgets_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = DashboardSearchAggregations {
                    author,
                    is_shared,
                    share_type,
                    shared_by_handle,
                    status,
                    tags,
                    template_variables_name,
                    type_,
                    widgets_metrics,
                    widgets_type,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DashboardSearchAggregationsVisitor)
    }
}
