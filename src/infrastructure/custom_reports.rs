//! Custom reports and analytics system
//!
//! Provides report generation, templates, filtering, and export capabilities
//! for comprehensive project analytics and insights.
//!
//! # Examples
//!
//! ```
//! use toad::infrastructure::custom_reports::{ReportBuilder, ReportFormat};
//!
//! let report = ReportBuilder::new("Weekly Summary")
//!     .add_filter("status", "completed")
//!     .format(ReportFormat::Json)
//!     .build();
//! ```

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Report format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReportFormat {
    /// JSON format
    Json,
    /// CSV format
    Csv,
    /// Markdown format
    Markdown,
    /// HTML format
    Html,
    /// Plain text
    Text,
}

impl ReportFormat {
    /// Get all formats
    pub fn all() -> &'static [ReportFormat] {
        &[
            ReportFormat::Json,
            ReportFormat::Csv,
            ReportFormat::Markdown,
            ReportFormat::Html,
            ReportFormat::Text,
        ]
    }

    /// Get format name
    pub fn name(&self) -> &'static str {
        match self {
            ReportFormat::Json => "JSON",
            ReportFormat::Csv => "CSV",
            ReportFormat::Markdown => "Markdown",
            ReportFormat::Html => "HTML",
            ReportFormat::Text => "Text",
        }
    }

    /// Get file extension
    pub fn extension(&self) -> &'static str {
        match self {
            ReportFormat::Json => "json",
            ReportFormat::Csv => "csv",
            ReportFormat::Markdown => "md",
            ReportFormat::Html => "html",
            ReportFormat::Text => "txt",
        }
    }
}

/// Report frequency for scheduled reports
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReportFrequency {
    /// Daily reports
    Daily,
    /// Weekly reports
    Weekly,
    /// Monthly reports
    Monthly,
    /// Quarterly reports
    Quarterly,
    /// One-time report
    Once,
}

impl ReportFrequency {
    /// Get frequency name
    pub fn name(&self) -> &'static str {
        match self {
            ReportFrequency::Daily => "Daily",
            ReportFrequency::Weekly => "Weekly",
            ReportFrequency::Monthly => "Monthly",
            ReportFrequency::Quarterly => "Quarterly",
            ReportFrequency::Once => "Once",
        }
    }
}

/// Report type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReportType {
    /// Task summary report
    TaskSummary,
    /// Time tracking report
    TimeTracking,
    /// Achievement report
    Achievements,
    /// Project status report
    ProjectStatus,
    /// Team performance report
    TeamPerformance,
    /// Custom report
    Custom,
}

impl ReportType {
    /// Get type name
    pub fn name(&self) -> &'static str {
        match self {
            ReportType::TaskSummary => "Task Summary",
            ReportType::TimeTracking => "Time Tracking",
            ReportType::Achievements => "Achievements",
            ReportType::ProjectStatus => "Project Status",
            ReportType::TeamPerformance => "Team Performance",
            ReportType::Custom => "Custom",
        }
    }
}

/// Report filter condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportFilter {
    /// Field to filter on
    pub field: String,
    /// Operator (equals, contains, greater_than, etc.)
    pub operator: String,
    /// Value to filter by
    pub value: String,
}

impl ReportFilter {
    /// Create a new filter
    pub fn new(
        field: impl Into<String>,
        operator: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        Self {
            field: field.into(),
            operator: operator.into(),
            value: value.into(),
        }
    }

    /// Create an equals filter
    pub fn equals(field: impl Into<String>, value: impl Into<String>) -> Self {
        Self::new(field, "equals", value)
    }

    /// Create a contains filter
    pub fn contains(field: impl Into<String>, value: impl Into<String>) -> Self {
        Self::new(field, "contains", value)
    }

    /// Create a greater than filter
    pub fn greater_than(field: impl Into<String>, value: impl Into<String>) -> Self {
        Self::new(field, "greater_than", value)
    }

    /// Create a less than filter
    pub fn less_than(field: impl Into<String>, value: impl Into<String>) -> Self {
        Self::new(field, "less_than", value)
    }
}

/// Report data row
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportRow {
    /// Row data as key-value pairs
    pub data: HashMap<String, String>,
}

impl ReportRow {
    /// Create a new row
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    /// Add a field
    pub fn add_field(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.data.insert(key.into(), value.into());
        self
    }

    /// Get field value
    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }
}

impl Default for ReportRow {
    fn default() -> Self {
        Self::new()
    }
}

