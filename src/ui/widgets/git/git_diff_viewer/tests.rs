use super::*;
use ratatui::style::Color;

#[test]
fn test_diff_line_type_parse() {
    assert_eq!(
        DiffLine::parse_type("diff --git a/file b/file"),
        DiffLineType::FileHeader
    );
    assert_eq!(
        DiffLine::parse_type("index 1234567..89abcdef"),
        DiffLineType::Index
    );
    assert_eq!(
        DiffLine::parse_type("--- a/file.txt"),
        DiffLineType::OldFile
    );
    assert_eq!(
        DiffLine::parse_type("+++ b/file.txt"),
        DiffLineType::NewFile
    );
    assert_eq!(DiffLine::parse_type("@@ -1,4 +1,5 @@"), DiffLineType::Hunk);
    assert_eq!(DiffLine::parse_type("+added line"), DiffLineType::Addition);
    assert_eq!(
        DiffLine::parse_type("-deleted line"),
        DiffLineType::Deletion
    );
    assert_eq!(DiffLine::parse_type(" context line"), DiffLineType::Context);
    assert_eq!(
        DiffLine::parse_type("\\ No newline at end of file"),
        DiffLineType::NoNewline
    );
}

#[test]
fn test_diff_viewer_new() {
    let viewer = GitDiffViewer::new();
    assert_eq!(viewer.line_count(), 0);
    assert!(viewer.show_line_numbers);
    assert!(viewer.syntax_highlighting);
}

#[test]
fn test_diff_viewer_set_diff() {
    let mut viewer = GitDiffViewer::new();
    let diff = "diff --git a/test.txt b/test.txt\n+new line\n-old line";
    viewer.set_diff(diff);

    assert_eq!(viewer.line_count(), 3);
}

#[test]
fn test_diff_viewer_stats() {
    let mut viewer = GitDiffViewer::new();
    let diff = "diff --git a/test.txt b/test.txt\n+line1\n+line2\n-line3";
    viewer.set_diff(diff);

    let (additions, deletions, _context) = viewer.stats();
    assert_eq!(additions, 2);
    assert_eq!(deletions, 1);
}

#[test]
fn test_parse_hunk_header() {
    assert_eq!(
        GitDiffViewer::parse_hunk_header("@@ -1,4 +1,5 @@"),
        Some((1, 1))
    );
    assert_eq!(
        GitDiffViewer::parse_hunk_header("@@ -10,7 +12,9 @@"),
        Some((10, 12))
    );
    assert_eq!(GitDiffViewer::parse_hunk_header("invalid"), None);
}

#[test]
fn test_diff_viewer_clear() {
    let mut viewer = GitDiffViewer::new();
    viewer.set_diff("+line");
    assert_eq!(viewer.line_count(), 1);
    viewer.clear();
    assert_eq!(viewer.line_count(), 0);
}

#[test]
fn test_diff_viewer_builder() {
    let viewer = GitDiffViewer::new()
        .with_title("Custom Diff")
        .with_line_numbers(false)
        .with_syntax_highlighting(false)
        .with_compact(true);

    assert_eq!(viewer.title, "Custom Diff");
    assert!(!viewer.show_line_numbers);
    assert!(!viewer.syntax_highlighting);
    assert!(viewer.compact);
}

#[test]
fn test_diff_line_colors() {
    assert_eq!(DiffLineType::Addition.color(), Color::Green);
    assert_eq!(DiffLineType::Deletion.color(), Color::Red);
    assert_eq!(DiffLineType::Hunk.color(), Color::Cyan);
    assert_eq!(DiffLineType::FileHeader.color(), Color::Yellow);
}

#[test]
fn test_diff_line_bg_colors() {
    assert_eq!(
        DiffLineType::Addition.bg_color(),
        Some(Color::Rgb(0, 64, 0))
    );
    assert_eq!(
        DiffLineType::Deletion.bg_color(),
        Some(Color::Rgb(64, 0, 0))
    );
    assert_eq!(DiffLineType::Context.bg_color(), None);
}

