// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Data Datadog collects from Twilio, keyed by dataflow id.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TwilioIntegrationDataflowsResponse {
    /// Twilio Alert resource logs, which detail the errors and warnings raised when Twilio makes a webhook request to your server or when your application calls the Twilio REST API.
    #[serde(rename = "twilio-alerts-logs")]
    pub twilio_alerts_logs:
        Option<crate::datadogV2::model::TwilioAlertsLogsIntegrationDataflowResponse>,
    /// Twilio Call Summary resource logs, covering the metadata and performance of the calls made from your Twilio account. Requires Voice Insights Advanced Features to be enabled on the Twilio account; without it this dataflow collects no data.
    #[serde(rename = "twilio-call-summaries-logs")]
    pub twilio_call_summaries_logs:
        Option<crate::datadogV2::model::TwilioCallSummariesLogsIntegrationDataflowResponse>,
    /// Your Twilio cost data, so that Twilio spend can be broken down and attributed in [Cloud Cost Management](<https://docs.datadoghq.com/cloud_cost_management/>).
    #[serde(rename = "twilio-cloud-cost-metrics")]
    pub twilio_cloud_cost_metrics:
        Option<crate::datadogV2::model::TwilioCloudCostMetricsIntegrationDataflowResponse>,
    /// Twilio Event resource logs, which record virtually every action taken in your Twilio account, such as provisioning a phone number, changing account security settings, or deleting a recording. Actions are recorded whether they came from the REST API, a user in the Twilio Console, or Twilio itself. [Cloud SIEM](<https://docs.datadoghq.com/security/cloud_siem/>) analyzes and correlates these logs to detect threats in real time.
    #[serde(rename = "twilio-events-logs")]
    pub twilio_events_logs:
        Option<crate::datadogV2::model::TwilioEventsLogsIntegrationDataflowResponse>,
    /// Twilio Message resource logs for inbound and outbound messages, used to track delivery and troubleshoot message errors. A log is produced when you send a message through the REST API, when Twilio executes a TwiML instruction, and when someone messages one of your Twilio numbers or channel addresses. Message bodies are never collected.
    #[serde(rename = "twilio-messages-logs")]
    pub twilio_messages_logs:
        Option<crate::datadogV2::model::TwilioMessagesLogsIntegrationDataflowResponse>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TwilioIntegrationDataflowsResponse {
    pub fn new() -> TwilioIntegrationDataflowsResponse {
        TwilioIntegrationDataflowsResponse {
            twilio_alerts_logs: None,
            twilio_call_summaries_logs: None,
            twilio_cloud_cost_metrics: None,
            twilio_events_logs: None,
            twilio_messages_logs: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn twilio_alerts_logs(
        mut self,
        value: crate::datadogV2::model::TwilioAlertsLogsIntegrationDataflowResponse,
    ) -> Self {
        self.twilio_alerts_logs = Some(value);
        self
    }

    pub fn twilio_call_summaries_logs(
        mut self,
        value: crate::datadogV2::model::TwilioCallSummariesLogsIntegrationDataflowResponse,
    ) -> Self {
        self.twilio_call_summaries_logs = Some(value);
        self
    }

    pub fn twilio_cloud_cost_metrics(
        mut self,
        value: crate::datadogV2::model::TwilioCloudCostMetricsIntegrationDataflowResponse,
    ) -> Self {
        self.twilio_cloud_cost_metrics = Some(value);
        self
    }

    pub fn twilio_events_logs(
        mut self,
        value: crate::datadogV2::model::TwilioEventsLogsIntegrationDataflowResponse,
    ) -> Self {
        self.twilio_events_logs = Some(value);
        self
    }

    pub fn twilio_messages_logs(
        mut self,
        value: crate::datadogV2::model::TwilioMessagesLogsIntegrationDataflowResponse,
    ) -> Self {
        self.twilio_messages_logs = Some(value);
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

impl Default for TwilioIntegrationDataflowsResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TwilioIntegrationDataflowsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TwilioIntegrationDataflowsResponseVisitor;
        impl<'a> Visitor<'a> for TwilioIntegrationDataflowsResponseVisitor {
            type Value = TwilioIntegrationDataflowsResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut twilio_alerts_logs: Option<
                    crate::datadogV2::model::TwilioAlertsLogsIntegrationDataflowResponse,
                > = None;
                let mut twilio_call_summaries_logs: Option<
                    crate::datadogV2::model::TwilioCallSummariesLogsIntegrationDataflowResponse,
                > = None;
                let mut twilio_cloud_cost_metrics: Option<
                    crate::datadogV2::model::TwilioCloudCostMetricsIntegrationDataflowResponse,
                > = None;
                let mut twilio_events_logs: Option<
                    crate::datadogV2::model::TwilioEventsLogsIntegrationDataflowResponse,
                > = None;
                let mut twilio_messages_logs: Option<
                    crate::datadogV2::model::TwilioMessagesLogsIntegrationDataflowResponse,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "twilio-alerts-logs" => {
                            if v.is_null() {
                                continue;
                            }
                            twilio_alerts_logs =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "twilio-call-summaries-logs" => {
                            if v.is_null() {
                                continue;
                            }
                            twilio_call_summaries_logs =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "twilio-cloud-cost-metrics" => {
                            if v.is_null() {
                                continue;
                            }
                            twilio_cloud_cost_metrics =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "twilio-events-logs" => {
                            if v.is_null() {
                                continue;
                            }
                            twilio_events_logs =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "twilio-messages-logs" => {
                            if v.is_null() {
                                continue;
                            }
                            twilio_messages_logs =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = TwilioIntegrationDataflowsResponse {
                    twilio_alerts_logs,
                    twilio_call_summaries_logs,
                    twilio_cloud_cost_metrics,
                    twilio_events_logs,
                    twilio_messages_logs,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TwilioIntegrationDataflowsResponseVisitor)
    }
}
