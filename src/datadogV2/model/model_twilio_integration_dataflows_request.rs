// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Dataflows to configure on the Twilio integration account, keyed by dataflow id.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TwilioIntegrationDataflowsRequest {
    /// The Twilio alerts logs dataflow.
    #[serde(rename = "twilio-alerts-logs")]
    pub twilio_alerts_logs:
        Option<crate::datadogV2::model::TwilioAlertsLogsIntegrationDataflowRequest>,
    /// The Twilio call summaries logs dataflow.
    #[serde(rename = "twilio-call-summaries-logs")]
    pub twilio_call_summaries_logs:
        Option<crate::datadogV2::model::TwilioCallSummariesLogsIntegrationDataflowRequest>,
    /// The Twilio cloud cost metrics dataflow.
    #[serde(rename = "twilio-cloud-cost-metrics")]
    pub twilio_cloud_cost_metrics:
        Option<crate::datadogV2::model::TwilioCloudCostMetricsIntegrationDataflowRequest>,
    /// The Twilio events logs dataflow.
    #[serde(rename = "twilio-events-logs")]
    pub twilio_events_logs:
        Option<crate::datadogV2::model::TwilioEventsLogsIntegrationDataflowRequest>,
    /// The Twilio messages logs dataflow.
    #[serde(rename = "twilio-messages-logs")]
    pub twilio_messages_logs:
        Option<crate::datadogV2::model::TwilioMessagesLogsIntegrationDataflowRequest>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TwilioIntegrationDataflowsRequest {
    pub fn new() -> TwilioIntegrationDataflowsRequest {
        TwilioIntegrationDataflowsRequest {
            twilio_alerts_logs: None,
            twilio_call_summaries_logs: None,
            twilio_cloud_cost_metrics: None,
            twilio_events_logs: None,
            twilio_messages_logs: None,
            _unparsed: false,
        }
    }

    pub fn twilio_alerts_logs(
        mut self,
        value: crate::datadogV2::model::TwilioAlertsLogsIntegrationDataflowRequest,
    ) -> Self {
        self.twilio_alerts_logs = Some(value);
        self
    }

    pub fn twilio_call_summaries_logs(
        mut self,
        value: crate::datadogV2::model::TwilioCallSummariesLogsIntegrationDataflowRequest,
    ) -> Self {
        self.twilio_call_summaries_logs = Some(value);
        self
    }

    pub fn twilio_cloud_cost_metrics(
        mut self,
        value: crate::datadogV2::model::TwilioCloudCostMetricsIntegrationDataflowRequest,
    ) -> Self {
        self.twilio_cloud_cost_metrics = Some(value);
        self
    }

    pub fn twilio_events_logs(
        mut self,
        value: crate::datadogV2::model::TwilioEventsLogsIntegrationDataflowRequest,
    ) -> Self {
        self.twilio_events_logs = Some(value);
        self
    }

    pub fn twilio_messages_logs(
        mut self,
        value: crate::datadogV2::model::TwilioMessagesLogsIntegrationDataflowRequest,
    ) -> Self {
        self.twilio_messages_logs = Some(value);
        self
    }
}

impl Default for TwilioIntegrationDataflowsRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TwilioIntegrationDataflowsRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TwilioIntegrationDataflowsRequestVisitor;
        impl<'a> Visitor<'a> for TwilioIntegrationDataflowsRequestVisitor {
            type Value = TwilioIntegrationDataflowsRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut twilio_alerts_logs: Option<
                    crate::datadogV2::model::TwilioAlertsLogsIntegrationDataflowRequest,
                > = None;
                let mut twilio_call_summaries_logs: Option<
                    crate::datadogV2::model::TwilioCallSummariesLogsIntegrationDataflowRequest,
                > = None;
                let mut twilio_cloud_cost_metrics: Option<
                    crate::datadogV2::model::TwilioCloudCostMetricsIntegrationDataflowRequest,
                > = None;
                let mut twilio_events_logs: Option<
                    crate::datadogV2::model::TwilioEventsLogsIntegrationDataflowRequest,
                > = None;
                let mut twilio_messages_logs: Option<
                    crate::datadogV2::model::TwilioMessagesLogsIntegrationDataflowRequest,
                > = None;
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
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = TwilioIntegrationDataflowsRequest {
                    twilio_alerts_logs,
                    twilio_call_summaries_logs,
                    twilio_cloud_cost_metrics,
                    twilio_events_logs,
                    twilio_messages_logs,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TwilioIntegrationDataflowsRequestVisitor)
    }
}