#[test]
fn test_full_diff_parsing() {
    let diff = r#"diff --git a/src/main.rs b/src/main.rs
index 1234567..89abcdef 100644
--- a/src/main.rs
+++ b/src/main.rs
@@ -1,4 +1,5 @@
 fn main() {
+    println!("Hello, world!");
     println!("Goodbye!");
 }
"#;

    let mut viewer = GitDiffViewer::new();
    viewer.set_diff(diff);

    assert!(viewer.line_count() > 0);

    let (additions, deletions, context) = viewer.stats();
    assert_eq!(additions, 1);
    assert_eq!(deletions, 0);
    assert_eq!(context, 3); // "fn main() {", "println!("Goodbye!");", "}"
}

// ============================================================================
// COMPREHENSIVE EDGE CASE TESTS (ADVANCED TIER - Git Integration)
// ============================================================================

// ============ Stress Tests ============

#[test]
fn test_viewer_10000_diff_lines() {
    let mut viewer = GitDiffViewer::new();
    let mut diff = String::from("diff --git a/test.txt b/test.txt\n");
    diff.push_str("@@ -1,10000 +1,10000 @@\n");
    for i in 0..10000 {
        diff.push_str(&format!("+line {}\n", i));
    }
    viewer.set_diff(&diff);
    assert_eq!(viewer.line_count(), 10002); // header + hunk + 10000 lines
}

#[test]
fn test_viewer_very_large_diff_1m_chars() {
    let mut viewer = GitDiffViewer::new();
    let large_line = "A".repeat(100000);
    let diff = format!("+{}", large_line);
    viewer.set_diff(&diff);
    assert_eq!(viewer.line_count(), 1);
    assert_eq!(viewer.lines[0].text.len(), 100001); // + prefix + 100000 chars
}

#[test]
fn test_viewer_rapid_set_diff_1000() {
    let mut viewer = GitDiffViewer::new();
    for i in 0..1000 {
        viewer.set_diff(&format!("+line {}", i));
    }
    assert_eq!(viewer.line_count(), 1);
}

#[test]
fn test_parse_1000_hunk_headers() {
    for i in 1..1000 {
        let header = format!("@@ -{},{} +{},{} @@", i, i * 2, i + 1, i * 2 + 1);
        let result = GitDiffViewer::parse_hunk_header(&header);
        assert_eq!(result, Some((i, i + 1)));
    }
}

// ============ Unicode Edge Cases ============

#[test]
fn test_diff_with_emoji() {
    let mut viewer = GitDiffViewer::new();
    let diff = "+Hello 🚀 World 🐸";
    viewer.set_diff(diff);
    assert!(viewer.lines[0].text.contains('🚀'));
    assert!(viewer.lines[0].text.contains('🐸'));
}

#[test]
fn test_diff_rtl_arabic() {
    let mut viewer = GitDiffViewer::new();
    let diff = "+مرحبا بك في العالم";
    viewer.set_diff(diff);
    assert_eq!(viewer.line_count(), 1);
    assert_eq!(viewer.lines[0].line_type, DiffLineType::Addition);
}

#[test]
fn test_diff_rtl_hebrew() {
    let mut viewer = GitDiffViewer::new();
    let diff = "-שלום עולם";
    viewer.set_diff(diff);
    assert_eq!(viewer.line_count(), 1);
    assert_eq!(viewer.lines[0].line_type, DiffLineType::Deletion);
}

#[test]
fn test_diff_japanese() {
    let mut viewer = GitDiffViewer::new();
    let diff = " こんにちは世界";
    viewer.set_diff(diff);
    assert_eq!(viewer.line_count(), 1);
    assert_eq!(viewer.lines[0].line_type, DiffLineType::Context);
}

#[test]
fn test_diff_mixed_scripts() {
    let mut viewer = GitDiffViewer::new();
    let diff = "+Hello مرحبا שלום こんにちは 🚀";
    viewer.set_diff(diff);
    assert_eq!(viewer.line_count(), 1);
}

#[test]
fn test_diff_combining_characters() {
    let mut viewer = GitDiffViewer::new();
    let diff = "+e\u{0301}"; // é with combining accent
    viewer.set_diff(diff);
    assert_eq!(viewer.line_count(), 1);
}

