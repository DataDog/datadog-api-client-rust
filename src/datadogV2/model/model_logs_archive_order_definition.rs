// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The definition of an archive order.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsArchiveOrderDefinition {
    /// The attributes associated with the archive order.
    #[serde(rename = "attributes")]
    pub attributes: Box<crate::datadogV2::model::LogsArchiveOrderAttributes>,
    /// Type of the archive order definition.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::LogsArchiveOrderDefinitionType,
}

impl LogsArchiveOrderDefinition {
    pub fn new(
        attributes: Box<crate::datadogV2::model::LogsArchiveOrderAttributes>,
        type_: crate::datadogV2::model::LogsArchiveOrderDefinitionType,
    ) -> LogsArchiveOrderDefinition {
        LogsArchiveOrderDefinition { attributes, type_ }
    }
}
