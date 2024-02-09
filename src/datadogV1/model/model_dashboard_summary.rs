// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Dashboard summary response.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DashboardSummary {
    /// List of dashboard definitions.
    #[serde(rename = "dashboards")]
    pub dashboards: Option<Vec<crate::datadogV1::model::DashboardSummaryDefinition>>,
}

impl DashboardSummary {
    pub fn new() -> DashboardSummary {
        DashboardSummary { dashboards: None }
    }

    pub fn dashboards(
        &mut self,
        value: Vec<crate::datadogV1::model::DashboardSummaryDefinition>,
    ) -> &mut Self {
        self.dashboards = Some(value);
        self
    }
}

impl Default for DashboardSummary {
    fn default() -> Self {
        Self::new()
    }
}