#[test]
fn test_diff_zero_width_characters() {
    let mut viewer = GitDiffViewer::new();
    let diff = "+Test\u{200B}Zero\u{200B}Width";
    viewer.set_diff(diff);
    assert!(viewer.lines[0].text.contains('\u{200B}'));
}

#[test]
fn test_diff_unicode_filename() {
    let mut viewer = GitDiffViewer::new();
    let diff = "diff --git a/日本語.txt b/日本語.txt\n+content";
    viewer.set_diff(diff);
    assert_eq!(viewer.line_count(), 2);
}

// ============ Extreme Values ============

#[test]
fn test_diff_empty() {
    let mut viewer = GitDiffViewer::new();
    viewer.set_diff("");
    assert_eq!(viewer.line_count(), 0);
    let (additions, deletions, context) = viewer.stats();
    assert_eq!(additions, 0);
    assert_eq!(deletions, 0);
    assert_eq!(context, 0);
}

#[test]
fn test_diff_single_character() {
    let mut viewer = GitDiffViewer::new();
    viewer.set_diff("+");
    assert_eq!(viewer.line_count(), 1);
}

#[test]
fn test_diff_very_long_line() {
    let mut viewer = GitDiffViewer::new();
    let long_line = "A".repeat(100000);
    let diff = format!("+{}", long_line);
    viewer.set_diff(&diff);
    assert_eq!(viewer.lines[0].text.len(), 100001);
}

#[test]
fn test_hunk_header_large_line_numbers() {
    let header = format!(
        "@@ -{},{} +{},{} @@",
        usize::MAX - 1,
        10,
        usize::MAX - 1,
        10
    );
    let result = GitDiffViewer::parse_hunk_header(&header);
    assert_eq!(result, Some((usize::MAX - 1, usize::MAX - 1)));
}

// ============ Diff Parsing Edge Cases ============

#[test]
fn test_parse_malformed_hunk_header() {
    assert_eq!(GitDiffViewer::parse_hunk_header("@@ invalid"), None);
    assert_eq!(GitDiffViewer::parse_hunk_header("not a hunk"), None);
    assert_eq!(GitDiffViewer::parse_hunk_header(""), None);
}

#[test]
fn test_diff_with_no_newline_marker() {
    let mut viewer = GitDiffViewer::new();
    let diff = "+new line\n\\ No newline at end of file";
    viewer.set_diff(diff);
    assert_eq!(viewer.line_count(), 2);
    assert_eq!(viewer.lines[1].line_type, DiffLineType::NoNewline);
}

#[test]
fn test_diff_only_whitespace() {
    let mut viewer = GitDiffViewer::new();
    let diff = "   \n\t\n   ";
    viewer.set_diff(diff);
    assert_eq!(viewer.line_count(), 3);
    // All should be context lines
    for line in &viewer.lines {
        assert_eq!(line.line_type, DiffLineType::Context);
    }
}

#[test]
fn test_diff_empty_lines() {
    let mut viewer = GitDiffViewer::new();
    let diff = "+line1\n\n+line2";
    viewer.set_diff(diff);
    assert_eq!(viewer.line_count(), 3);
}

#[test]
fn test_diff_all_line_types() {
    let mut viewer = GitDiffViewer::new();
    let diff = r#"diff --git a/file.txt b/file.txt
index 1234567..89abcdef
--- a/file.txt
+++ b/file.txt
@@ -1,3 +1,3 @@
 context
+addition
-deletion
\ No newline at end of file"#;
    viewer.set_diff(diff);

    let types: Vec<DiffLineType> = viewer.lines.iter().map(|l| l.line_type).collect();
    assert!(types.contains(&DiffLineType::FileHeader));
    assert!(types.contains(&DiffLineType::Index));
    assert!(types.contains(&DiffLineType::OldFile));
    assert!(types.contains(&DiffLineType::NewFile));
    assert!(types.contains(&DiffLineType::Hunk));
    assert!(types.contains(&DiffLineType::Context));
    assert!(types.contains(&DiffLineType::Addition));
    assert!(types.contains(&DiffLineType::Deletion));
    assert!(types.contains(&DiffLineType::NoNewline));
}

