// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Which column and order to sort by
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WidgetFieldSort {
    /// Facet path for the column
    #[serde(rename = "column")]
    pub column: String,
    /// Widget sorting methods.
    #[serde(rename = "order")]
    pub order: crate::datadogV1::model::WidgetSort,
}

impl WidgetFieldSort {
    pub fn new(column: String, order: crate::datadogV1::model::WidgetSort) -> WidgetFieldSort {
        WidgetFieldSort { column, order }
    }
}
