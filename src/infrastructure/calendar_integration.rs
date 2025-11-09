//! Calendar integration for exporting tasks to iCal/Google Calendar
//!
//! Provides iCal (.ics) export functionality for tasks with due dates,
//! priority-based color coding, and Google Calendar compatibility.
//!
//! # Examples
//!
//! ```
//! use toad::infrastructure::calendar_integration::{CalendarEvent, CalendarExporter};
//! use chrono::Utc;
//!
//! let event = CalendarEvent::new("Review PR #123", Utc::now());
//! let exporter = CalendarExporter::new("TOAD Tasks");
//! let ical = exporter.export_events(&[event]);
//! ```

use chrono::{DateTime, Duration, Utc};
use std::fmt::Write as FmtWrite;

/// Priority level for events (affects color coding)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventPriority {
    /// Low priority (P3) - Blue
    Low,
    /// Medium priority (P2) - Yellow
    Medium,
    /// High priority (P1) - Orange
    High,
    /// Critical priority (P0) - Red
    Critical,
}

impl EventPriority {
    /// Get color for calendar display (CSS color name)
    pub fn color(&self) -> &'static str {
        match self {
            EventPriority::Low => "blue",
            EventPriority::Medium => "yellow",
            EventPriority::High => "orange",
            EventPriority::Critical => "red",
        }
    }

    /// Get numeric priority for iCal (1=highest, 9=lowest)
    pub fn ical_priority(&self) -> u8 {
        match self {
            EventPriority::Critical => 1,
            EventPriority::High => 3,
            EventPriority::Medium => 5,
            EventPriority::Low => 7,
        }
    }
}

/// Recurrence pattern for recurring events
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Recurrence {
    /// No recurrence
    None,
    /// Daily recurrence
    Daily,
    /// Weekly recurrence
    Weekly,
    /// Monthly recurrence
    Monthly,
    /// Yearly recurrence
    Yearly,
    /// Custom recurrence rule (RFC 5545 RRULE format)
    Custom(String),
}

impl Recurrence {
    /// Convert to iCal RRULE format
    pub fn to_rrule(&self) -> Option<String> {
        match self {
            Recurrence::None => None,
            Recurrence::Daily => Some("FREQ=DAILY".to_string()),
            Recurrence::Weekly => Some("FREQ=WEEKLY".to_string()),
            Recurrence::Monthly => Some("FREQ=MONTHLY".to_string()),
            Recurrence::Yearly => Some("FREQ=YEARLY".to_string()),
            Recurrence::Custom(rule) => Some(rule.clone()),
        }
    }
}

/// Calendar event for export
#[derive(Debug, Clone)]
pub struct CalendarEvent {
    /// Unique identifier (UID in iCal)
    pub uid: String,
    /// Event summary/title
    pub summary: String,
    /// Event description
    pub description: Option<String>,
    /// Event location
    pub location: Option<String>,
    /// Event start time
    pub start: DateTime<Utc>,
    /// Event end time (defaults to start + 1 hour)
    pub end: Option<DateTime<Utc>>,
    /// Priority level
    pub priority: EventPriority,
    /// Recurrence pattern
    pub recurrence: Recurrence,
    /// Categories/tags
    pub categories: Vec<String>,
    /// Whether event is all-day
    pub all_day: bool,
    /// Event status (TENTATIVE, CONFIRMED, CANCELLED)
    pub status: EventStatus,
    /// Created timestamp
    pub created: DateTime<Utc>,
    /// Last modified timestamp
    pub last_modified: DateTime<Utc>,
}

/// Event status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventStatus {
    /// Tentative/unconfirmed
    Tentative,
    /// Confirmed
    Confirmed,
    /// Cancelled
    Cancelled,
}

impl EventStatus {
    /// Convert to iCal STATUS value
    pub fn to_ical(&self) -> &'static str {
        match self {
            EventStatus::Tentative => "TENTATIVE",
            EventStatus::Confirmed => "CONFIRMED",
            EventStatus::Cancelled => "CANCELLED",
        }
    }
}

