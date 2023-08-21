// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScalarMeta {
    /// Detailed information about the unit.
First element describes the "primary unit" (for example, `bytes` in `bytes per second`).
The second element describes the "per unit" (for example, `second` in `bytes per second`).
If the second element is not present, the API returns null.
    #[serde(rename = "unit", skip_serializing_if = "Option::is_none")]
    pub unit: Vec<Unit>,
}