/// Generated report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    /// Report ID
    pub id: String,
    /// Report title
    pub title: String,
    /// Report type
    pub report_type: ReportType,
    /// Generated timestamp
    pub generated_at: DateTime<Utc>,
    /// Report format
    pub format: ReportFormat,
    /// Column headers
    pub columns: Vec<String>,
    /// Report data rows
    pub rows: Vec<ReportRow>,
    /// Report summary/metadata
    pub summary: HashMap<String, String>,
}

impl Report {
    /// Create a new report
    pub fn new(title: impl Into<String>, report_type: ReportType) -> Self {
        Self {
            id: format!("report-{}", Utc::now().timestamp()),
            title: title.into(),
            report_type,
            generated_at: Utc::now(),
            format: ReportFormat::Json,
            columns: Vec::new(),
            rows: Vec::new(),
            summary: HashMap::new(),
        }
    }

    /// Add column
    pub fn add_column(&mut self, name: impl Into<String>) {
        self.columns.push(name.into());
    }

    /// Add row
    pub fn add_row(&mut self, row: ReportRow) {
        self.rows.push(row);
    }

    /// Add summary field
    pub fn add_summary(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.summary.insert(key.into(), value.into());
    }

    /// Get row count
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    /// Export to JSON
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_default()
    }

    /// Export to CSV
    pub fn to_csv(&self) -> String {
        let mut output = String::new();

        // Header
        output.push_str(&self.columns.join(","));
        output.push('\n');

        // Rows
        for row in &self.rows {
            let values: Vec<String> = self
                .columns
                .iter()
                .map(|col| row.get(col).cloned().unwrap_or_default())
                .collect();
            output.push_str(&values.join(","));
            output.push('\n');
        }

        output
    }

    /// Export to Markdown
    pub fn to_markdown(&self) -> String {
        let mut output = String::new();

        // Title
        output.push_str(&format!("# {}\n\n", self.title));
        output.push_str(&format!("Generated: {}\n\n", self.generated_at));

        // Summary
        if !self.summary.is_empty() {
            output.push_str("## Summary\n\n");
            for (key, value) in &self.summary {
                output.push_str(&format!("- **{}**: {}\n", key, value));
            }
            output.push('\n');
        }

        // Table
        output.push_str("## Data\n\n");

        // Header
        output.push_str("| ");
        output.push_str(&self.columns.join(" | "));
        output.push_str(" |\n");

        // Separator
        output.push_str("| ");
        output.push_str(&vec!["---"; self.columns.len()].join(" | "));
        output.push_str(" |\n");

        // Rows
        for row in &self.rows {
            output.push_str("| ");
            let values: Vec<String> = self
                .columns
                .iter()
                .map(|col| row.get(col).cloned().unwrap_or_default())
                .collect();
            output.push_str(&values.join(" | "));
            output.push_str(" |\n");
        }

        output
    }
}

/// Report template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportTemplate {
    /// Template ID
    pub id: String,
    /// Template name
    pub name: String,
    /// Template description
    pub description: String,
    /// Report type
    pub report_type: ReportType,
    /// Format
    pub format: ReportFormat,
    /// Filters
    pub filters: Vec<ReportFilter>,
    /// Columns to include
    pub columns: Vec<String>,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
}

impl ReportTemplate {
    /// Create a new template
    pub fn new(name: impl Into<String>, report_type: ReportType) -> Self {
        Self {
            id: format!("template-{}", Utc::now().timestamp()),
            name: name.into(),
            description: String::new(),
            report_type,
            format: ReportFormat::Json,
            filters: Vec::new(),
            columns: Vec::new(),
            created_at: Utc::now(),
        }
    }

    /// Set description
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    /// Set format
    pub fn format(mut self, format: ReportFormat) -> Self {
        self.format = format;
        self
    }

    /// Add filter
    pub fn add_filter(mut self, filter: ReportFilter) -> Self {
        self.filters.push(filter);
        self
    }

    /// Add column
    pub fn add_column(mut self, column: impl Into<String>) -> Self {
        self.columns.push(column.into());
        self
    }
}

/// Report builder
#[derive(Debug)]
pub struct ReportBuilder {
    title: String,
    report_type: ReportType,
    format: ReportFormat,
    filters: Vec<ReportFilter>,
    columns: Vec<String>,
}

