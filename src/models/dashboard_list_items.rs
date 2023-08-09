/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// DashboardListItems : Dashboards within a list.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DashboardListItems {
    /// List of dashboards in the dashboard list.
    #[serde(rename = "dashboards")]
    pub dashboards: Vec<crate::models::DashboardListItem>,
    /// Number of dashboards in the dashboard list.
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}

impl DashboardListItems {
    /// Dashboards within a list.
    pub fn new(dashboards: Vec<crate::models::DashboardListItem>) -> DashboardListItems {
        DashboardListItems {
            dashboards,
            total: None,
        }
    }
}


