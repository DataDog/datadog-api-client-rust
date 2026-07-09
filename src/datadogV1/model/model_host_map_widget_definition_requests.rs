// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Query definition for the host map widget. Supports three mutually exclusive formats distinguished by `request_type`: the deprecated legacy metric-based format (`fill`/`size`, no `request_type`), the infrastructure-backed format (`request_type: infrastructure_hostmap`), and the DDSQL published-dataset format (`request_type: data_projection`).
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct HostMapWidgetDefinitionRequests {
    /// Infrastructure-backed request for the host map widget. Supports entity-based
    /// visualization with metric query enrichments, tag-based filtering, flexible grouping,
    /// and hierarchical views.
    #[serde(rename = "child")]
    pub child: Option<crate::datadogV1::model::HostMapWidgetInfrastructureRequest>,
    /// List of conditional formatting rules applied to fill values.
    #[serde(rename = "conditional_formats")]
    pub conditional_formats: Option<Vec<crate::datadogV1::model::WidgetConditionalFormat>>,
    /// Metric or event queries joined to the entity set. Each formula specifies a visual dimension. Only used by the infrastructure-backed format.
    #[serde(rename = "enrichments")]
    pub enrichments: Option<Vec<crate::datadogV1::model::HostMapWidgetScalarRequest>>,
    /// Deprecated - Legacy metric-based host map request. Use the infrastructure-backed (`request_type: infrastructure_hostmap`) or DDSQL (`request_type: data_projection`) format instead.
    #[deprecated]
    #[serde(rename = "fill")]
    pub fill: Option<crate::datadogV1::model::HostMapRequest>,
    /// Filter string for the entity set in tag format (for example, `env:prod`). Only used by the infrastructure-backed format.
    #[serde(rename = "filter")]
    pub filter: Option<String>,
    /// Defines how entities are grouped into tiles. The ordering of entries implies
    /// the grouping hierarchy. Only used by the infrastructure-backed format.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<crate::datadogV1::model::HostMapWidgetGroupBy>>,
    /// Maximum number of rows to return from the dataset query. Only used by the DDSQL format.
    #[serde(rename = "limit")]
    pub limit: Option<i64>,
    /// Whether to hide entities that have no group assignment.
    #[serde(rename = "no_group_hosts")]
    pub no_group_hosts: Option<bool>,
    /// Whether to hide entities that have no enrichment data.
    #[serde(rename = "no_metric_hosts")]
    pub no_metric_hosts: Option<bool>,
    /// Which type of infrastructure entity to visualize in the host map.
    #[serde(rename = "node_type")]
    pub node_type: Option<crate::datadogV1::model::HostMapWidgetNodeType>,
    /// Projection for the DDSQL host map request. Maps dataset columns to map dimensions: `node` identifies the entity, repeated `group` entries define the grouping hierarchy (outermost first), and `fill`/`size` drive the tile color and size.
    #[serde(rename = "projection")]
    pub projection: Option<crate::datadogV1::model::HostMapWidgetProjection>,
    /// Query that lists the rows of a published dataset (a DDSQL query) without aggregation.
    #[serde(rename = "query")]
    pub query: Option<crate::datadogV1::model::DatasetListQuery>,
    /// Identifies which host map request format the sibling fields on `HostMapWidgetDefinitionRequests` describe: an infrastructure-backed request or a DDSQL published-dataset request.
    #[serde(rename = "request_type")]
    pub request_type: Option<crate::datadogV1::model::HostMapWidgetDefinitionRequestType>,
    /// Deprecated - Legacy metric-based host map request. Use the infrastructure-backed (`request_type: infrastructure_hostmap`) or DDSQL (`request_type: data_projection`) format instead.
    #[deprecated]
    #[serde(rename = "size")]
    pub size: Option<crate::datadogV1::model::HostMapRequest>,
    /// Style configuration for the infrastructure host map.
    #[serde(rename = "style")]
    pub style: Option<crate::datadogV1::model::HostMapWidgetInfrastructureStyle>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl HostMapWidgetDefinitionRequests {
    pub fn new() -> HostMapWidgetDefinitionRequests {
        #[allow(deprecated)]
        HostMapWidgetDefinitionRequests {
            child: None,
            conditional_formats: None,
            enrichments: None,
            fill: None,
            filter: None,
            group_by: None,
            limit: None,
            no_group_hosts: None,
            no_metric_hosts: None,
            node_type: None,
            projection: None,
            query: None,
            request_type: None,
            size: None,
            style: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    #[allow(deprecated)]
    pub fn child(
        mut self,
        value: crate::datadogV1::model::HostMapWidgetInfrastructureRequest,
    ) -> Self {
        self.child = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn conditional_formats(
        mut self,
        value: Vec<crate::datadogV1::model::WidgetConditionalFormat>,
    ) -> Self {
        self.conditional_formats = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn enrichments(
        mut self,
        value: Vec<crate::datadogV1::model::HostMapWidgetScalarRequest>,
    ) -> Self {
        self.enrichments = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn fill(mut self, value: crate::datadogV1::model::HostMapRequest) -> Self {
        self.fill = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn filter(mut self, value: String) -> Self {
        self.filter = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn group_by(mut self, value: Vec<crate::datadogV1::model::HostMapWidgetGroupBy>) -> Self {
        self.group_by = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn no_group_hosts(mut self, value: bool) -> Self {
        self.no_group_hosts = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn no_metric_hosts(mut self, value: bool) -> Self {
        self.no_metric_hosts = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn node_type(mut self, value: crate::datadogV1::model::HostMapWidgetNodeType) -> Self {
        self.node_type = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn projection(mut self, value: crate::datadogV1::model::HostMapWidgetProjection) -> Self {
        self.projection = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn query(mut self, value: crate::datadogV1::model::DatasetListQuery) -> Self {
        self.query = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn request_type(
        mut self,
        value: crate::datadogV1::model::HostMapWidgetDefinitionRequestType,
    ) -> Self {
        self.request_type = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn size(mut self, value: crate::datadogV1::model::HostMapRequest) -> Self {
        self.size = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn style(
        mut self,
        value: crate::datadogV1::model::HostMapWidgetInfrastructureStyle,
    ) -> Self {
        self.style = Some(value);
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

impl Default for HostMapWidgetDefinitionRequests {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for HostMapWidgetDefinitionRequests {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HostMapWidgetDefinitionRequestsVisitor;
        impl<'a> Visitor<'a> for HostMapWidgetDefinitionRequestsVisitor {
            type Value = HostMapWidgetDefinitionRequests;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut child: Option<crate::datadogV1::model::HostMapWidgetInfrastructureRequest> =
                    None;
                let mut conditional_formats: Option<
                    Vec<crate::datadogV1::model::WidgetConditionalFormat>,
                > = None;
                let mut enrichments: Option<
                    Vec<crate::datadogV1::model::HostMapWidgetScalarRequest>,
                > = None;
                let mut fill: Option<crate::datadogV1::model::HostMapRequest> = None;
                let mut filter: Option<String> = None;
                let mut group_by: Option<Vec<crate::datadogV1::model::HostMapWidgetGroupBy>> = None;
                let mut limit: Option<i64> = None;
                let mut no_group_hosts: Option<bool> = None;
                let mut no_metric_hosts: Option<bool> = None;
                let mut node_type: Option<crate::datadogV1::model::HostMapWidgetNodeType> = None;
                let mut projection: Option<crate::datadogV1::model::HostMapWidgetProjection> = None;
                let mut query: Option<crate::datadogV1::model::DatasetListQuery> = None;
                let mut request_type: Option<
                    crate::datadogV1::model::HostMapWidgetDefinitionRequestType,
                > = None;
                let mut size: Option<crate::datadogV1::model::HostMapRequest> = None;
                let mut style: Option<crate::datadogV1::model::HostMapWidgetInfrastructureStyle> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "child" => {
                            if v.is_null() {
                                continue;
                            }
                            child = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "conditional_formats" => {
                            if v.is_null() {
                                continue;
                            }
                            conditional_formats =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enrichments" => {
                            if v.is_null() {
                                continue;
                            }
                            enrichments =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "fill" => {
                            if v.is_null() {
                                continue;
                            }
                            fill = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "filter" => {
                            if v.is_null() {
                                continue;
                            }
                            filter = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "group_by" => {
                            if v.is_null() {
                                continue;
                            }
                            group_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "limit" => {
                            if v.is_null() {
                                continue;
                            }
                            limit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "no_group_hosts" => {
                            if v.is_null() {
                                continue;
                            }
                            no_group_hosts =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "no_metric_hosts" => {
                            if v.is_null() {
                                continue;
                            }
                            no_metric_hosts =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "node_type" => {
                            if v.is_null() {
                                continue;
                            }
                            node_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _node_type) = node_type {
                                match _node_type {
                                    crate::datadogV1::model::HostMapWidgetNodeType::UnparsedObject(_node_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "projection" => {
                            if v.is_null() {
                                continue;
                            }
                            projection = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            if v.is_null() {
                                continue;
                            }
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "request_type" => {
                            if v.is_null() {
                                continue;
                            }
                            request_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _request_type) = request_type {
                                match _request_type {
                                    crate::datadogV1::model::HostMapWidgetDefinitionRequestType::UnparsedObject(_request_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "size" => {
                            if v.is_null() {
                                continue;
                            }
                            size = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "style" => {
                            if v.is_null() {
                                continue;
                            }
                            style = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                #[allow(deprecated)]
                let content = HostMapWidgetDefinitionRequests {
                    child,
                    conditional_formats,
                    enrichments,
                    fill,
                    filter,
                    group_by,
                    limit,
                    no_group_hosts,
                    no_metric_hosts,
                    node_type,
                    projection,
                    query,
                    request_type,
                    size,
                    style,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(HostMapWidgetDefinitionRequestsVisitor)
    }
}