impl ReportBuilder {
    /// Create a new report builder
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            report_type: ReportType::Custom,
            format: ReportFormat::Json,
            filters: Vec::new(),
            columns: Vec::new(),
        }
    }

    /// Set report type
    pub fn report_type(mut self, report_type: ReportType) -> Self {
        self.report_type = report_type;
        self
    }

    /// Set format
    pub fn format(mut self, format: ReportFormat) -> Self {
        self.format = format;
        self
    }

    /// Add filter
    pub fn add_filter(mut self, field: impl Into<String>, value: impl Into<String>) -> Self {
        self.filters.push(ReportFilter::equals(field, value));
        self
    }

    /// Add custom filter
    pub fn add_custom_filter(mut self, filter: ReportFilter) -> Self {
        self.filters.push(filter);
        self
    }

    /// Add column
    pub fn add_column(mut self, column: impl Into<String>) -> Self {
        self.columns.push(column.into());
        self
    }

    /// Build the report structure
    pub fn build(self) -> Report {
        let mut report = Report::new(self.title, self.report_type);
        report.format = self.format;
        for column in self.columns {
            report.add_column(column);
        }
        report
    }

    /// Build report from template
    pub fn from_template(template: &ReportTemplate) -> Self {
        let mut builder = Self::new(template.name.clone())
            .report_type(template.report_type)
            .format(template.format);

        for filter in &template.filters {
            builder = builder.add_custom_filter(filter.clone());
        }

        for column in &template.columns {
            builder = builder.add_column(column.clone());
        }

        builder
    }
}

/// Scheduled report configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledReport {
    /// Schedule ID
    pub id: String,
    /// Report template ID
    pub template_id: String,
    /// Report frequency
    pub frequency: ReportFrequency,
    /// Enabled status
    pub enabled: bool,
    /// Last run timestamp
    pub last_run: Option<DateTime<Utc>>,
    /// Next run timestamp
    pub next_run: Option<DateTime<Utc>>,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
}

impl ScheduledReport {
    /// Create a new scheduled report
    pub fn new(template_id: impl Into<String>, frequency: ReportFrequency) -> Self {
        Self {
            id: format!("schedule-{}", Utc::now().timestamp()),
            template_id: template_id.into(),
            frequency,
            enabled: true,
            last_run: None,
            next_run: None,
            created_at: Utc::now(),
        }
    }

    /// Enable/disable schedule
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

/// Report manager
///
/// Manages report generation, templates, and scheduled reports.
#[derive(Debug)]
pub struct ReportManager {
    /// Generated reports
    reports: HashMap<String, Report>,
    /// Report templates
    templates: HashMap<String, ReportTemplate>,
    /// Scheduled reports
    schedules: HashMap<String, ScheduledReport>,
}

impl ReportManager {
    /// Create a new report manager
    pub fn new() -> Self {
        Self {
            reports: HashMap::new(),
            templates: HashMap::new(),
            schedules: HashMap::new(),
        }
    }

    /// Save a report
    pub fn save_report(&mut self, report: Report) -> String {
        let id = report.id.clone();
        self.reports.insert(id.clone(), report);
        id
    }

    /// Get report by ID
    pub fn get_report(&self, id: &str) -> Option<&Report> {
        self.reports.get(id)
    }

    /// Delete report
    pub fn delete_report(&mut self, id: &str) -> bool {
        self.reports.remove(id).is_some()
    }

    /// Get all reports
    pub fn all_reports(&self) -> Vec<&Report> {
        self.reports.values().collect()
    }

    /// Save a template
    pub fn save_template(&mut self, template: ReportTemplate) -> String {
        let id = template.id.clone();
        self.templates.insert(id.clone(), template);
        id
    }

    /// Get template by ID
    pub fn get_template(&self, id: &str) -> Option<&ReportTemplate> {
        self.templates.get(id)
    }

    /// Delete template
    pub fn delete_template(&mut self, id: &str) -> bool {
        self.templates.remove(id).is_some()
    }

    /// Get all templates
    pub fn all_templates(&self) -> Vec<&ReportTemplate> {
        self.templates.values().collect()
    }

    /// Schedule a report
    pub fn schedule_report(
        &mut self,
        template_id: impl Into<String>,
        frequency: ReportFrequency,
    ) -> String {
        let schedule = ScheduledReport::new(template_id, frequency);
        let id = schedule.id.clone();
        self.schedules.insert(id.clone(), schedule);
        id
    }

    /// Get scheduled report
    pub fn get_schedule(&self, id: &str) -> Option<&ScheduledReport> {
        self.schedules.get(id)
    }

