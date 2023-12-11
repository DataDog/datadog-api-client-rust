// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The mute properties to be updated.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MuteFindingRequestAttributes {
    /// Object containing the new mute properties of the finding.
    #[serde(rename = "mute")]
    pub mute: Box<crate::datadogV2::model::MuteFindingRequestProperties>,
}

impl MuteFindingRequestAttributes {
    pub fn new(
        mute: crate::datadogV2::model::MuteFindingRequestProperties,
    ) -> MuteFindingRequestAttributes {
        MuteFindingRequestAttributes {
            mute: Box::new(mute),
        }
    }
}
