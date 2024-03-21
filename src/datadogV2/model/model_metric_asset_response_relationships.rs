// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Relationships to assets related to the metric.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MetricAssetResponseRelationships {
    /// An object containing the list of dashboards that can be referenced in the `included` data.
    #[serde(rename = "dashboards")]
    pub dashboards: Option<crate::datadogV2::model::MetricAssetDashboardRelationships>,
    /// A object containing the list of monitors that can be referenced in the `included` data.
    #[serde(rename = "monitors")]
    pub monitors: Option<crate::datadogV2::model::MetricAssetMonitorRelationships>,
    /// An object containing the list of notebooks that can be referenced in the `included` data.
    #[serde(rename = "notebooks")]
    pub notebooks: Option<crate::datadogV2::model::MetricAssetNotebookRelationships>,
    /// An object containing a list of SLOs that can be referenced in the `included` data.
    #[serde(rename = "slos")]
    pub slos: Option<crate::datadogV2::model::MetricAssetSLORelationships>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MetricAssetResponseRelationships {
    pub fn new() -> MetricAssetResponseRelationships {
        MetricAssetResponseRelationships {
            dashboards: None,
            monitors: None,
            notebooks: None,
            slos: None,
            _unparsed: false,
        }
    }

    pub fn dashboards(
        mut self,
        value: crate::datadogV2::model::MetricAssetDashboardRelationships,
    ) -> Self {
        self.dashboards = Some(value);
        self
    }

    pub fn monitors(
        mut self,
        value: crate::datadogV2::model::MetricAssetMonitorRelationships,
    ) -> Self {
        self.monitors = Some(value);
        self
    }

    pub fn notebooks(
        mut self,
        value: crate::datadogV2::model::MetricAssetNotebookRelationships,
    ) -> Self {
        self.notebooks = Some(value);
        self
    }

    pub fn slos(mut self, value: crate::datadogV2::model::MetricAssetSLORelationships) -> Self {
        self.slos = Some(value);
        self
    }
}

impl Default for MetricAssetResponseRelationships {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for MetricAssetResponseRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MetricAssetResponseRelationshipsVisitor;
        impl<'a> Visitor<'a> for MetricAssetResponseRelationshipsVisitor {
            type Value = MetricAssetResponseRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut dashboards: Option<
                    crate::datadogV2::model::MetricAssetDashboardRelationships,
                > = None;
                let mut monitors: Option<crate::datadogV2::model::MetricAssetMonitorRelationships> =
                    None;
                let mut notebooks: Option<
                    crate::datadogV2::model::MetricAssetNotebookRelationships,
                > = None;
                let mut slos: Option<crate::datadogV2::model::MetricAssetSLORelationships> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "dashboards" => {
                            if v.is_null() {
                                continue;
                            }
                            dashboards = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "monitors" => {
                            if v.is_null() {
                                continue;
                            }
                            monitors = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "notebooks" => {
                            if v.is_null() {
                                continue;
                            }
                            notebooks = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "slos" => {
                            if v.is_null() {
                                continue;
                            }
                            slos = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = MetricAssetResponseRelationships {
                    dashboards,
                    monitors,
                    notebooks,
                    slos,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MetricAssetResponseRelationshipsVisitor)
    }
}