    /// Get mutable scheduled report
    pub fn get_schedule_mut(&mut self, id: &str) -> Option<&mut ScheduledReport> {
        self.schedules.get_mut(id)
    }

    /// Delete schedule
    pub fn delete_schedule(&mut self, id: &str) -> bool {
        self.schedules.remove(id).is_some()
    }

    /// Get all schedules
    pub fn all_schedules(&self) -> Vec<&ScheduledReport> {
        self.schedules.values().collect()
    }

    /// Get enabled schedules
    pub fn enabled_schedules(&self) -> Vec<&ScheduledReport> {
        self.schedules.values().filter(|s| s.enabled).collect()
    }

    /// Generate report from template
    pub fn generate_from_template(&mut self, template_id: &str) -> Option<String> {
        let template = self.templates.get(template_id)?;
        let report = ReportBuilder::from_template(template).build();
        Some(self.save_report(report))
    }
}

impl Default for ReportManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_report_format_all() {
        let formats = ReportFormat::all();
        assert_eq!(formats.len(), 5);
    }

    #[test]
    fn test_report_format_name() {
        assert_eq!(ReportFormat::Json.name(), "JSON");
        assert_eq!(ReportFormat::Csv.name(), "CSV");
    }

    #[test]
    fn test_report_format_extension() {
        assert_eq!(ReportFormat::Json.extension(), "json");
        assert_eq!(ReportFormat::Markdown.extension(), "md");
    }

    #[test]
    fn test_report_frequency_name() {
        assert_eq!(ReportFrequency::Daily.name(), "Daily");
        assert_eq!(ReportFrequency::Weekly.name(), "Weekly");
    }

    #[test]
    fn test_report_type_name() {
        assert_eq!(ReportType::TaskSummary.name(), "Task Summary");
        assert_eq!(ReportType::TimeTracking.name(), "Time Tracking");
    }

    #[test]
    fn test_report_filter_creation() {
        let filter = ReportFilter::new("status", "equals", "completed");
        assert_eq!(filter.field, "status");
        assert_eq!(filter.operator, "equals");
        assert_eq!(filter.value, "completed");
    }

    #[test]
    fn test_report_filter_helpers() {
        let filter = ReportFilter::equals("priority", "high");
        assert_eq!(filter.operator, "equals");

        let filter = ReportFilter::contains("title", "bug");
        assert_eq!(filter.operator, "contains");

        let filter = ReportFilter::greater_than("count", "10");
        assert_eq!(filter.operator, "greater_than");
    }

    #[test]
    fn test_report_row_creation() {
        let row = ReportRow::new()
            .add_field("name", "Task 1")
            .add_field("status", "completed");

        assert_eq!(row.get("name"), Some(&"Task 1".to_string()));
        assert_eq!(row.get("status"), Some(&"completed".to_string()));
    }

    #[test]
    fn test_report_creation() {
        let report = Report::new("Weekly Summary", ReportType::TaskSummary);
        assert_eq!(report.title, "Weekly Summary");
        assert_eq!(report.report_type, ReportType::TaskSummary);
        assert_eq!(report.row_count(), 0);
    }

    #[test]
    fn test_report_add_column_row() {
        let mut report = Report::new("Test", ReportType::Custom);
        report.add_column("Name");
        report.add_column("Status");

        let row = ReportRow::new()
            .add_field("Name", "Task 1")
            .add_field("Status", "Done");
        report.add_row(row);

        assert_eq!(report.columns.len(), 2);
        assert_eq!(report.row_count(), 1);
    }

    #[test]
    fn test_report_to_csv() {
        let mut report = Report::new("Test", ReportType::Custom);
        report.add_column("Name");
        report.add_column("Status");

        let row = ReportRow::new()
            .add_field("Name", "Task 1")
            .add_field("Status", "Done");
        report.add_row(row);

        let csv = report.to_csv();
        assert!(csv.contains("Name,Status"));
        assert!(csv.contains("Task 1,Done"));
    }

    #[test]
    fn test_report_to_markdown() {
        let mut report = Report::new("Weekly Summary", ReportType::TaskSummary);
        report.add_column("Task");
        report.add_column("Status");
        report.add_summary("Total Tasks", "5");

        let row = ReportRow::new()
            .add_field("Task", "Task 1")
            .add_field("Status", "Done");
        report.add_row(row);

        let md = report.to_markdown();
        assert!(md.contains("# Weekly Summary"));
        assert!(md.contains("## Summary"));
        assert!(md.contains("**Total Tasks**: 5"));
        assert!(md.contains("| Task | Status |"));
        assert!(md.contains("| Task 1 | Done |"));
    }

