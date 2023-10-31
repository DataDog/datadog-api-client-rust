// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// IP ranges.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct IPRanges {
    /// Available prefix information for the Agent endpoints.
    #[serde(rename = "agents")]
    pub agents: Option<Box<crate::datadogV1::model::IPPrefixesAgents>>,
    /// Available prefix information for the API endpoints.
    #[serde(rename = "api")]
    pub api: Option<Box<crate::datadogV1::model::IPPrefixesAPI>>,
    /// Available prefix information for the APM endpoints.
    #[serde(rename = "apm")]
    pub apm: Option<Box<crate::datadogV1::model::IPPrefixesAPM>>,
    /// Available prefix information for all Datadog endpoints.
    #[serde(rename = "global")]
    pub global: Option<Box<crate::datadogV1::model::IPPrefixesGlobal>>,
    /// Available prefix information for the Logs endpoints.
    #[serde(rename = "logs")]
    pub logs: Option<Box<crate::datadogV1::model::IPPrefixesLogs>>,
    /// Date when last updated, in the form `YYYY-MM-DD-hh-mm-ss`.
    #[serde(rename = "modified")]
    pub modified: Option<String>,
    /// Available prefix information for the Orchestrator endpoints.
    #[serde(rename = "orchestrator")]
    pub orchestrator: Option<Box<crate::datadogV1::model::IPPrefixesOrchestrator>>,
    /// Available prefix information for the Process endpoints.
    #[serde(rename = "process")]
    pub process: Option<Box<crate::datadogV1::model::IPPrefixesProcess>>,
    /// Available prefix information for the Remote Configuration endpoints.
    #[serde(rename = "remote-configuration")]
    pub remote_configuration: Option<Box<crate::datadogV1::model::IPPrefixesRemoteConfiguration>>,
    /// Available prefix information for the Synthetics endpoints.
    #[serde(rename = "synthetics")]
    pub synthetics: Option<Box<crate::datadogV1::model::IPPrefixesSynthetics>>,
    /// Available prefix information for the Synthetics Private Locations endpoints.
    #[serde(rename = "synthetics-private-locations")]
    pub synthetics_private_locations: Option<Box<crate::datadogV1::model::IPPrefixesSyntheticsPrivateLocations>>,
    /// Version of the IP list.
    #[serde(rename = "version")]
    pub version: Option<i64>,
    /// Available prefix information for the Webhook endpoints.
    #[serde(rename = "webhooks")]
    pub webhooks: Option<Box<crate::datadogV1::model::IPPrefixesWebhooks>>,
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
}
