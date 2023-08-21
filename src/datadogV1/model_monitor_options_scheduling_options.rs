// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorOptionsSchedulingOptions {
    /// Configuration options for the evaluation window. If `hour_starts` is set, no other fields may be set. Otherwise, `day_starts` and `month_starts` must be set together.
    #[serde(rename = "evaluation_window", skip_serializing_if = "Option::is_none")]
    pub evaluation_window: MonitorOptionsSchedulingOptionsEvaluationWindow,
}