// ============ File Filtering Edge Cases ============

#[test]
fn test_filter_by_nonexistent_file() {
    let mut viewer = GitDiffViewer::new();
    let diff = "diff --git a/file1.txt b/file1.txt\n+content";
    viewer.set_diff_for_file(diff, "nonexistent.txt");
    assert_eq!(viewer.line_count(), 0);
}

#[test]
fn test_filter_multiple_files() {
    let mut viewer = GitDiffViewer::new();
    let diff = r#"diff --git a/file1.txt b/file1.txt
+file1 content
diff --git a/file2.txt b/file2.txt
+file2 content"#;
    viewer.set_diff_for_file(diff, "file2.txt");
    assert_eq!(viewer.line_count(), 2); // header + content
    assert!(viewer.lines[0].text.contains("file2.txt"));
}

#[test]
fn test_filter_unicode_filename() {
    let mut viewer = GitDiffViewer::new();
    let diff = "diff --git a/日本語.txt b/日本語.txt\n+content";
    viewer.set_diff_for_file(diff, "日本語.txt");
    assert_eq!(viewer.line_count(), 2);
}

#[test]
fn test_clear_removes_filter() {
    let mut viewer = GitDiffViewer::new();
    viewer.set_diff_for_file("+content", "test.txt");
    assert!(viewer.current_file.is_some());
    viewer.clear();
    assert!(viewer.current_file.is_none());
}

// ============ Trait Coverage ============

#[test]
fn test_diff_line_type_clone() {
    let line_type = DiffLineType::Addition;
    let cloned = line_type.clone();
    assert_eq!(line_type, cloned);
}

#[test]
fn test_diff_line_type_copy() {
    let line_type = DiffLineType::Deletion;
    let copied = line_type;
    assert_eq!(line_type, copied);
}

#[test]
fn test_diff_line_type_equality() {
    assert_eq!(DiffLineType::Addition, DiffLineType::Addition);
    assert_ne!(DiffLineType::Addition, DiffLineType::Deletion);
}

#[test]
fn test_diff_line_type_debug() {
    let line_type = DiffLineType::Hunk;
    let debug_str = format!("{:?}", line_type);
    assert!(debug_str.contains("Hunk"));
}

#[test]
fn test_diff_line_clone() {
    let line = DiffLine::new("+test", DiffLineType::Addition, None, Some(1));
    let cloned = line.clone();
    assert_eq!(line.text, cloned.text);
    assert_eq!(line.line_type, cloned.line_type);
}

#[test]
fn test_diff_line_debug() {
    let line = DiffLine::new("+test", DiffLineType::Addition, None, Some(1));
    let debug_str = format!("{:?}", line);
    assert!(debug_str.contains("DiffLine"));
}

#[test]
fn test_viewer_clone() {
    let viewer = GitDiffViewer::new().with_title("Test");
    let cloned = viewer.clone();
    assert_eq!(viewer.title, cloned.title);
}

#[test]
fn test_viewer_debug() {
    let viewer = GitDiffViewer::new();
    let debug_str = format!("{:?}", viewer);
    assert!(debug_str.contains("GitDiffViewer"));
}

#[test]
fn test_viewer_default() {
    let viewer = GitDiffViewer::default();
    assert_eq!(viewer.line_count(), 0);
    assert_eq!(viewer.title, "Git Diff");
    assert!(viewer.show_line_numbers);
    assert!(viewer.syntax_highlighting);
}

// ============ Complex Workflows ============

#[test]
fn test_set_clear_set_workflow() {
    let mut viewer = GitDiffViewer::new();
    viewer.set_diff("+line1");
    assert_eq!(viewer.line_count(), 1);
    viewer.clear();
    assert_eq!(viewer.line_count(), 0);
    viewer.set_diff("+line2");
    assert_eq!(viewer.line_count(), 1);
}

#[test]
fn test_toggle_line_numbers_with_content() {
    let viewer = GitDiffViewer::new()
        .with_line_numbers(true)
        .with_line_numbers(false)
        .with_line_numbers(true);
    assert!(viewer.show_line_numbers);
}

