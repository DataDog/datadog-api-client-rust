// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MuteFindingRequestAttributes {
    /// Object containing the new mute properties of the finding.
    #[serde(rename = "mute")]
    pub mute: Box<crate::datadogV2::model::MuteFindingRequestProperties>,
}

impl MuteFindingRequestAttributes {
    /// The mute properties to be updated.
    pub fn new(mute: crate::datadogV2::model::MuteFindingRequestProperties) -> MuteFindingRequestAttributes {
        MuteFindingRequestAttributes { mute: Box::new(mute) }
    }
}
