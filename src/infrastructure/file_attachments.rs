//! File Attachments module for card attachments with version history
//!
//! This module provides comprehensive file attachment management for cards,
//! inspired by Trello and Notion. It supports uploads, previews, cloud links,
//! version history, and file search.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Attachment type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AttachmentType {
    /// Direct file upload
    Upload,
    /// Cloud link (Google Docs, Notion, etc.)
    CloudLink,
    /// GitHub PR/issue link
    GitHub,
    /// External URL
    Link,
}

impl AttachmentType {
    /// Returns the type name
    pub fn name(&self) -> &'static str {
        match self {
            AttachmentType::Upload => "Upload",
            AttachmentType::CloudLink => "Cloud Link",
            AttachmentType::GitHub => "GitHub",
            AttachmentType::Link => "Link",
        }
    }
}

/// File attachment version
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachmentVersion {
    /// Version number (1-based)
    pub version: usize,
    /// File path or URL for this version
    pub path: String,
    /// File size in bytes (for uploads)
    pub size_bytes: Option<u64>,
    /// Uploaded/updated by user ID
    pub updated_by: String,
    /// Uploaded/updated timestamp
    pub updated_at: DateTime<Utc>,
    /// Version comment/note
    pub comment: Option<String>,
}

impl AttachmentVersion {
    /// Creates a new attachment version
    pub fn new(version: usize, path: String, updated_by: String) -> Self {
        Self {
            version,
            path,
            size_bytes: None,
            updated_by,
            updated_at: Utc::now(),
            comment: None,
        }
    }

    /// Gets a human-readable file size
    pub fn human_readable_size(&self) -> String {
        if let Some(bytes) = self.size_bytes {
            if bytes < 1024 {
                format!("{} B", bytes)
            } else if bytes < 1024 * 1024 {
                format!("{:.2} KB", bytes as f64 / 1024.0)
            } else if bytes < 1024 * 1024 * 1024 {
                format!("{:.2} MB", bytes as f64 / (1024.0 * 1024.0))
            } else {
                format!("{:.2} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
            }
        } else {
            "Unknown".to_string()
        }
    }
}

/// File attachment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    /// Attachment ID
    pub id: String,
    /// Card ID this attachment belongs to
    pub card_id: String,
    /// Attachment type
    pub attachment_type: AttachmentType,
    /// File name
    pub name: String,
    /// File extension (e.g., "pdf", "png")
    pub extension: Option<String>,
    /// MIME type (e.g., "image/png", "application/pdf")
    pub mime_type: Option<String>,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Created by user ID
    pub created_by: String,
    /// Version history (ordered, most recent last)
    pub versions: Vec<AttachmentVersion>,
    /// Whether this attachment is deleted
    pub deleted: bool,
}

impl Attachment {
    /// Creates a new attachment
    pub fn new(
        id: String,
        card_id: String,
        attachment_type: AttachmentType,
        name: String,
        path: String,
        created_by: String,
    ) -> Self {
        let extension = Self::extract_extension(&name);
        let mime_type = Self::guess_mime_type(&extension);

        let version = AttachmentVersion::new(1, path, created_by.clone());

        Self {
            id,
            card_id,
            attachment_type,
            name,
            extension,
            mime_type,
            created_at: Utc::now(),
            created_by,
            versions: vec![version],
            deleted: false,
        }
    }

    /// Extracts file extension from filename
    fn extract_extension(name: &str) -> Option<String> {
        name.rsplit('.').next().map(|s| s.to_lowercase())
    }

    /// Guesses MIME type from extension
    fn guess_mime_type(extension: &Option<String>) -> Option<String> {
        extension.as_ref().and_then(|ext| {
            match ext.as_str() {
                "jpg" | "jpeg" => Some("image/jpeg"),
                "png" => Some("image/png"),
                "gif" => Some("image/gif"),
                "svg" => Some("image/svg+xml"),
                "pdf" => Some("application/pdf"),
                "txt" => Some("text/plain"),
                "md" => Some("text/markdown"),
                "json" => Some("application/json"),
                "xml" => Some("application/xml"),
                "zip" => Some("application/zip"),
                "doc" | "docx" => Some("application/vnd.openxmlformats-officedocument.wordprocessingml.document"),
                "xls" | "xlsx" => Some("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"),
                "ppt" | "pptx" => Some("application/vnd.openxmlformats-officedocument.presentationml.presentation"),
                _ => None,
            }
            .map(String::from)
        })
    }