#[test]
fn test_toggle_syntax_highlighting() {
    let viewer = GitDiffViewer::new()
        .with_syntax_highlighting(false)
        .with_syntax_highlighting(true);
    assert!(viewer.syntax_highlighting);
}

#[test]
fn test_filter_by_file_multiple_times() {
    let mut viewer = GitDiffViewer::new();
    let diff = r#"diff --git a/file1.txt b/file1.txt
+file1
diff --git a/file2.txt b/file2.txt
+file2"#;

    viewer.set_diff_for_file(diff, "file1.txt");
    let count1 = viewer.line_count();

    viewer.set_diff_for_file(diff, "file2.txt");
    let count2 = viewer.line_count();

    assert_eq!(count1, count2); // Both should have same structure
}

// ============ Stats Edge Cases ============

#[test]
fn test_stats_no_changes() {
    let mut viewer = GitDiffViewer::new();
    let diff = " context line";
    viewer.set_diff(diff);
    let (additions, deletions, context) = viewer.stats();
    assert_eq!(additions, 0);
    assert_eq!(deletions, 0);
    assert_eq!(context, 1);
}

#[test]
fn test_stats_only_additions() {
    let mut viewer = GitDiffViewer::new();
    let diff = "+line1\n+line2\n+line3";
    viewer.set_diff(diff);
    let (additions, deletions, context) = viewer.stats();
    assert_eq!(additions, 3);
    assert_eq!(deletions, 0);
    assert_eq!(context, 0);
}

#[test]
fn test_stats_only_deletions() {
    let mut viewer = GitDiffViewer::new();
    let diff = "-line1\n-line2\n-line3";
    viewer.set_diff(diff);
    let (additions, deletions, context) = viewer.stats();
    assert_eq!(additions, 0);
    assert_eq!(deletions, 3);
    assert_eq!(context, 0);
}

#[test]
fn test_stats_mixed_content() {
    let mut viewer = GitDiffViewer::new();
    let diff = "+add1\n-del1\n context\n+add2\n-del2";
    viewer.set_diff(diff);
    let (additions, deletions, context) = viewer.stats();
    assert_eq!(additions, 2);
    assert_eq!(deletions, 2);
    assert_eq!(context, 1);
}

// ============ Line Number Tracking ============

#[test]
fn test_line_numbers_additions() {
    let mut viewer = GitDiffViewer::new();
    let diff = "@@ -1,2 +1,3 @@\n context\n+addition\n context2";
    viewer.set_diff(diff);

    // Hunk header
    assert_eq!(viewer.lines[0].old_line_no, None);
    assert_eq!(viewer.lines[0].new_line_no, None);

    // Context line (old: 1, new: 1)
    assert_eq!(viewer.lines[1].old_line_no, Some(1));
    assert_eq!(viewer.lines[1].new_line_no, Some(1));

    // Addition (new: 2)
    assert_eq!(viewer.lines[2].old_line_no, None);
    assert_eq!(viewer.lines[2].new_line_no, Some(2));

    // Context line (old: 2, new: 3)
    assert_eq!(viewer.lines[3].old_line_no, Some(2));
    assert_eq!(viewer.lines[3].new_line_no, Some(3));
}

#[test]
fn test_line_numbers_deletions() {
    let mut viewer = GitDiffViewer::new();
    let diff = "@@ -1,3 +1,2 @@\n context\n-deletion\n context2";
    viewer.set_diff(diff);

    // Context line (old: 1, new: 1)
    assert_eq!(viewer.lines[1].old_line_no, Some(1));
    assert_eq!(viewer.lines[1].new_line_no, Some(1));

    // Deletion (old: 2)
    assert_eq!(viewer.lines[2].old_line_no, Some(2));
    assert_eq!(viewer.lines[2].new_line_no, None);

    // Context line (old: 3, new: 2)
    assert_eq!(viewer.lines[3].old_line_no, Some(3));
    assert_eq!(viewer.lines[3].new_line_no, Some(2));
}

// ============ Comprehensive Stress Test ============

