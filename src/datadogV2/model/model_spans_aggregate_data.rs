// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The object containing the query content.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpansAggregateData {
    /// The object containing all the query parameters.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::SpansAggregateRequestAttributes>,
    /// The type of resource. The value should always be aggregate_request.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::SpansAggregateRequestType>,
}

impl SpansAggregateData {
    pub fn new() -> SpansAggregateData {
        SpansAggregateData {
            attributes: None,
            type_: None,
        }
    }

    pub fn attributes(
        &mut self,
        value: crate::datadogV2::model::SpansAggregateRequestAttributes,
    ) -> &mut Self {
        self.attributes = Some(value);
        self
    }

    pub fn type_(
        &mut self,
        value: crate::datadogV2::model::SpansAggregateRequestType,
    ) -> &mut Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for SpansAggregateData {
    fn default() -> Self {
        Self::new()
    }
}
