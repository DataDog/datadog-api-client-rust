// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object description of a spans after being processed and stored by Datadog.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Span {
    /// JSON object containing all span attributes and their associated values.
    #[serde(rename = "attributes")]
    pub attributes: Option<Box<crate::datadogV2::model::SpansAttributes>>,
    /// Unique ID of the Span.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Type of the span.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::SpansType>,
}

impl Span {
    pub fn new() -> Span {
        Span {
            attributes: None,
            id: None,
            type_: None,
        }
    }
}