impl CalendarEvent {
    /// Create a new calendar event
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::infrastructure::calendar_integration::CalendarEvent;
    /// use chrono::Utc;
    ///
    /// let event = CalendarEvent::new("Team standup", Utc::now());
    /// ```
    pub fn new(summary: impl Into<String>, start: DateTime<Utc>) -> Self {
        let now = Utc::now();
        let summary = summary.into();
        let uid = format!("toad-{}-{}", start.timestamp(), Self::sanitize_uid(&summary));

        Self {
            uid,
            summary,
            description: None,
            location: None,
            start,
            end: None,
            priority: EventPriority::Medium,
            recurrence: Recurrence::None,
            categories: Vec::new(),
            all_day: false,
            status: EventStatus::Confirmed,
            created: now,
            last_modified: now,
        }
    }

    /// Set event description
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set event location
    pub fn location(mut self, location: impl Into<String>) -> Self {
        self.location = Some(location.into());
        self
    }

    /// Set event end time
    pub fn end(mut self, end: DateTime<Utc>) -> Self {
        self.end = Some(end);
        self
    }

    /// Set event duration (calculates end from start + duration)
    pub fn duration(mut self, duration: Duration) -> Self {
        self.end = Some(self.start + duration);
        self
    }

    /// Set priority
    pub fn priority(mut self, priority: EventPriority) -> Self {
        self.priority = priority;
        self
    }

    /// Set recurrence pattern
    pub fn recurrence(mut self, recurrence: Recurrence) -> Self {
        self.recurrence = recurrence;
        self
    }

    /// Add category/tag
    pub fn add_category(mut self, category: impl Into<String>) -> Self {
        self.categories.push(category.into());
        self
    }

    /// Set as all-day event
    pub fn all_day(mut self, all_day: bool) -> Self {
        self.all_day = all_day;
        self
    }

    /// Set event status
    pub fn status(mut self, status: EventStatus) -> Self {
        self.status = status;
        self
    }

    /// Get event end time (default: start + 1 hour)
    pub fn get_end(&self) -> DateTime<Utc> {
        self.end.unwrap_or_else(|| self.start + Duration::hours(1))
    }

    /// Sanitize UID component
    fn sanitize_uid(s: &str) -> String {
        s.chars()
            .filter(|c| c.is_alphanumeric() || *c == '-')
            .take(32)
            .collect()
    }
}

/// Calendar exporter
///
/// Exports events to iCal (.ics) format compatible with Google Calendar,
/// Apple Calendar, Outlook, and other calendar applications.
#[derive(Debug)]
pub struct CalendarExporter {
    /// Calendar name
    calendar_name: String,
    /// Calendar description
    calendar_description: Option<String>,
    /// Product identifier (PRODID in iCal)
    product_id: String,
}

impl CalendarExporter {
    /// Create a new calendar exporter
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::infrastructure::calendar_integration::CalendarExporter;
    ///
    /// let exporter = CalendarExporter::new("My Tasks");
    /// ```
    pub fn new(calendar_name: impl Into<String>) -> Self {
        Self {
            calendar_name: calendar_name.into(),
            calendar_description: None,
            product_id: "-//TOAD//AI Coding Terminal//EN".to_string(),
        }
    }

