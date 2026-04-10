// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An individual APM metrics query.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ApmMetricsQuery {
    /// A data source for APM metrics queries.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV2::model::ApmMetricsDataSource,
    /// Optional fields to group the query results by.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<String>>,
    /// The variable name for use in formulas.
    #[serde(rename = "name")]
    pub name: String,
    /// Optional operation mode to aggregate across operation names.
    #[serde(rename = "operation_mode")]
    pub operation_mode: Option<String>,
    /// Name of operation on service. If not provided, the primary operation name is used.
    #[serde(rename = "operation_name")]
    pub operation_name: Option<String>,
    /// Tags to query for a specific downstream entity (peer.service, peer.db_instance, peer.s3, peer.s3.bucket, etc.).
    #[serde(rename = "peer_tags")]
    pub peer_tags: Option<Vec<String>>,
    /// Additional filters for the query using metrics query syntax (e.g., env, primary_tag).
    #[serde(rename = "query_filter")]
    pub query_filter: Option<String>,
    /// The resource hash for exact matching.
    #[serde(rename = "resource_hash")]
    pub resource_hash: Option<String>,
    /// The full name of a specific resource to filter by.
    #[serde(rename = "resource_name")]
    pub resource_name: Option<String>,
    /// The service name to filter by.
    #[serde(rename = "service")]
    pub service: Option<String>,
    /// Describes the relationship between the span, its parents, and its children in a trace.
    #[serde(rename = "span_kind")]
    pub span_kind: Option<crate::datadogV2::model::ApmMetricsSpanKind>,
    /// The APM metric statistic to query.
    #[serde(rename = "stat")]
    pub stat: crate::datadogV2::model::ApmMetricsStat,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ApmMetricsQuery {
    pub fn new(
        data_source: crate::datadogV2::model::ApmMetricsDataSource,
        name: String,
        stat: crate::datadogV2::model::ApmMetricsStat,
    ) -> ApmMetricsQuery {
        ApmMetricsQuery {
            data_source,
            group_by: None,
            name,
            operation_mode: None,
            operation_name: None,
            peer_tags: None,
            query_filter: None,
            resource_hash: None,
            resource_name: None,
            service: None,
            span_kind: None,
            stat,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn group_by(mut self, value: Vec<String>) -> Self {
        self.group_by = Some(value);
        self
    }

    pub fn operation_mode(mut self, value: String) -> Self {
        self.operation_mode = Some(value);
        self
    }

    pub fn operation_name(mut self, value: String) -> Self {
        self.operation_name = Some(value);
        self
    }

    pub fn peer_tags(mut self, value: Vec<String>) -> Self {
        self.peer_tags = Some(value);
        self
    }

    pub fn query_filter(mut self, value: String) -> Self {
        self.query_filter = Some(value);
        self
    }

    pub fn resource_hash(mut self, value: String) -> Self {
        self.resource_hash = Some(value);
        self
    }

    pub fn resource_name(mut self, value: String) -> Self {
        self.resource_name = Some(value);
        self
    }

    pub fn service(mut self, value: String) -> Self {
        self.service = Some(value);
        self
    }

    pub fn span_kind(mut self, value: crate::datadogV2::model::ApmMetricsSpanKind) -> Self {
        self.span_kind = Some(value);
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

impl<'de> Deserialize<'de> for ApmMetricsQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ApmMetricsQueryVisitor;
        impl<'a> Visitor<'a> for ApmMetricsQueryVisitor {
            type Value = ApmMetricsQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data_source: Option<crate::datadogV2::model::ApmMetricsDataSource> = None;
                let mut group_by: Option<Vec<String>> = None;
                let mut name: Option<String> = None;
                let mut operation_mode: Option<String> = None;
                let mut operation_name: Option<String> = None;
                let mut peer_tags: Option<Vec<String>> = None;
                let mut query_filter: Option<String> = None;
                let mut resource_hash: Option<String> = None;
                let mut resource_name: Option<String> = None;
                let mut service: Option<String> = None;
                let mut span_kind: Option<crate::datadogV2::model::ApmMetricsSpanKind> = None;
                let mut stat: Option<crate::datadogV2::model::ApmMetricsStat> = None;
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
                            if let Some(ref _data_source) = data_source {
                                match _data_source {
                                    crate::datadogV2::model::ApmMetricsDataSource::UnparsedObject(_data_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "group_by" => {
                            if v.is_null() {
                                continue;
                            }
                            group_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "operation_mode" => {
                            if v.is_null() {
                                continue;
                            }
                            operation_mode =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "operation_name" => {
                            if v.is_null() {
                                continue;
                            }
                            operation_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "peer_tags" => {
                            if v.is_null() {
                                continue;
                            }
                            peer_tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query_filter" => {
                            if v.is_null() {
                                continue;
                            }
                            query_filter =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resource_hash" => {
                            if v.is_null() {
                                continue;
                            }
                            resource_hash =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resource_name" => {
                            if v.is_null() {
                                continue;
                            }
                            resource_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "service" => {
                            if v.is_null() {
                                continue;
                            }
                            service = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "span_kind" => {
                            if v.is_null() {
                                continue;
                            }
                            span_kind = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _span_kind) = span_kind {
                                match _span_kind {
                                    crate::datadogV2::model::ApmMetricsSpanKind::UnparsedObject(
                                        _span_kind,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "stat" => {
                            stat = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _stat) = stat {
                                match _stat {
                                    crate::datadogV2::model::ApmMetricsStat::UnparsedObject(
                                        _stat,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
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
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let stat = stat.ok_or_else(|| M::Error::missing_field("stat"))?;

                let content = ApmMetricsQuery {
                    data_source,
                    group_by,
                    name,
                    operation_mode,
                    operation_name,
                    peer_tags,
                    query_filter,
                    resource_hash,
                    resource_name,
                    service,
                    span_kind,
                    stat,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ApmMetricsQueryVisitor)
    }
}