    #[test]
    fn test_report_template_creation() {
        let template = ReportTemplate::new("Task Report", ReportType::TaskSummary);
        assert_eq!(template.name, "Task Report");
        assert_eq!(template.report_type, ReportType::TaskSummary);
    }

    #[test]
    fn test_report_template_builder() {
        let template = ReportTemplate::new("Test", ReportType::Custom)
            .description("Test description")
            .format(ReportFormat::Csv)
            .add_filter(ReportFilter::equals("status", "done"))
            .add_column("Name")
            .add_column("Status");

        assert_eq!(template.description, "Test description");
        assert_eq!(template.format, ReportFormat::Csv);
        assert_eq!(template.filters.len(), 1);
        assert_eq!(template.columns.len(), 2);
    }

    #[test]
    fn test_report_builder() {
        let report = ReportBuilder::new("Weekly Report")
            .report_type(ReportType::TaskSummary)
            .format(ReportFormat::Markdown)
            .add_filter("status", "completed")
            .add_column("Task")
            .add_column("Completed Date")
            .build();

        assert_eq!(report.title, "Weekly Report");
        assert_eq!(report.report_type, ReportType::TaskSummary);
        assert_eq!(report.format, ReportFormat::Markdown);
        assert_eq!(report.columns.len(), 2);
    }

    #[test]
    fn test_report_builder_from_template() {
        let template = ReportTemplate::new("Task Report", ReportType::TaskSummary)
            .format(ReportFormat::Csv)
            .add_column("Name");

        let report = ReportBuilder::from_template(&template).build();
        assert_eq!(report.title, "Task Report");
        assert_eq!(report.format, ReportFormat::Csv);
        assert_eq!(report.columns.len(), 1);
    }

    #[test]
    fn test_scheduled_report_creation() {
        let schedule = ScheduledReport::new("template-1", ReportFrequency::Weekly);
        assert_eq!(schedule.template_id, "template-1");
        assert_eq!(schedule.frequency, ReportFrequency::Weekly);
        assert!(schedule.enabled);
    }

    #[test]
    fn test_scheduled_report_set_enabled() {
        let mut schedule = ScheduledReport::new("template-1", ReportFrequency::Daily);
        schedule.set_enabled(false);
        assert!(!schedule.enabled);
    }

    #[test]
    fn test_manager_creation() {
        let manager = ReportManager::new();
        assert_eq!(manager.all_reports().len(), 0);
        assert_eq!(manager.all_templates().len(), 0);
    }

    #[test]
    fn test_manager_save_report() {
        let mut manager = ReportManager::new();
        let report = Report::new("Test", ReportType::Custom);
        let id = manager.save_report(report);

        assert_eq!(manager.all_reports().len(), 1);
        assert!(manager.get_report(&id).is_some());
    }

    #[test]
    fn test_manager_delete_report() {
        let mut manager = ReportManager::new();
        let report = Report::new("Test", ReportType::Custom);
        let id = manager.save_report(report);

        assert!(manager.delete_report(&id));
        assert_eq!(manager.all_reports().len(), 0);
    }

    #[test]
    fn test_manager_save_template() {
        let mut manager = ReportManager::new();
        let template = ReportTemplate::new("Test", ReportType::Custom);
        let id = manager.save_template(template);

        assert_eq!(manager.all_templates().len(), 1);
        assert!(manager.get_template(&id).is_some());
    }

    #[test]
    fn test_manager_schedule_report() {
        let mut manager = ReportManager::new();
        let template = ReportTemplate::new("Test", ReportType::Custom);
        let template_id = manager.save_template(template);

        let schedule_id = manager.schedule_report(template_id, ReportFrequency::Weekly);

        assert_eq!(manager.all_schedules().len(), 1);
        assert_eq!(manager.enabled_schedules().len(), 1);
    }

    #[test]
    fn test_manager_generate_from_template() {
        let mut manager = ReportManager::new();
        let template = ReportTemplate::new("Test", ReportType::Custom).add_column("Col1");
        let template_id = manager.save_template(template);

        let report_id = manager.generate_from_template(&template_id).unwrap();
        let report = manager.get_report(&report_id).unwrap();

        assert_eq!(report.columns.len(), 1);
    }

    #[test]
    fn test_default_manager() {
        let manager = ReportManager::default();
        assert_eq!(manager.all_reports().len(), 0);
    }
}