    /// Set calendar description
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.calendar_description = Some(description.into());
        self
    }

    /// Set product identifier
    pub fn product_id(mut self, product_id: impl Into<String>) -> Self {
        self.product_id = product_id.into();
        self
    }

    /// Export events to iCal format
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::infrastructure::calendar_integration::{CalendarEvent, CalendarExporter};
    /// use chrono::Utc;
    ///
    /// let event = CalendarEvent::new("Review PR", Utc::now());
    /// let exporter = CalendarExporter::new("TOAD Tasks");
    /// let ical = exporter.export_events(&[event]);
    /// ```
    pub fn export_events(&self, events: &[CalendarEvent]) -> String {
        let mut ical = String::new();

        // Calendar header
        writeln!(ical, "BEGIN:VCALENDAR").unwrap();
        writeln!(ical, "VERSION:2.0").unwrap();
        writeln!(ical, "PRODID:{}", self.product_id).unwrap();
        writeln!(ical, "CALSCALE:GREGORIAN").unwrap();
        writeln!(ical, "METHOD:PUBLISH").unwrap();
        writeln!(ical, "X-WR-CALNAME:{}", self.calendar_name).unwrap();

        if let Some(ref desc) = self.calendar_description {
            writeln!(ical, "X-WR-CALDESC:{}", Self::escape_text(desc)).unwrap();
        }

        writeln!(ical, "X-WR-TIMEZONE:UTC").unwrap();

        // Events
        for event in events {
            self.write_event(&mut ical, event);
        }

        // Calendar footer
        writeln!(ical, "END:VCALENDAR").unwrap();

        ical
    }

    /// Write a single event to iCal format
    fn write_event(&self, ical: &mut String, event: &CalendarEvent) {
        writeln!(ical, "BEGIN:VEVENT").unwrap();

        // UID (required)
        writeln!(ical, "UID:{}", event.uid).unwrap();

        // DTSTAMP (required)
        writeln!(ical, "DTSTAMP:{}", Self::format_datetime(&Utc::now())).unwrap();

        // Summary (required)
        writeln!(ical, "SUMMARY:{}", Self::escape_text(&event.summary)).unwrap();

        // Start time
        if event.all_day {
            writeln!(ical, "DTSTART;VALUE=DATE:{}", Self::format_date(&event.start)).unwrap();
        } else {
            writeln!(ical, "DTSTART:{}", Self::format_datetime(&event.start)).unwrap();
        }

        // End time
        let end = event.get_end();
        if event.all_day {
            writeln!(ical, "DTEND;VALUE=DATE:{}", Self::format_date(&end)).unwrap();
        } else {
            writeln!(ical, "DTEND:{}", Self::format_datetime(&end)).unwrap();
        }

        // Description
        if let Some(ref desc) = event.description {
            writeln!(ical, "DESCRIPTION:{}", Self::escape_text(desc)).unwrap();
        }

        // Location
        if let Some(ref loc) = event.location {
            writeln!(ical, "LOCATION:{}", Self::escape_text(loc)).unwrap();
        }

        // Priority
        writeln!(ical, "PRIORITY:{}", event.priority.ical_priority()).unwrap();

        // Status
        writeln!(ical, "STATUS:{}", event.status.to_ical()).unwrap();

        // Categories
        if !event.categories.is_empty() {
            writeln!(ical, "CATEGORIES:{}", event.categories.join(",")).unwrap();
        }

        // Color (Google Calendar extension)
        writeln!(ical, "COLOR:{}", event.priority.color()).unwrap();

        // Recurrence
        if let Some(rrule) = event.recurrence.to_rrule() {
            writeln!(ical, "RRULE:{}", rrule).unwrap();
        }

        // Created and modified timestamps
        writeln!(ical, "CREATED:{}", Self::format_datetime(&event.created)).unwrap();
        writeln!(ical, "LAST-MODIFIED:{}", Self::format_datetime(&event.last_modified)).unwrap();

        writeln!(ical, "END:VEVENT").unwrap();
    }

    /// Format datetime in iCal format (UTC)
    fn format_datetime(dt: &DateTime<Utc>) -> String {
        dt.format("%Y%m%dT%H%M%SZ").to_string()
    }

    /// Format date in iCal format (DATE value)
    fn format_date(dt: &DateTime<Utc>) -> String {
        dt.format("%Y%m%d").to_string()
    }

    /// Escape text for iCal (RFC 5545)
    fn escape_text(text: &str) -> String {
        text.replace('\\', "\\\\")
            .replace(',', "\\,")
            .replace(';', "\\;")
            .replace('\n', "\\n")
    }
}

impl Default for CalendarExporter {
    fn default() -> Self {
        Self::new("TOAD Calendar")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn make_datetime(year: i32, month: u32, day: u32, hour: u32, min: u32) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(year, month, day, hour, min, 0).unwrap()
    }

    #[test]
    fn test_event_priority_color() {
        assert_eq!(EventPriority::Low.color(), "blue");
        assert_eq!(EventPriority::Medium.color(), "yellow");
        assert_eq!(EventPriority::High.color(), "orange");
        assert_eq!(EventPriority::Critical.color(), "red");
    }

    #[test]
    fn test_event_priority_ical() {
        assert_eq!(EventPriority::Critical.ical_priority(), 1);
        assert_eq!(EventPriority::High.ical_priority(), 3);
        assert_eq!(EventPriority::Medium.ical_priority(), 5);
        assert_eq!(EventPriority::Low.ical_priority(), 7);
    }

