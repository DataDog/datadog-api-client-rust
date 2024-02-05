// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The attributes of the retention filter.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RetentionFilterAttributes {
    /// The creation timestamp of the retention filter.
    #[serde(rename = "created_at")]
    pub created_at: Option<i64>,
    /// The creator of the retention filter.
    #[serde(rename = "created_by")]
    pub created_by: Option<String>,
    /// Shows whether the filter can be edited.
    #[serde(rename = "editable")]
    pub editable: Option<bool>,
    /// The status of the retention filter (Enabled/Disabled).
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// The execution order of the retention filter.
    #[serde(rename = "execution_order")]
    pub execution_order: Option<i64>,
    /// The spans filter used to index spans.
    #[serde(rename = "filter")]
    pub filter: Option<crate::datadogV2::model::SpansFilter>,
    /// The type of retention filter. The value should always be spans-sampling-processor.
    #[serde(rename = "filter_type")]
    pub filter_type: Option<crate::datadogV2::model::RetentionFilterType>,
    /// The modification timestamp of the retention filter.
    #[serde(rename = "modified_at")]
    pub modified_at: Option<i64>,
    /// The modifier of the retention filter.
    #[serde(rename = "modified_by")]
    pub modified_by: Option<String>,
    /// The name of the retention filter.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Sample rate to apply to spans going through this retention filter,
    /// a value of 1.0 keeps all spans matching the query.
    #[serde(rename = "rate")]
    pub rate: Option<f64>,
}

impl RetentionFilterAttributes {
    pub fn new() -> RetentionFilterAttributes {
        RetentionFilterAttributes {
            created_at: None,
            created_by: None,
            editable: None,
            enabled: None,
            execution_order: None,
            filter: None,
            filter_type: None,
            modified_at: None,
            modified_by: None,
            name: None,
            rate: None,
        }
    }

    pub fn created_at(&mut self, value: i64) -> &mut Self {
        self.created_at = Some(value);
        self
    }

    pub fn created_by(&mut self, value: String) -> &mut Self {
        self.created_by = Some(value);
        self
    }

    pub fn editable(&mut self, value: bool) -> &mut Self {
        self.editable = Some(value);
        self
    }

    pub fn enabled(&mut self, value: bool) -> &mut Self {
        self.enabled = Some(value);
        self
    }

    pub fn execution_order(&mut self, value: i64) -> &mut Self {
        self.execution_order = Some(value);
        self
    }

    pub fn filter(&mut self, value: crate::datadogV2::model::SpansFilter) -> &mut Self {
        self.filter = Some(value);
        self
    }

    pub fn filter_type(
        &mut self,
        value: crate::datadogV2::model::RetentionFilterType,
    ) -> &mut Self {
        self.filter_type = Some(value);
        self
    }

    pub fn modified_at(&mut self, value: i64) -> &mut Self {
        self.modified_at = Some(value);
        self
    }

    pub fn modified_by(&mut self, value: String) -> &mut Self {
        self.modified_by = Some(value);
        self
    }

    pub fn name(&mut self, value: String) -> &mut Self {
        self.name = Some(value);
        self
    }

    pub fn rate(&mut self, value: f64) -> &mut Self {
        self.rate = Some(value);
        self
    }
}

impl Default for RetentionFilterAttributes {
    fn default() -> Self {
        Self::new()
    }
}