    /// Checks if the file is an image
    pub fn is_image(&self) -> bool {
        self.mime_type
            .as_ref()
            .map(|m| m.starts_with("image/"))
            .unwrap_or(false)
    }

    /// Checks if the file is a PDF
    pub fn is_pdf(&self) -> bool {
        self.mime_type
            .as_ref()
            .map(|m| m == "application/pdf")
            .unwrap_or(false)
    }

    /// Checks if the file is code
    pub fn is_code(&self) -> bool {
        matches!(
            self.extension.as_deref(),
            Some("rs") | Some("js") | Some("ts") | Some("py") | Some("go") | Some("java") | Some("c") | Some("cpp") | Some("h")
        )
    }

    /// Gets the current (latest) version
    pub fn current_version(&self) -> Option<&AttachmentVersion> {
        self.versions.last()
    }

    /// Adds a new version
    pub fn add_version(&mut self, path: String, updated_by: String, comment: Option<String>) {
        let version_number = self.versions.len() + 1;
        let mut version = AttachmentVersion::new(version_number, path, updated_by);
        version.comment = comment;
        self.versions.push(version);
    }

    /// Gets the total number of versions
    pub fn version_count(&self) -> usize {
        self.versions.len()
    }

    /// Marks the attachment as deleted
    pub fn delete(&mut self) {
        self.deleted = true;
    }
}

/// Attachment manager
#[derive(Debug)]
pub struct AttachmentManager {
    attachments: HashMap<String, Attachment>,
    card_attachments: HashMap<String, Vec<String>>, // card_id -> attachment_ids
    next_attachment_id: usize,
}

impl AttachmentManager {
    /// Creates a new attachment manager
    pub fn new() -> Self {
        Self {
            attachments: HashMap::new(),
            card_attachments: HashMap::new(),
            next_attachment_id: 1,
        }
    }

    /// Creates a new attachment
    pub fn create_attachment(
        &mut self,
        card_id: String,
        attachment_type: AttachmentType,
        name: String,
        path: String,
        created_by: String,
    ) -> String {
        let id = format!("attachment-{}", self.next_attachment_id);
        self.next_attachment_id += 1;

        let attachment = Attachment::new(
            id.clone(),
            card_id.clone(),
            attachment_type,
            name,
            path,
            created_by,
        );

        self.attachments.insert(id.clone(), attachment);

        self.card_attachments
            .entry(card_id)
            .or_default()
            .push(id.clone());

        id
    }

    /// Gets an attachment by ID
    pub fn get_attachment(&self, attachment_id: &str) -> Option<&Attachment> {
        self.attachments.get(attachment_id)
    }

    /// Gets a mutable attachment by ID
    pub fn get_attachment_mut(&mut self, attachment_id: &str) -> Option<&mut Attachment> {
        self.attachments.get_mut(attachment_id)
    }

    /// Deletes an attachment (soft delete)
    pub fn delete_attachment(&mut self, attachment_id: &str) -> bool {
        if let Some(attachment) = self.get_attachment_mut(attachment_id) {
            attachment.delete();
            true
        } else {
            false
        }
    }

