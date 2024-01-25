// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Attributes to create a DORA deployment event.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DORADeploymentRequestAttributes {
    /// Unix timestamp in nanoseconds when the deployment finished. It should not be older than 3 hours.
    #[serde(rename = "finished_at")]
    pub finished_at: i64,
    /// Git info for DORA Metrics events.
    #[serde(rename = "git")]
    pub git: Option<Box<crate::datadogV2::model::DORAGitInfo>>,
    /// Deployment ID.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Service name from a service available in the Service Catalog.
    #[serde(rename = "service")]
    pub service: String,
    /// Unix timestamp in nanoseconds when the deployment started.
    #[serde(rename = "started_at")]
    pub started_at: i64,
    /// Version to correlate with [APM Deployment Tracking](<https://docs.datadoghq.com/tracing/services/deployment_tracking/>).
    #[serde(rename = "version")]
    pub version: Option<String>,
}

impl DORADeploymentRequestAttributes {
    pub fn new(
        finished_at: i64,
        service: String,
        started_at: i64,
    ) -> DORADeploymentRequestAttributes {
        DORADeploymentRequestAttributes {
            finished_at,
            git: None,
            id: None,
            service,
            started_at,
            version: None,
        }
    }
}
