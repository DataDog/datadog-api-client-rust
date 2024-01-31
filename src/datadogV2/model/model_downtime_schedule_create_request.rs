// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

/// Schedule for the downtime.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DowntimeScheduleCreateRequest {
    DowntimeScheduleRecurrencesCreateRequest(
        crate::datadogV2::model::DowntimeScheduleRecurrencesCreateRequest,
    ),
    DowntimeScheduleOneTimeCreateUpdateRequest(
        crate::datadogV2::model::DowntimeScheduleOneTimeCreateUpdateRequest,
    ),
}
