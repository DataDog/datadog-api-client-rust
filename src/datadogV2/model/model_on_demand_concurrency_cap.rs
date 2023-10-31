// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// On-demand concurrency cap.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OnDemandConcurrencyCap {
    /// On-demand concurrency cap attributes.
    #[serde(rename = "attributes")]
    pub attributes: Option<Box<crate::datadogV2::model::OnDemandConcurrencyCapAttributes>>,
    /// On-demand concurrency cap type.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::OnDemandConcurrencyCapType>,
}

impl OnDemandConcurrencyCap {
    pub fn new() -> OnDemandConcurrencyCap {
        OnDemandConcurrencyCap {
            attributes: None,
            type_: None,
        }
    }
}