#[test]
fn test_comprehensive_git_diff_viewer_stress() {
    let mut viewer = GitDiffViewer::new()
        .with_title("Comprehensive Test")
        .with_line_numbers(true)
        .with_syntax_highlighting(true)
        .with_compact(false);

    // Phase 1: Parse complex diff with all line types
    let diff = r#"diff --git a/src/main.rs b/src/main.rs
index abc123..def456 100644
--- a/src/main.rs
+++ b/src/main.rs
@@ -1,10 +1,15 @@
 fn main() {
+    // New comment 🚀
     println!("Hello");
-    println!("Old line");
+    println!("مرحبا");
+    println!("こんにちは");
     println!("Context");
 }
\ No newline at end of file
diff --git a/test.txt b/test.txt
+new file content"#;

    viewer.set_diff(diff);
    let initial_count = viewer.line_count();
    assert!(initial_count > 0);

    // Phase 2: Verify stats
    let (additions, deletions, context) = viewer.stats();
    assert!(additions > 0);
    assert!(deletions > 0);
    assert!(context > 0);

    // Phase 3: Toggle features
    viewer = viewer
        .with_line_numbers(false)
        .with_syntax_highlighting(false)
        .with_compact(true);
    assert!(!viewer.show_line_numbers);
    assert!(!viewer.syntax_highlighting);
    assert!(viewer.compact);

    // Phase 4: Filter by file
    viewer.set_diff_for_file(diff, "src/main.rs");
    let filtered_count = viewer.line_count();
    assert!(filtered_count < initial_count); // Should have fewer lines

    // Phase 5: Clear and reset
    viewer.clear();
    assert_eq!(viewer.line_count(), 0);
    assert!(viewer.current_file.is_none());

    // Phase 6: Set new diff with unicode
    let unicode_diff = "+Hello 🚀\n-مرحبا\n こんにちは";
    viewer.set_diff(unicode_diff);
    assert_eq!(viewer.line_count(), 3);

    // Phase 7: Verify all line types parsed correctly
    let types: Vec<DiffLineType> = viewer.lines.iter().map(|l| l.line_type).collect();
    assert_eq!(types[0], DiffLineType::Addition);
    assert_eq!(types[1], DiffLineType::Deletion);
    assert_eq!(types[2], DiffLineType::Context);
}

// ============ Color Configuration Tests ============

#[test]
fn test_all_line_type_colors() {
    assert_eq!(DiffLineType::FileHeader.color(), Color::Yellow);
    assert_eq!(DiffLineType::Index.color(), Color::DarkGray);
    assert_eq!(DiffLineType::OldFile.color(), Color::Red);
    assert_eq!(DiffLineType::NewFile.color(), Color::Green);
    assert_eq!(DiffLineType::Hunk.color(), Color::Cyan);
    assert_eq!(DiffLineType::Addition.color(), Color::Green);
    assert_eq!(DiffLineType::Deletion.color(), Color::Red);
    assert_eq!(DiffLineType::Context.color(), Color::White);
    assert_eq!(DiffLineType::NoNewline.color(), Color::DarkGray);
}

#[test]
fn test_all_line_type_bg_colors() {
    assert_eq!(
        DiffLineType::Addition.bg_color(),
        Some(Color::Rgb(0, 64, 0))
    );
    assert_eq!(
        DiffLineType::Deletion.bg_color(),
        Some(Color::Rgb(64, 0, 0))
    );
    assert_eq!(DiffLineType::FileHeader.bg_color(), None);
    assert_eq!(DiffLineType::Index.bg_color(), None);
    assert_eq!(DiffLineType::OldFile.bg_color(), None);
    assert_eq!(DiffLineType::NewFile.bg_color(), None);
    assert_eq!(DiffLineType::Hunk.bg_color(), None);
    assert_eq!(DiffLineType::Context.bg_color(), None);
    assert_eq!(DiffLineType::NoNewline.bg_color(), None);
}
// ============ ADDITIONAL FUNCTIONAL TESTS FROM INTEGRATION ============

#[test]
fn test_diff_line_new() {
    let line = DiffLine::new("+added", DiffLineType::Addition, None, Some(42));
    assert_eq!(line.text, "+added");
    assert_eq!(line.line_type, DiffLineType::Addition);
    assert_eq!(line.old_line_no, None);
    assert_eq!(line.new_line_no, Some(42));
}

