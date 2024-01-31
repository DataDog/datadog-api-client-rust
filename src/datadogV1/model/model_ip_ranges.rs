// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// IP ranges.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IPRanges {
    /// Available prefix information for the Agent endpoints.
    #[serde(rename = "agents")]
    pub agents: Option<crate::datadogV1::model::IPPrefixesAgents>,
    /// Available prefix information for the API endpoints.
    #[serde(rename = "api")]
    pub api: Option<crate::datadogV1::model::IPPrefixesAPI>,
    /// Available prefix information for the APM endpoints.
    #[serde(rename = "apm")]
    pub apm: Option<crate::datadogV1::model::IPPrefixesAPM>,
    /// Available prefix information for all Datadog endpoints.
    #[serde(rename = "global")]
    pub global: Option<crate::datadogV1::model::IPPrefixesGlobal>,
    /// Available prefix information for the Logs endpoints.
    #[serde(rename = "logs")]
    pub logs: Option<crate::datadogV1::model::IPPrefixesLogs>,
    /// Date when last updated, in the form `YYYY-MM-DD-hh-mm-ss`.
    #[serde(rename = "modified")]
    pub modified: Option<String>,
    /// Available prefix information for the Orchestrator endpoints.
    #[serde(rename = "orchestrator")]
    pub orchestrator: Option<crate::datadogV1::model::IPPrefixesOrchestrator>,
    /// Available prefix information for the Process endpoints.
    #[serde(rename = "process")]
    pub process: Option<crate::datadogV1::model::IPPrefixesProcess>,
    /// Available prefix information for the Remote Configuration endpoints.
    #[serde(rename = "remote-configuration")]
    pub remote_configuration: Option<crate::datadogV1::model::IPPrefixesRemoteConfiguration>,
    /// Available prefix information for the Synthetics endpoints.
    #[serde(rename = "synthetics")]
    pub synthetics: Option<crate::datadogV1::model::IPPrefixesSynthetics>,
    /// Available prefix information for the Synthetics Private Locations endpoints.
    #[serde(rename = "synthetics-private-locations")]
    pub synthetics_private_locations:
        Option<crate::datadogV1::model::IPPrefixesSyntheticsPrivateLocations>,
    /// Version of the IP list.
    #[serde(rename = "version")]
    pub version: Option<i64>,
    /// Available prefix information for the Webhook endpoints.
    #[serde(rename = "webhooks")]
    pub webhooks: Option<crate::datadogV1::model::IPPrefixesWebhooks>,
}

impl IPRanges {
    pub fn new() -> IPRanges {
        IPRanges {
            agents: None,
            api: None,
            apm: None,
            global: None,
            logs: None,
            modified: None,
            orchestrator: None,
            process: None,
            remote_configuration: None,
            synthetics: None,
            synthetics_private_locations: None,
            version: None,
            webhooks: None,
        }
    }

    pub fn with_agents(&mut self, value: crate::datadogV1::model::IPPrefixesAgents) -> &mut Self {
        self.agents = Some(value);
        self
    }

    pub fn with_api(&mut self, value: crate::datadogV1::model::IPPrefixesAPI) -> &mut Self {
        self.api = Some(value);
        self
    }

    pub fn with_apm(&mut self, value: crate::datadogV1::model::IPPrefixesAPM) -> &mut Self {
        self.apm = Some(value);
        self
    }

    pub fn with_global(&mut self, value: crate::datadogV1::model::IPPrefixesGlobal) -> &mut Self {
        self.global = Some(value);
        self
    }

    pub fn with_logs(&mut self, value: crate::datadogV1::model::IPPrefixesLogs) -> &mut Self {
        self.logs = Some(value);
        self
    }

    pub fn with_modified(&mut self, value: String) -> &mut Self {
        self.modified = Some(value);
        self
    }

    pub fn with_orchestrator(
        &mut self,
        value: crate::datadogV1::model::IPPrefixesOrchestrator,
    ) -> &mut Self {
        self.orchestrator = Some(value);
        self
    }

    pub fn with_process(&mut self, value: crate::datadogV1::model::IPPrefixesProcess) -> &mut Self {
        self.process = Some(value);
        self
    }

    pub fn with_remote_configuration(
        &mut self,
        value: crate::datadogV1::model::IPPrefixesRemoteConfiguration,
    ) -> &mut Self {
        self.remote_configuration = Some(value);
        self
    }

    pub fn with_synthetics(
        &mut self,
        value: crate::datadogV1::model::IPPrefixesSynthetics,
    ) -> &mut Self {
        self.synthetics = Some(value);
        self
    }

    pub fn with_synthetics_private_locations(
        &mut self,
        value: crate::datadogV1::model::IPPrefixesSyntheticsPrivateLocations,
    ) -> &mut Self {
        self.synthetics_private_locations = Some(value);
        self
    }

    pub fn with_version(&mut self, value: i64) -> &mut Self {
        self.version = Some(value);
        self
    }

    pub fn with_webhooks(
        &mut self,
        value: crate::datadogV1::model::IPPrefixesWebhooks,
    ) -> &mut Self {
        self.webhooks = Some(value);
        self
    }
}
impl Default for IPRanges {
    fn default() -> Self {
        Self::new()
    }
}
