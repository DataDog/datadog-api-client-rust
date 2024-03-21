// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// IP ranges.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
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
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
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
            _unparsed: false,
        }
    }

    pub fn agents(mut self, value: crate::datadogV1::model::IPPrefixesAgents) -> Self {
        self.agents = Some(value);
        self
    }

    pub fn api(mut self, value: crate::datadogV1::model::IPPrefixesAPI) -> Self {
        self.api = Some(value);
        self
    }

    pub fn apm(mut self, value: crate::datadogV1::model::IPPrefixesAPM) -> Self {
        self.apm = Some(value);
        self
    }

    pub fn global(mut self, value: crate::datadogV1::model::IPPrefixesGlobal) -> Self {
        self.global = Some(value);
        self
    }

    pub fn logs(mut self, value: crate::datadogV1::model::IPPrefixesLogs) -> Self {
        self.logs = Some(value);
        self
    }

    pub fn modified(mut self, value: String) -> Self {
        self.modified = Some(value);
        self
    }

    pub fn orchestrator(mut self, value: crate::datadogV1::model::IPPrefixesOrchestrator) -> Self {
        self.orchestrator = Some(value);
        self
    }

    pub fn process(mut self, value: crate::datadogV1::model::IPPrefixesProcess) -> Self {
        self.process = Some(value);
        self
    }

    pub fn remote_configuration(
        mut self,
        value: crate::datadogV1::model::IPPrefixesRemoteConfiguration,
    ) -> Self {
        self.remote_configuration = Some(value);
        self
    }

    pub fn synthetics(mut self, value: crate::datadogV1::model::IPPrefixesSynthetics) -> Self {
        self.synthetics = Some(value);
        self
    }

    pub fn synthetics_private_locations(
        mut self,
        value: crate::datadogV1::model::IPPrefixesSyntheticsPrivateLocations,
    ) -> Self {
        self.synthetics_private_locations = Some(value);
        self
    }

    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    pub fn webhooks(mut self, value: crate::datadogV1::model::IPPrefixesWebhooks) -> Self {
        self.webhooks = Some(value);
        self
    }
}

impl Default for IPRanges {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IPRanges {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IPRangesVisitor;
        impl<'a> Visitor<'a> for IPRangesVisitor {
            type Value = IPRanges;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut agents: Option<crate::datadogV1::model::IPPrefixesAgents> = None;
                let mut api: Option<crate::datadogV1::model::IPPrefixesAPI> = None;
                let mut apm: Option<crate::datadogV1::model::IPPrefixesAPM> = None;
                let mut global: Option<crate::datadogV1::model::IPPrefixesGlobal> = None;
                let mut logs: Option<crate::datadogV1::model::IPPrefixesLogs> = None;
                let mut modified: Option<String> = None;
                let mut orchestrator: Option<crate::datadogV1::model::IPPrefixesOrchestrator> =
                    None;
                let mut process: Option<crate::datadogV1::model::IPPrefixesProcess> = None;
                let mut remote_configuration: Option<
                    crate::datadogV1::model::IPPrefixesRemoteConfiguration,
                > = None;
                let mut synthetics: Option<crate::datadogV1::model::IPPrefixesSynthetics> = None;
                let mut synthetics_private_locations: Option<
                    crate::datadogV1::model::IPPrefixesSyntheticsPrivateLocations,
                > = None;
                let mut version: Option<i64> = None;
                let mut webhooks: Option<crate::datadogV1::model::IPPrefixesWebhooks> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "agents" => {
                            if v.is_null() {
                                continue;
                            }
                            agents = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "api" => {
                            if v.is_null() {
                                continue;
                            }
                            api = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "apm" => {
                            if v.is_null() {
                                continue;
                            }
                            apm = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "global" => {
                            if v.is_null() {
                                continue;
                            }
                            global = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "logs" => {
                            if v.is_null() {
                                continue;
                            }
                            logs = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified" => {
                            if v.is_null() {
                                continue;
                            }
                            modified = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "orchestrator" => {
                            if v.is_null() {
                                continue;
                            }
                            orchestrator =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "process" => {
                            if v.is_null() {
                                continue;
                            }
                            process = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "remote-configuration" => {
                            if v.is_null() {
                                continue;
                            }
                            remote_configuration =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "synthetics" => {
                            if v.is_null() {
                                continue;
                            }
                            synthetics = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "synthetics-private-locations" => {
                            if v.is_null() {
                                continue;
                            }
                            synthetics_private_locations =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version" => {
                            if v.is_null() {
                                continue;
                            }
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "webhooks" => {
                            if v.is_null() {
                                continue;
                            }
                            webhooks = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = IPRanges {
                    agents,
                    api,
                    apm,
                    global,
                    logs,
                    modified,
                    orchestrator,
                    process,
                    remote_configuration,
                    synthetics,
                    synthetics_private_locations,
                    version,
                    webhooks,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IPRangesVisitor)
    }
}