#[test]
fn test_diff_viewer_default() {
    let viewer = GitDiffViewer::default();
    assert_eq!(viewer.line_count(), 0);
    assert!(viewer.show_line_numbers);
    assert!(viewer.syntax_highlighting);
    assert!(!viewer.compact);
}

#[test]
fn test_diff_viewer_empty_diff() {
    let mut viewer = GitDiffViewer::new();
    viewer.set_diff("");
    assert_eq!(viewer.line_count(), 0);

    let (additions, deletions, context) = viewer.stats();
    assert_eq!(additions, 0);
    assert_eq!(deletions, 0);
    assert_eq!(context, 0);
}

#[test]
fn test_diff_viewer_set_diff_for_file() {
    let diff = r#"diff --git a/file1.txt b/file1.txt
+line in file1
diff --git a/file2.txt b/file2.txt
+line in file2
-removed from file2
"#;

    let mut viewer = GitDiffViewer::new();
    viewer.set_diff_for_file(diff, "file2.txt");

    // Should only have lines from file2.txt
    assert_eq!(viewer.line_count(), 3); // header, +line, -line

    let (additions, deletions, _) = viewer.stats();
    assert_eq!(additions, 1);
    assert_eq!(deletions, 1);
}

#[test]
fn test_diff_viewer_set_diff_for_nonexistent_file() {
    let diff = r#"diff --git a/file1.txt b/file1.txt
+line in file1
"#;

    let mut viewer = GitDiffViewer::new();
    viewer.set_diff_for_file(diff, "nonexistent.txt");

    // Should be empty (file not found)
    assert_eq!(viewer.line_count(), 0);
}

#[test]
fn test_diff_viewer_only_additions() {
    let diff = r#"diff --git a/file.txt b/file.txt
+line1
+line2
+line3
"#;

    let mut viewer = GitDiffViewer::new();
    viewer.set_diff(diff);

    let (additions, deletions, context) = viewer.stats();
    assert_eq!(additions, 3);
    assert_eq!(deletions, 0);
    assert_eq!(context, 0);
}

#[test]
fn test_diff_viewer_only_deletions() {
    let diff = r#"diff --git a/file.txt b/file.txt
-line1
-line2
-line3
"#;

    let mut viewer = GitDiffViewer::new();
    viewer.set_diff(diff);

    let (additions, deletions, context) = viewer.stats();
    assert_eq!(additions, 0);
    assert_eq!(deletions, 3);
    assert_eq!(context, 0);
}

#[test]
fn test_diff_viewer_multiple_files() {
    let diff = r#"diff --git a/file1.txt b/file1.txt
+line in file1
diff --git a/file2.txt b/file2.txt
+line in file2
-removed from file2
"#;

    let mut viewer = GitDiffViewer::new();
    viewer.set_diff(diff);

    assert_eq!(viewer.line_count(), 5);

    let (additions, deletions, _) = viewer.stats();
    assert_eq!(additions, 2);
    assert_eq!(deletions, 1);
}

#[test]
fn test_diff_viewer_multiple_hunks() {
    let diff = r#"diff --git a/file.txt b/file.txt
@@ -1,3 +1,4 @@
 line1
+added line
 line2
@@ -10,2 +11,3 @@
 line10
+another addition
 line11
"#;

    let mut viewer = GitDiffViewer::new();
    viewer.set_diff(diff);

    let (additions, deletions, context) = viewer.stats();
    assert_eq!(additions, 2);
    assert_eq!(deletions, 0);
    assert_eq!(context, 4);
}

#[test]
fn test_diff_viewer_very_long_line() {
    let long_line = "+".to_string() + &"x".repeat(10000);
    let diff = format!("diff --git a/file b/file\n{}", long_line);

    let mut viewer = GitDiffViewer::new();
    viewer.set_diff(&diff);

    assert_eq!(viewer.line_count(), 2);
    let (additions, _, _) = viewer.stats();
    assert_eq!(additions, 1);
}

