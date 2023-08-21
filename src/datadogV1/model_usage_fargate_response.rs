// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageFargateResponse {
    /// Array with the number of hourly Fargate tasks recorded for a given organization.
    #[serde(rename = "usage", skip_serializing_if = "Option::is_none")]
    pub usage: Vec<UsageFargateHour>,
}

