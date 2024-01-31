// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A recurring downtime schedule definition.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DowntimeScheduleRecurrencesUpdateRequest {
    /// A list of downtime recurrences.
    #[serde(rename = "recurrences")]
    pub recurrences:
        Option<Vec<crate::datadogV2::model::DowntimeScheduleRecurrenceCreateUpdateRequest>>,
    /// The timezone in which to schedule the downtime.
    #[serde(rename = "timezone")]
    pub timezone: Option<String>,
}

impl DowntimeScheduleRecurrencesUpdateRequest {
    pub fn new() -> DowntimeScheduleRecurrencesUpdateRequest {
        DowntimeScheduleRecurrencesUpdateRequest {
            recurrences: None,
            timezone: None,
        }
    }

    pub fn with_recurrences(
        &mut self,
        value: Vec<crate::datadogV2::model::DowntimeScheduleRecurrenceCreateUpdateRequest>,
    ) -> &mut Self {
        self.recurrences = Some(value);
        self
    }

    pub fn with_timezone(&mut self, value: String) -> &mut Self {
        self.timezone = Some(value);
        self
    }
}
impl Default for DowntimeScheduleRecurrencesUpdateRequest {
    fn default() -> Self {
        Self::new()
    }
}