#[test]
fn test_diff_viewer_clear_resets_file_filter() {
    let mut viewer = GitDiffViewer::new();
    viewer.set_diff_for_file("diff --git a/file.txt b/file.txt\n+line", "file.txt");

    assert!(viewer.current_file.is_some());

    viewer.clear();
    assert!(viewer.current_file.is_none());
    assert_eq!(viewer.line_count(), 0);
}

#[test]
fn test_diff_viewer_line_number_tracking() {
    let diff = r#"diff --git a/file.txt b/file.txt
@@ -5,4 +5,5 @@
 context1
 context2
+addition
-deletion
 context3
"#;

    let mut viewer = GitDiffViewer::new();
    viewer.set_diff(diff);

    // Check that line numbers are tracked correctly
    // Hunk starts at line 5 in both old and new
    let lines = &viewer.lines;

    // Find the context lines and check their numbers
    let context_lines: Vec<_> = lines
        .iter()
        .filter(|l| l.line_type == DiffLineType::Context)
        .collect();

    assert_eq!(context_lines.len(), 3);
    assert_eq!(context_lines[0].old_line_no, Some(5));
    assert_eq!(context_lines[0].new_line_no, Some(5));
}

#[test]
fn test_diff_viewer_stats_empty() {
    let viewer = GitDiffViewer::new();
    let (additions, deletions, context) = viewer.stats();
    assert_eq!(additions, 0);
    assert_eq!(deletions, 0);
    assert_eq!(context, 0);
}

#[test]
fn test_diff_line_context_has_both_line_nos() {
    let line = DiffLine::new(" unchanged", DiffLineType::Context, Some(5), Some(7));
    assert_eq!(line.old_line_no, Some(5));
    assert_eq!(line.new_line_no, Some(7));
}

#[test]
fn test_diff_line_deletion_has_old_line_no() {
    let line = DiffLine::new("-removed", DiffLineType::Deletion, Some(10), None);
    assert_eq!(line.old_line_no, Some(10));
    assert_eq!(line.new_line_no, None);
}

#[test]
fn test_diff_line_type_all_variants_color() {
    // Ensure all line types have defined colors
    assert_eq!(DiffLineType::FileHeader.color(), Color::Yellow);
    assert_eq!(DiffLineType::Index.color(), Color::DarkGray);
    assert_eq!(DiffLineType::OldFile.color(), Color::Red);
    assert_eq!(DiffLineType::NewFile.color(), Color::Green);
    assert_eq!(DiffLineType::Hunk.color(), Color::Cyan);
    assert_eq!(DiffLineType::Addition.color(), Color::Green);
    assert_eq!(DiffLineType::Deletion.color(), Color::Red);
    assert_eq!(DiffLineType::Context.color(), Color::White);
    assert_eq!(DiffLineType::NoNewline.color(), Color::DarkGray);
}

#[test]
fn test_diff_line_type_bg_colors_all_variants() {
    // Only additions and deletions have background colors
    assert!(DiffLineType::Addition.bg_color().is_some());
    assert!(DiffLineType::Deletion.bg_color().is_some());
    assert!(DiffLineType::FileHeader.bg_color().is_none());
    assert!(DiffLineType::Index.bg_color().is_none());
    assert!(DiffLineType::OldFile.bg_color().is_none());
    assert!(DiffLineType::NewFile.bg_color().is_none());
    assert!(DiffLineType::Hunk.bg_color().is_none());
    assert!(DiffLineType::Context.bg_color().is_none());
    assert!(DiffLineType::NoNewline.bg_color().is_none());
}

#[test]
fn test_parse_hunk_header_edge_cases() {
    // Single line change
    assert_eq!(
        GitDiffViewer::parse_hunk_header("@@ -1 +1 @@"),
        Some((1, 1))
    );

    // Large line numbers
    assert_eq!(
        GitDiffViewer::parse_hunk_header("@@ -1234,56 +5678,90 @@"),
        Some((1234, 5678))
    );

    // Malformed headers
    assert_eq!(GitDiffViewer::parse_hunk_header("@@"), None);
    assert_eq!(GitDiffViewer::parse_hunk_header("@@ -a,b +c,d @@"), None);
    assert_eq!(GitDiffViewer::parse_hunk_header("not a hunk"), None);
}
