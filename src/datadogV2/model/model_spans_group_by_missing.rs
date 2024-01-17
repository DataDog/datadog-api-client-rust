// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

/// The value to use for spans that don't have the facet used to group by.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SpansGroupByMissing {
    SpansGroupByMissingString(Box<String>),
    SpansGroupByMissingNumber(Box<f64>),
}
