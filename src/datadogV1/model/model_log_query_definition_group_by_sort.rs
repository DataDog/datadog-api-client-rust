// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Define a sorting method.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogQueryDefinitionGroupBySort {
    /// The aggregation method.
    #[serde(rename = "aggregation")]
    pub aggregation: String,
    /// Facet name.
    #[serde(rename = "facet")]
    pub facet: Option<String>,
    /// Widget sorting methods.
    #[serde(rename = "order")]
    pub order: crate::datadogV1::model::WidgetSort,
}

impl LogQueryDefinitionGroupBySort {
    pub fn new(
        aggregation: String,
        order: crate::datadogV1::model::WidgetSort,
    ) -> LogQueryDefinitionGroupBySort {
        LogQueryDefinitionGroupBySort {
            aggregation,
            facet: None,
            order,
        }
    }

    pub fn with_facet(&mut self, value: String) -> &mut Self {
        self.facet = Some(value);
        self
    }
}
