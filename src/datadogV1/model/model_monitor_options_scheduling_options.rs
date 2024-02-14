// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Configuration options for scheduling.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorOptionsSchedulingOptions {
    /// Configuration options for the custom schedule. **This feature is in private beta.**
    #[serde(rename = "custom_schedule")]
    pub custom_schedule: Option<crate::datadogV1::model::MonitorOptionsCustomSchedule>,
    /// Configuration options for the evaluation window. If `hour_starts` is set, no other fields may be set. Otherwise, `day_starts` and `month_starts` must be set together.
    #[serde(rename = "evaluation_window")]
    pub evaluation_window:
        Option<crate::datadogV1::model::MonitorOptionsSchedulingOptionsEvaluationWindow>,
}

impl MonitorOptionsSchedulingOptions {
    pub fn new() -> MonitorOptionsSchedulingOptions {
        MonitorOptionsSchedulingOptions {
            custom_schedule: None,
            evaluation_window: None,
        }
    }

    pub fn custom_schedule(
        &mut self,
        value: crate::datadogV1::model::MonitorOptionsCustomSchedule,
    ) -> &mut Self {
        self.custom_schedule = Some(value);
        self
    }

    pub fn evaluation_window(
        &mut self,
        value: crate::datadogV1::model::MonitorOptionsSchedulingOptionsEvaluationWindow,
    ) -> &mut Self {
        self.evaluation_window = Some(value);
        self
    }
}

impl Default for MonitorOptionsSchedulingOptions {
    fn default() -> Self {
        Self::new()
    }
}
