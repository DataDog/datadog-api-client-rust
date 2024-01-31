// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Attributes to create a DORA incident event.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DORAIncidentRequestAttributes {
    /// Unix timestamp in nanoseconds when the incident finished. It should not be older than 3 hours.
    #[serde(rename = "finished_at")]
    pub finished_at: Option<i64>,
    /// Git info for DORA Metrics events.
    #[serde(rename = "git")]
    pub git: Option<crate::datadogV2::model::DORAGitInfo>,
    /// Incident ID
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Incident name.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Service name from a service available in the Service Catalog.
    #[serde(rename = "service")]
    pub service: String,
    /// Incident severity.
    #[serde(rename = "severity")]
    pub severity: Option<String>,
    /// Unix timestamp in nanoseconds when the incident started.
    #[serde(rename = "started_at")]
    pub started_at: i64,
    /// Version to correlate with [APM Deployment Tracking](<https://docs.datadoghq.com/tracing/services/deployment_tracking/>).
    #[serde(rename = "version")]
    pub version: Option<String>,
}

impl DORAIncidentRequestAttributes {
    pub fn new(service: String, started_at: i64) -> DORAIncidentRequestAttributes {
        DORAIncidentRequestAttributes {
            finished_at: None,
            git: None,
            id: None,
            name: None,
            service,
            severity: None,
            started_at,
            version: None,
        }
    }

    pub fn with_finished_at(&mut self, value: i64) -> &mut Self {
        self.finished_at = Some(value);
        self
    }

    pub fn with_git(&mut self, value: crate::datadogV2::model::DORAGitInfo) -> &mut Self {
        self.git = Some(value);
        self
    }

    pub fn with_id(&mut self, value: String) -> &mut Self {
        self.id = Some(value);
        self
    }

    pub fn with_name(&mut self, value: String) -> &mut Self {
        self.name = Some(value);
        self
    }

    pub fn with_severity(&mut self, value: String) -> &mut Self {
        self.severity = Some(value);
        self
    }

    pub fn with_version(&mut self, value: String) -> &mut Self {
        self.version = Some(value);
        self
    }
}
