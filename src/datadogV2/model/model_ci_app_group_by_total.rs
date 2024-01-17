// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

/// A resulting object to put the given computes in over all the matching records.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CIAppGroupByTotal {
    CIAppGroupByTotalBoolean(bool),
    CIAppGroupByTotalString(String),
    CIAppGroupByTotalNumber(f64),
}
