// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The APM stats query for table and distributions widgets.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApmStatsQueryDefinition {
    /// Column properties used by the front end for display.
    #[serde(rename = "columns")]
    pub columns: Option<Vec<crate::datadogV1::model::ApmStatsQueryColumnType>>,
    /// Environment name.
    #[serde(rename = "env")]
    pub env: String,
    /// Operation name associated with service.
    #[serde(rename = "name")]
    pub name: String,
    /// The organization's host group name and value.
    #[serde(rename = "primary_tag")]
    pub primary_tag: String,
    /// Resource name.
    #[serde(rename = "resource")]
    pub resource: Option<String>,
    /// The level of detail for the request.
    #[serde(rename = "row_type")]
    pub row_type: crate::datadogV1::model::ApmStatsQueryRowType,
    /// Service name.
    #[serde(rename = "service")]
    pub service: String,
}

impl ApmStatsQueryDefinition {
    pub fn new(
        env: String,
        name: String,
        primary_tag: String,
        row_type: crate::datadogV1::model::ApmStatsQueryRowType,
        service: String,
    ) -> ApmStatsQueryDefinition {
        ApmStatsQueryDefinition {
            columns: None,
            env,
            name,
            primary_tag,
            resource: None,
            row_type,
            service,
        }
    }

    pub fn columns(
        &mut self,
        value: Vec<crate::datadogV1::model::ApmStatsQueryColumnType>,
    ) -> &mut Self {
        self.columns = Some(value);
        self
    }

    pub fn resource(&mut self, value: String) -> &mut Self {
        self.resource = Some(value);
        self
    }
}