    #[test]
    fn test_recurrence_to_rrule() {
        assert_eq!(Recurrence::None.to_rrule(), None);
        assert_eq!(Recurrence::Daily.to_rrule(), Some("FREQ=DAILY".to_string()));
        assert_eq!(Recurrence::Weekly.to_rrule(), Some("FREQ=WEEKLY".to_string()));
        assert_eq!(Recurrence::Monthly.to_rrule(), Some("FREQ=MONTHLY".to_string()));
        assert_eq!(Recurrence::Yearly.to_rrule(), Some("FREQ=YEARLY".to_string()));
        assert_eq!(
            Recurrence::Custom("FREQ=DAILY;COUNT=5".to_string()).to_rrule(),
            Some("FREQ=DAILY;COUNT=5".to_string())
        );
    }

    #[test]
    fn test_event_status_to_ical() {
        assert_eq!(EventStatus::Tentative.to_ical(), "TENTATIVE");
        assert_eq!(EventStatus::Confirmed.to_ical(), "CONFIRMED");
        assert_eq!(EventStatus::Cancelled.to_ical(), "CANCELLED");
    }

    #[test]
    fn test_calendar_event_creation() {
        let start = make_datetime(2025, 11, 9, 10, 0);
        let event = CalendarEvent::new("Team Meeting", start);

        assert_eq!(event.summary, "Team Meeting");
        assert_eq!(event.start, start);
        assert_eq!(event.priority, EventPriority::Medium);
        assert_eq!(event.recurrence, Recurrence::None);
        assert_eq!(event.status, EventStatus::Confirmed);
        assert!(!event.all_day);
    }

    #[test]
    fn test_calendar_event_builder() {
        let start = make_datetime(2025, 11, 9, 10, 0);
        let event = CalendarEvent::new("Team Meeting", start)
            .description("Weekly team sync")
            .location("Conference Room A")
            .duration(Duration::hours(2))
            .priority(EventPriority::High)
            .add_category("Work")
            .add_category("Meeting")
            .status(EventStatus::Confirmed);

        assert_eq!(event.description, Some("Weekly team sync".to_string()));
        assert_eq!(event.location, Some("Conference Room A".to_string()));
        assert_eq!(event.get_end(), make_datetime(2025, 11, 9, 12, 0));
        assert_eq!(event.priority, EventPriority::High);
        assert_eq!(event.categories, vec!["Work", "Meeting"]);
        assert_eq!(event.status, EventStatus::Confirmed);
    }

    #[test]
    fn test_calendar_event_all_day() {
        let start = make_datetime(2025, 11, 9, 0, 0);
        let event = CalendarEvent::new("Holiday", start).all_day(true);

        assert!(event.all_day);
    }

    #[test]
    fn test_calendar_event_recurrence() {
        let start = make_datetime(2025, 11, 9, 10, 0);
        let event = CalendarEvent::new("Daily Standup", start)
            .recurrence(Recurrence::Daily);

        assert_eq!(event.recurrence, Recurrence::Daily);
    }

    #[test]
    fn test_calendar_exporter_creation() {
        let exporter = CalendarExporter::new("My Calendar");
        assert_eq!(exporter.calendar_name, "My Calendar");
        assert!(exporter.calendar_description.is_none());
    }

    #[test]
    fn test_calendar_exporter_builder() {
        let exporter = CalendarExporter::new("My Calendar")
            .description("Personal tasks")
            .product_id("-//Custom//Product//EN");

        assert_eq!(exporter.calendar_description, Some("Personal tasks".to_string()));
        assert_eq!(exporter.product_id, "-//Custom//Product//EN");
    }

