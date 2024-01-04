// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Spans aggregate.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpansAggregateBucket {
    /// A bucket values.
    #[serde(rename = "attributes")]
    pub attributes: Option<Box<crate::datadogV2::model::SpansAggregateBucketAttributes>>,
    /// ID of the spans aggregate.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The spans aggregate bucket type.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::SpansAggregateBucketType>,
}

impl SpansAggregateBucket {
    pub fn new() -> SpansAggregateBucket {
        SpansAggregateBucket {
            attributes: None,
            id: None,
            type_: None,
        }
    }
}