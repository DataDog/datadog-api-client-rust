// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The user who created the downtime.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DowntimeRelationshipsCreatedBy {
    /// Data for the user who created the downtime.
    #[serde(rename = "data", default, with = "::serde_with::rust::double_option")]
    pub data: Option<Option<Box<crate::datadogV2::model::DowntimeRelationshipsCreatedByData>>>,
}

impl DowntimeRelationshipsCreatedBy {
    pub fn new() -> DowntimeRelationshipsCreatedBy {
        DowntimeRelationshipsCreatedBy { data: None }
    }
}