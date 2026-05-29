// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Infrastructure-backed request for the host map widget. Supports entity-based
/// visualization with metric query enrichments, tag-based filtering, flexible grouping,
/// and hierarchical views.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct HostMapWidgetInfrastructureRequest {
    /// Infrastructure-backed host map child request (leaf node, no further nesting supported).
    #[serde(rename = "child")]
    pub child: Option<crate::datadogV1::model::HostMapWidgetInfrastructureRequestLeaf>,
    /// List of conditional formatting rules applied to fill values.
    #[serde(rename = "conditional_formats")]
    pub conditional_formats: Option<Vec<crate::datadogV1::model::WidgetConditionalFormat>>,
    /// Metric or event queries joined to the entity set. Each formula specifies a visual dimension.
    #[serde(rename = "enrichments")]
    pub enrichments: Vec<crate::datadogV1::model::HostMapWidgetScalarRequest>,
    /// Filter string for the entity set in tag format (for example, `env:prod`).
    #[serde(rename = "filter")]
    pub filter: Option<String>,
    /// Defines how entities are grouped into tiles. The ordering of entries implies
    /// the grouping hierarchy.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<crate::datadogV1::model::HostMapWidgetGroupBy>>,
    /// Whether to hide entities that have no group assignment.
    #[serde(rename = "no_group_hosts")]
    pub no_group_hosts: Option<bool>,
    /// Whether to hide entities that have no enrichment data.
    #[serde(rename = "no_metric_hosts")]
    pub no_metric_hosts: Option<bool>,
    /// Which type of infrastructure entity to visualize in the host map.
    #[serde(rename = "node_type")]
    pub node_type: crate::datadogV1::model::HostMapWidgetNodeType,
    /// Identifies this as an infrastructure-backed host map request.
    #[serde(rename = "request_type")]
    pub request_type: crate::datadogV1::model::HostMapWidgetInfrastructureRequestRequestType,
    /// Style configuration for the infrastructure host map.
    #[serde(rename = "style")]
    pub style: Option<crate::datadogV1::model::HostMapWidgetInfrastructureStyle>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl HostMapWidgetInfrastructureRequest {
    pub fn new(
        enrichments: Vec<crate::datadogV1::model::HostMapWidgetScalarRequest>,
        node_type: crate::datadogV1::model::HostMapWidgetNodeType,
        request_type: crate::datadogV1::model::HostMapWidgetInfrastructureRequestRequestType,
    ) -> HostMapWidgetInfrastructureRequest {
        HostMapWidgetInfrastructureRequest {
            child: None,
            conditional_formats: None,
            enrichments,
            filter: None,
            group_by: None,
            no_group_hosts: None,
            no_metric_hosts: None,
            node_type,
            request_type,
            style: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn child(
        mut self,
        value: crate::datadogV1::model::HostMapWidgetInfrastructureRequestLeaf,
    ) -> Self {
        self.child = Some(value);
        self
    }

    pub fn conditional_formats(
        mut self,
        value: Vec<crate::datadogV1::model::WidgetConditionalFormat>,
    ) -> Self {
        self.conditional_formats = Some(value);
        self
    }

    pub fn filter(mut self, value: String) -> Self {
        self.filter = Some(value);
        self
    }

    pub fn group_by(mut self, value: Vec<crate::datadogV1::model::HostMapWidgetGroupBy>) -> Self {
        self.group_by = Some(value);
        self
    }

    pub fn no_group_hosts(mut self, value: bool) -> Self {
        self.no_group_hosts = Some(value);
        self
    }

    pub fn no_metric_hosts(mut self, value: bool) -> Self {
        self.no_metric_hosts = Some(value);
        self
    }

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

impl<'de> Deserialize<'de> for HostMapWidgetInfrastructureRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HostMapWidgetInfrastructureRequestVisitor;
        impl<'a> Visitor<'a> for HostMapWidgetInfrastructureRequestVisitor {
            type Value = HostMapWidgetInfrastructureRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut child: Option<
                    crate::datadogV1::model::HostMapWidgetInfrastructureRequestLeaf,
                > = None;
                let mut conditional_formats: Option<
                    Vec<crate::datadogV1::model::WidgetConditionalFormat>,
                > = None;
                let mut enrichments: Option<
                    Vec<crate::datadogV1::model::HostMapWidgetScalarRequest>,
                > = None;
                let mut filter: Option<String> = None;
                let mut group_by: Option<Vec<crate::datadogV1::model::HostMapWidgetGroupBy>> = None;
                let mut no_group_hosts: Option<bool> = None;
                let mut no_metric_hosts: Option<bool> = None;
                let mut node_type: Option<crate::datadogV1::model::HostMapWidgetNodeType> = None;
                let mut request_type: Option<
                    crate::datadogV1::model::HostMapWidgetInfrastructureRequestRequestType,
                > = None;
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
                            enrichments =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "request_type" => {
                            request_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _request_type) = request_type {
                                match _request_type {
                                    crate::datadogV1::model::HostMapWidgetInfrastructureRequestRequestType::UnparsedObject(_request_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
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
                let enrichments =
                    enrichments.ok_or_else(|| M::Error::missing_field("enrichments"))?;
                let node_type = node_type.ok_or_else(|| M::Error::missing_field("node_type"))?;
                let request_type =
                    request_type.ok_or_else(|| M::Error::missing_field("request_type"))?;

                let content = HostMapWidgetInfrastructureRequest {
                    child,
                    conditional_formats,
                    enrichments,
                    filter,
                    group_by,
                    no_group_hosts,
                    no_metric_hosts,
                    node_type,
                    request_type,
                    style,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(HostMapWidgetInfrastructureRequestVisitor)
    }
}