    #[test]
    fn test_export_single_event() {
        let start = make_datetime(2025, 11, 9, 10, 0);
        let event = CalendarEvent::new("Test Event", start)
            .description("Test description")
            .location("Test location")
            .priority(EventPriority::High);

        let exporter = CalendarExporter::new("Test Calendar");
        let ical = exporter.export_events(&[event]);

        // Check required iCal components
        assert!(ical.contains("BEGIN:VCALENDAR"));
        assert!(ical.contains("VERSION:2.0"));
        assert!(ical.contains("BEGIN:VEVENT"));
        assert!(ical.contains("SUMMARY:Test Event"));
        assert!(ical.contains("DESCRIPTION:Test description"));
        assert!(ical.contains("LOCATION:Test location"));
        assert!(ical.contains("PRIORITY:3")); // High = 3
        assert!(ical.contains("COLOR:orange")); // High = orange
        assert!(ical.contains("DTSTART:20251109T100000Z"));
        assert!(ical.contains("END:VEVENT"));
        assert!(ical.contains("END:VCALENDAR"));
    }

    #[test]
    fn test_export_multiple_events() {
        let event1 = CalendarEvent::new("Event 1", make_datetime(2025, 11, 9, 10, 0));
        let event2 = CalendarEvent::new("Event 2", make_datetime(2025, 11, 10, 14, 0));

        let exporter = CalendarExporter::new("Test Calendar");
        let ical = exporter.export_events(&[event1, event2]);

        assert!(ical.contains("SUMMARY:Event 1"));
        assert!(ical.contains("SUMMARY:Event 2"));
    }

    #[test]
    fn test_export_all_day_event() {
        let start = make_datetime(2025, 11, 9, 0, 0);
        let event = CalendarEvent::new("All Day Event", start).all_day(true);

        let exporter = CalendarExporter::new("Test Calendar");
        let ical = exporter.export_events(&[event]);

        assert!(ical.contains("DTSTART;VALUE=DATE:20251109"));
        assert!(ical.contains("DTEND;VALUE=DATE:"));
    }

    #[test]
    fn test_export_recurring_event() {
        let start = make_datetime(2025, 11, 9, 10, 0);
        let event = CalendarEvent::new("Daily Standup", start)
            .recurrence(Recurrence::Daily);

        let exporter = CalendarExporter::new("Test Calendar");
        let ical = exporter.export_events(&[event]);

        assert!(ical.contains("RRULE:FREQ=DAILY"));
    }

    #[test]
    fn test_export_with_categories() {
        let start = make_datetime(2025, 11, 9, 10, 0);
        let event = CalendarEvent::new("Meeting", start)
            .add_category("Work")
            .add_category("Important");

        let exporter = CalendarExporter::new("Test Calendar");
        let ical = exporter.export_events(&[event]);

        assert!(ical.contains("CATEGORIES:Work,Important"));
    }

    #[test]
    fn test_escape_text() {
        let escaped = CalendarExporter::escape_text("Text with, commas; semicolons\nand newlines\\backslashes");
        assert_eq!(escaped, "Text with\\, commas\\; semicolons\\nand newlines\\\\backslashes");
    }

    #[test]
    fn test_format_datetime() {
        let dt = make_datetime(2025, 11, 9, 14, 30);
        assert_eq!(CalendarExporter::format_datetime(&dt), "20251109T143000Z");
    }

    #[test]
    fn test_format_date() {
        let dt = make_datetime(2025, 11, 9, 14, 30);
        assert_eq!(CalendarExporter::format_date(&dt), "20251109");
    }

    #[test]
    fn test_event_status() {
        let start = make_datetime(2025, 11, 9, 10, 0);
        let event = CalendarEvent::new("Tentative Meeting", start)
            .status(EventStatus::Tentative);

        let exporter = CalendarExporter::new("Test Calendar");
        let ical = exporter.export_events(&[event]);

        assert!(ical.contains("STATUS:TENTATIVE"));
    }

    #[test]
    fn test_default_exporter() {
        let exporter = CalendarExporter::default();
        assert_eq!(exporter.calendar_name, "TOAD Calendar");
    }

    #[test]
    fn test_event_get_end_default() {
        let start = make_datetime(2025, 11, 9, 10, 0);
        let event = CalendarEvent::new("Test", start);

        // Default end is start + 1 hour
        assert_eq!(event.get_end(), make_datetime(2025, 11, 9, 11, 0));
    }

    #[test]
    fn test_event_get_end_custom() {
        let start = make_datetime(2025, 11, 9, 10, 0);
        let end = make_datetime(2025, 11, 9, 12, 30);
        let event = CalendarEvent::new("Test", start).end(end);

        assert_eq!(event.get_end(), end);
    }
}
