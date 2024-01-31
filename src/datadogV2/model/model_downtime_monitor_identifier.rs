// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

/// Monitor identifier for the downtime.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DowntimeMonitorIdentifier {
    DowntimeMonitorIdentifierId(crate::datadogV2::model::DowntimeMonitorIdentifierId),
    DowntimeMonitorIdentifierTags(crate::datadogV2::model::DowntimeMonitorIdentifierTags),
}
