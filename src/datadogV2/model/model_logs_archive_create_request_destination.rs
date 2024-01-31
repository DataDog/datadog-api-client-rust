// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

/// An archive's destination.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LogsArchiveCreateRequestDestination {
    LogsArchiveDestinationAzure(crate::datadogV2::model::LogsArchiveDestinationAzure),
    LogsArchiveDestinationGCS(crate::datadogV2::model::LogsArchiveDestinationGCS),
    LogsArchiveDestinationS3(crate::datadogV2::model::LogsArchiveDestinationS3),
}
