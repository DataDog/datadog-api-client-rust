// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

/// An object related to the configuration.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SensitiveDataScannerGetConfigIncludedItem {
    SensitiveDataScannerRuleIncludedItem(
        crate::datadogV2::model::SensitiveDataScannerRuleIncludedItem,
    ),
    SensitiveDataScannerGroupIncludedItem(
        crate::datadogV2::model::SensitiveDataScannerGroupIncludedItem,
    ),
}
