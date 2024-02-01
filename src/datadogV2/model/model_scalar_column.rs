// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

/// A single column in a scalar query response.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ScalarColumn {
    GroupScalarColumn(Box<crate::datadogV2::model::GroupScalarColumn>),
    DataScalarColumn(Box<crate::datadogV2::model::DataScalarColumn>),
}
