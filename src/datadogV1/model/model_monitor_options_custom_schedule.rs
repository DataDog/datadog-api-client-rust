// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Configuration options for the custom schedule. **This feature is in private beta.**
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorOptionsCustomSchedule {
    /// Array of custom schedule recurrences.
    #[serde(rename = "recurrences")]
    pub recurrences: Option<Vec<crate::datadogV1::model::MonitorOptionsCustomScheduleRecurrence>>,
}

impl MonitorOptionsCustomSchedule {
    pub fn new() -> MonitorOptionsCustomSchedule {
        MonitorOptionsCustomSchedule { recurrences: None }
    }

    pub fn recurrences(
        &mut self,
        value: Vec<crate::datadogV1::model::MonitorOptionsCustomScheduleRecurrence>,
    ) -> &mut Self {
        self.recurrences = Some(value);
        self
    }
}

impl Default for MonitorOptionsCustomSchedule {
    fn default() -> Self {
        Self::new()
    }
}