    /// Gets all attachments for a card
    pub fn attachments_for_card(&self, card_id: &str) -> Vec<&Attachment> {
        self.card_attachments
            .get(card_id)
            .map(|attachment_ids| {
                attachment_ids
                    .iter()
                    .filter_map(|id| self.attachments.get(id))
                    .filter(|a| !a.deleted)
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Searches attachments by filename
    pub fn search_by_name(&self, query: &str) -> Vec<&Attachment> {
        let query_lower = query.to_lowercase();
        self.attachments
            .values()
            .filter(|a| {
                !a.deleted && a.name.to_lowercase().contains(&query_lower)
            })
            .collect()
    }

    /// Gets attachments by type
    pub fn attachments_by_type(&self, attachment_type: AttachmentType) -> Vec<&Attachment> {
        self.attachments
            .values()
            .filter(|a| !a.deleted && a.attachment_type == attachment_type)
            .collect()
    }

    /// Gets all image attachments
    pub fn image_attachments(&self) -> Vec<&Attachment> {
        self.attachments
            .values()
            .filter(|a| !a.deleted && a.is_image())
            .collect()
    }

    /// Gets total attachment count (excluding deleted)
    pub fn total_attachments(&self) -> usize {
        self.attachments.values().filter(|a| !a.deleted).count()
    }

    /// Gets total size of all attachments in bytes
    pub fn total_size_bytes(&self) -> u64 {
        self.attachments
            .values()
            .filter(|a| !a.deleted)
            .filter_map(|a| a.current_version())
            .filter_map(|v| v.size_bytes)
            .sum()
    }

    /// Updates an attachment's file (adds new version)
    pub fn update_attachment(
        &mut self,
        attachment_id: &str,
        path: String,
        updated_by: String,
        comment: Option<String>,
    ) -> bool {
        if let Some(attachment) = self.get_attachment_mut(attachment_id) {
            attachment.add_version(path, updated_by, comment);
            true
        } else {
            false
        }
    }

    /// Sets the size for the current version of an attachment
    pub fn set_attachment_size(&mut self, attachment_id: &str, size_bytes: u64) -> bool {
        if let Some(attachment) = self.get_attachment_mut(attachment_id)
            && let Some(version) = attachment.versions.last_mut() {
                version.size_bytes = Some(size_bytes);
                return true;
            }
        false
    }
}

impl Default for AttachmentManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attachment_type_name() {
        assert_eq!(AttachmentType::Upload.name(), "Upload");
        assert_eq!(AttachmentType::CloudLink.name(), "Cloud Link");
        assert_eq!(AttachmentType::GitHub.name(), "GitHub");
        assert_eq!(AttachmentType::Link.name(), "Link");
    }

    #[test]
    fn test_create_attachment() {
        let mut manager = AttachmentManager::new();
        let attachment_id = manager.create_attachment(
            "card-1".to_string(),
            AttachmentType::Upload,
            "document.pdf".to_string(),
            "/uploads/document.pdf".to_string(),
            "user-1".to_string(),
        );

        assert_eq!(attachment_id, "attachment-1");
        let attachment = manager.get_attachment(&attachment_id).unwrap();
        assert_eq!(attachment.name, "document.pdf");
        assert_eq!(attachment.extension, Some("pdf".to_string()));
        assert_eq!(attachment.mime_type, Some("application/pdf".to_string()));
    }

    #[test]
    fn test_is_image() {
        let mut manager = AttachmentManager::new();
        let img_id = manager.create_attachment(
            "card-1".to_string(),
            AttachmentType::Upload,
            "photo.png".to_string(),
            "/uploads/photo.png".to_string(),
            "user-1".to_string(),
        );

        let attachment = manager.get_attachment(&img_id).unwrap();
        assert!(attachment.is_image());
        assert!(!attachment.is_pdf());
    }

    #[test]
    fn test_is_pdf() {
        let mut manager = AttachmentManager::new();
        let pdf_id = manager.create_attachment(
            "card-1".to_string(),
            AttachmentType::Upload,
            "document.pdf".to_string(),
            "/uploads/document.pdf".to_string(),
            "user-1".to_string(),
        );

        let attachment = manager.get_attachment(&pdf_id).unwrap();
        assert!(attachment.is_pdf());
        assert!(!attachment.is_image());
    }

    #[test]
    fn test_is_code() {
        let mut manager = AttachmentManager::new();
        let code_id = manager.create_attachment(
            "card-1".to_string(),
            AttachmentType::Upload,
            "main.rs".to_string(),
            "/uploads/main.rs".to_string(),
            "user-1".to_string(),
        );

        let attachment = manager.get_attachment(&code_id).unwrap();
        assert!(attachment.is_code());
    }

    #[test]
    fn test_attachments_for_card() {
        let mut manager = AttachmentManager::new();
        manager.create_attachment(
            "card-1".to_string(),
            AttachmentType::Upload,
            "file1.pdf".to_string(),
            "/uploads/file1.pdf".to_string(),
            "user-1".to_string(),
        );
        manager.create_attachment(
            "card-1".to_string(),
            AttachmentType::Upload,
            "file2.png".to_string(),
            "/uploads/file2.png".to_string(),
            "user-1".to_string(),
        );
        manager.create_attachment(
            "card-2".to_string(),
            AttachmentType::Upload,
            "file3.txt".to_string(),
            "/uploads/file3.txt".to_string(),
            "user-1".to_string(),
        );

        let attachments = manager.attachments_for_card("card-1");
        assert_eq!(attachments.len(), 2);
    }

    #[test]
    fn test_delete_attachment() {
        let mut manager = AttachmentManager::new();
        let attachment_id = manager.create_attachment(
            "card-1".to_string(),
            AttachmentType::Upload,
            "file.pdf".to_string(),
            "/uploads/file.pdf".to_string(),
            "user-1".to_string(),
        );

        assert_eq!(manager.total_attachments(), 1);

        manager.delete_attachment(&attachment_id);

        assert_eq!(manager.total_attachments(), 0);

        let attachment = manager.get_attachment(&attachment_id).unwrap();
        assert!(attachment.deleted);
    }

    #[test]
    fn test_version_history() {
        let mut manager = AttachmentManager::new();
        let attachment_id = manager.create_attachment(
            "card-1".to_string(),
            AttachmentType::Upload,
            "doc.pdf".to_string(),
            "/uploads/v1/doc.pdf".to_string(),
            "user-1".to_string(),
        );

        let attachment = manager.get_attachment(&attachment_id).unwrap();
        assert_eq!(attachment.version_count(), 1);

        manager.update_attachment(
            &attachment_id,
            "/uploads/v2/doc.pdf".to_string(),
            "user-1".to_string(),
            Some("Updated version".to_string()),
        );

        let attachment = manager.get_attachment(&attachment_id).unwrap();
        assert_eq!(attachment.version_count(), 2);

        let current = attachment.current_version().unwrap();
        assert_eq!(current.version, 2);
        assert_eq!(current.comment, Some("Updated version".to_string()));
    }

    #[test]
    fn test_search_by_name() {
        let mut manager = AttachmentManager::new();
        manager.create_attachment(
            "card-1".to_string(),
            AttachmentType::Upload,
            "report.pdf".to_string(),
            "/uploads/report.pdf".to_string(),
            "user-1".to_string(),
        );
        manager.create_attachment(
            "card-2".to_string(),
            AttachmentType::Upload,
            "diagram.png".to_string(),
            "/uploads/diagram.png".to_string(),
            "user-1".to_string(),
        );
        manager.create_attachment(
            "card-3".to_string(),
            AttachmentType::Upload,
            "final_report.pdf".to_string(),
            "/uploads/final_report.pdf".to_string(),
            "user-1".to_string(),
        );

        let results = manager.search_by_name("report");
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_attachments_by_type() {
        let mut manager = AttachmentManager::new();
        manager.create_attachment(
            "card-1".to_string(),
            AttachmentType::Upload,
            "file.pdf".to_string(),
            "/uploads/file.pdf".to_string(),
            "user-1".to_string(),
        );
        manager.create_attachment(
            "card-2".to_string(),
            AttachmentType::CloudLink,
            "Google Doc".to_string(),
            "https://docs.google.com/...".to_string(),
            "user-1".to_string(),
        );
        manager.create_attachment(
            "card-3".to_string(),
            AttachmentType::GitHub,
            "PR #123".to_string(),
            "https://github.com/repo/pull/123".to_string(),
            "user-1".to_string(),
        );

        let uploads = manager.attachments_by_type(AttachmentType::Upload);
        assert_eq!(uploads.len(), 1);

        let cloud_links = manager.attachments_by_type(AttachmentType::CloudLink);
        assert_eq!(cloud_links.len(), 1);
    }

    #[test]
    fn test_image_attachments() {
        let mut manager = AttachmentManager::new();
        manager.create_attachment(
            "card-1".to_string(),
            AttachmentType::Upload,
            "photo1.jpg".to_string(),
            "/uploads/photo1.jpg".to_string(),
            "user-1".to_string(),
        );
        manager.create_attachment(
            "card-2".to_string(),
            AttachmentType::Upload,
            "photo2.png".to_string(),
            "/uploads/photo2.png".to_string(),
            "user-1".to_string(),
        );
        manager.create_attachment(
            "card-3".to_string(),
            AttachmentType::Upload,
            "document.pdf".to_string(),
            "/uploads/document.pdf".to_string(),
            "user-1".to_string(),
        );

        let images = manager.image_attachments();
        assert_eq!(images.len(), 2);
    }

    #[test]
    fn test_set_attachment_size() {
        let mut manager = AttachmentManager::new();
        let attachment_id = manager.create_attachment(
            "card-1".to_string(),
            AttachmentType::Upload,
            "file.pdf".to_string(),
            "/uploads/file.pdf".to_string(),
            "user-1".to_string(),
        );

        manager.set_attachment_size(&attachment_id, 1024 * 1024); // 1 MB

        let attachment = manager.get_attachment(&attachment_id).unwrap();
        let version = attachment.current_version().unwrap();
        assert_eq!(version.size_bytes, Some(1024 * 1024));
    }

    #[test]
    fn test_human_readable_size() {
        let mut version = AttachmentVersion::new(1, "/path".to_string(), "user-1".to_string());

        version.size_bytes = Some(512);
        assert_eq!(version.human_readable_size(), "512 B");

        version.size_bytes = Some(1536); // 1.5 KB
        assert_eq!(version.human_readable_size(), "1.50 KB");

        version.size_bytes = Some(1024 * 1024 * 2); // 2 MB
        assert_eq!(version.human_readable_size(), "2.00 MB");

        version.size_bytes = Some(1024 * 1024 * 1024 * 3); // 3 GB
        assert_eq!(version.human_readable_size(), "3.00 GB");
    }

    #[test]
    fn test_total_size_bytes() {
        let mut manager = AttachmentManager::new();
        let id1 = manager.create_attachment(
            "card-1".to_string(),
            AttachmentType::Upload,
            "file1.pdf".to_string(),
            "/uploads/file1.pdf".to_string(),
            "user-1".to_string(),
        );
        let id2 = manager.create_attachment(
            "card-1".to_string(),
            AttachmentType::Upload,
            "file2.png".to_string(),
            "/uploads/file2.png".to_string(),
            "user-1".to_string(),
        );

        manager.set_attachment_size(&id1, 1024);
        manager.set_attachment_size(&id2, 2048);

        assert_eq!(manager.total_size_bytes(), 3072);
    }

    #[test]
    fn test_extension_extraction() {
        let attachment = Attachment::new(
            "att-1".to_string(),
            "card-1".to_string(),
            AttachmentType::Upload,
            "my.file.pdf".to_string(),
            "/path".to_string(),
            "user-1".to_string(),
        );

        assert_eq!(attachment.extension, Some("pdf".to_string()));
    }

    #[test]
    fn test_no_extension() {
        let attachment = Attachment::new(
            "att-1".to_string(),
            "card-1".to_string(),
            AttachmentType::Link,
            "README".to_string(),
            "https://example.com/README".to_string(),
            "user-1".to_string(),
        );

        assert_eq!(attachment.extension, Some("readme".to_string()));
    }

    #[test]
    fn test_mime_type_guessing() {
        let test_cases = vec![
            ("file.jpg", Some("image/jpeg")),
            ("file.png", Some("image/png")),
            ("file.pdf", Some("application/pdf")),
            ("file.txt", Some("text/plain")),
            ("file.json", Some("application/json")),
            ("file.unknown", None),
        ];

        for (filename, expected_mime) in test_cases {
            let attachment = Attachment::new(
                "att-1".to_string(),
                "card-1".to_string(),
                AttachmentType::Upload,
                filename.to_string(),
                "/path".to_string(),
                "user-1".to_string(),
            );

            assert_eq!(attachment.mime_type.as_deref(), expected_mime);
        }
    }

    #[test]
    fn test_version_comment() {
        let mut manager = AttachmentManager::new();
        let attachment_id = manager.create_attachment(
            "card-1".to_string(),
            AttachmentType::Upload,
            "doc.pdf".to_string(),
            "/v1/doc.pdf".to_string(),
            "user-1".to_string(),
        );

        manager.update_attachment(
            &attachment_id,
            "/v2/doc.pdf".to_string(),
            "user-2".to_string(),
            Some("Fixed typos".to_string()),
        );

        let attachment = manager.get_attachment(&attachment_id).unwrap();
        let version2 = &attachment.versions[1];
        assert_eq!(version2.comment, Some("Fixed typos".to_string()));
        assert_eq!(version2.updated_by, "user-2");
    }
}
