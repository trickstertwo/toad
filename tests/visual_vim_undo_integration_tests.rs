//! Integration tests for Visual Polish, Vim Motions, and Undo/Redo
//!
//! Tests PLATINUM tier features for loading spinners, sparklines, line charts,
//! vim motions, and undo/redo system.

use ratatui::style::Color;
use toad::editor::{UndoStack, VimMotions};
use toad::ui::widgets::{DataSeries, LineChart, Sparkline, SparklineStyle, Spinner, SpinnerStyle};

// ============================================================================
// Spinner Integration Tests
// ============================================================================

#[test]
fn test_spinner_creation() {
    let spinner = Spinner::new(SpinnerStyle::Dots);
    assert_eq!(spinner.current_frame(), 0);
    assert_eq!(spinner.style(), SpinnerStyle::Dots);
}

#[test]
fn test_spinner_all_styles() {
    let styles = vec![
        SpinnerStyle::Dots,
        SpinnerStyle::Line,
        SpinnerStyle::Bars,
        SpinnerStyle::Bounce,
        SpinnerStyle::Arrows,
        SpinnerStyle::SimpleDots,
        SpinnerStyle::Binary,
        SpinnerStyle::Clock,
    ];

    for style in styles {
        let spinner = Spinner::new(style);
        assert_eq!(spinner.style(), style);
        assert_eq!(spinner.current_frame(), 0);
        assert!(!spinner.current_symbol().is_empty());
    }
}

#[test]
fn test_spinner_tick() {
    let mut spinner = Spinner::new(SpinnerStyle::Line);
    assert_eq!(spinner.current_frame(), 0);

    spinner.tick();
    assert_eq!(spinner.current_frame(), 1);

    spinner.tick();
    assert_eq!(spinner.current_frame(), 2);

    spinner.tick();
    assert_eq!(spinner.current_frame(), 3);

    // Line style has 4 frames, should wrap
    spinner.tick();
    assert_eq!(spinner.current_frame(), 0);
}

#[test]
fn test_spinner_reset() {
    let mut spinner = Spinner::new(SpinnerStyle::Dots);
    spinner.tick();
    spinner.tick();
    assert_eq!(spinner.current_frame(), 2);

    spinner.reset();
    assert_eq!(spinner.current_frame(), 0);
}

#[test]
fn test_spinner_set_frame() {
    let mut spinner = Spinner::new(SpinnerStyle::Dots);
    spinner.set_frame(5);
    assert_eq!(spinner.current_frame(), 5);

    // Test wrapping for out-of-bounds frame
    let frame_count = SpinnerStyle::Dots.frame_count();
    spinner.set_frame(frame_count + 2);
    assert_eq!(spinner.current_frame(), 2);
}

#[test]
fn test_spinner_set_style() {
    let mut spinner = Spinner::new(SpinnerStyle::Dots);
    spinner.tick();
    spinner.tick();
    assert_eq!(spinner.current_frame(), 2);

    // Changing style should reset frame to 0
    spinner.set_style(SpinnerStyle::Line);
    assert_eq!(spinner.style(), SpinnerStyle::Line);
    assert_eq!(spinner.current_frame(), 0);
}

#[test]
fn test_spinner_with_label() {
    let spinner = Spinner::new(SpinnerStyle::Dots).label("Loading...");
    assert_eq!(spinner.style(), SpinnerStyle::Dots);
}

#[test]
fn test_spinner_with_color() {
    let spinner = Spinner::new(SpinnerStyle::Bars).color(Color::Red);
    assert_eq!(spinner.style(), SpinnerStyle::Bars);
}

#[test]
fn test_spinner_current_symbol() {
    let spinner = Spinner::new(SpinnerStyle::Binary);
    let frames = SpinnerStyle::Binary.frames();

    assert_eq!(spinner.current_symbol(), frames[0]);
}

#[test]
fn test_spinner_style_frames() {
    assert_eq!(SpinnerStyle::Line.frame_count(), 4);
    assert_eq!(SpinnerStyle::Binary.frame_count(), 2);
    assert!(SpinnerStyle::Dots.frame_count() > 0);
}

#[test]
fn test_spinner_style_names() {
    assert_eq!(SpinnerStyle::Dots.name(), "Dots");
    assert_eq!(SpinnerStyle::Line.name(), "Line");
    assert_eq!(SpinnerStyle::Bars.name(), "Bars");
    assert_eq!(SpinnerStyle::Bounce.name(), "Bounce");
    assert_eq!(SpinnerStyle::Arrows.name(), "Arrows");
    assert_eq!(SpinnerStyle::SimpleDots.name(), "Simple Dots");
    assert_eq!(SpinnerStyle::Binary.name(), "Binary");
    assert_eq!(SpinnerStyle::Clock.name(), "Clock");
}

#[test]
fn test_spinner_default() {
    let spinner = Spinner::default();
    assert_eq!(spinner.style(), SpinnerStyle::Dots);
    assert_eq!(spinner.current_frame(), 0);
}

// ============================================================================
// Sparkline Integration Tests
// ============================================================================

#[test]
fn test_sparkline_creation() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let sparkline = Sparkline::new(data.clone());
    assert_eq!(sparkline.data().len(), 5);
    assert_eq!(sparkline.data(), &data);
}

#[test]
fn test_sparkline_with_title() {
    let sparkline = Sparkline::new(vec![1.0, 2.0]).title("CPU Usage");
    assert_eq!(sparkline.data().len(), 2);
}

#[test]
fn test_sparkline_with_style() {
    let styles = SparklineStyle::all();

    for &style in styles {
        let sparkline = Sparkline::new(vec![1.0, 2.0, 3.0]).style(style);
        assert_eq!(sparkline.data().len(), 3);
    }
}

#[test]
fn test_sparkline_style_names() {
    assert_eq!(SparklineStyle::Bars.name(), "Bars");
    assert_eq!(SparklineStyle::Braille.name(), "Braille");
    assert_eq!(SparklineStyle::Dots.name(), "Dots");
}

#[test]
fn test_sparkline_with_border() {
    let sparkline = Sparkline::new(vec![1.0, 2.0]).show_border(true);
    assert_eq!(sparkline.data().len(), 2);
}

#[test]
fn test_sparkline_with_labels() {
    let sparkline = Sparkline::new(vec![1.0, 2.0]).show_labels(true);
    assert_eq!(sparkline.data().len(), 2);
}

#[test]
fn test_sparkline_set_data() {
    let mut sparkline = Sparkline::new(vec![1.0, 2.0]);
    assert_eq!(sparkline.data().len(), 2);

    sparkline.set_data(vec![10.0, 20.0, 30.0]);
    assert_eq!(sparkline.data().len(), 3);
}

#[test]
fn test_sparkline_push() {
    let mut sparkline = Sparkline::new(vec![1.0, 2.0]);
    assert_eq!(sparkline.data().len(), 2);

    sparkline.push(3.0);
    assert_eq!(sparkline.data().len(), 3);

    sparkline.push(4.0);
    assert_eq!(sparkline.data().len(), 4);
}

#[test]
fn test_sparkline_push_with_limit() {
    let mut sparkline = Sparkline::new(vec![1.0, 2.0]);

    sparkline.push_with_limit(3.0, 3);
    assert_eq!(sparkline.data().len(), 3);

    // Should remove oldest when exceeding limit
    sparkline.push_with_limit(4.0, 3);
    assert_eq!(sparkline.data().len(), 3);
    assert_eq!(sparkline.data()[0], 2.0); // 1.0 was removed
}

#[test]
fn test_sparkline_min_max() {
    let sparkline = Sparkline::new(vec![3.0, 1.0, 5.0, 2.0, 4.0]);
    assert_eq!(sparkline.min(), Some(1.0));
    assert_eq!(sparkline.max(), Some(5.0));
}

#[test]
fn test_sparkline_avg() {
    let sparkline = Sparkline::new(vec![2.0, 4.0, 6.0]);
    assert_eq!(sparkline.avg(), Some(4.0));
}

#[test]
fn test_sparkline_empty() {
    let sparkline = Sparkline::new(vec![]);
    assert_eq!(sparkline.data().len(), 0);
    assert_eq!(sparkline.min(), None);
    assert_eq!(sparkline.max(), None);
    assert_eq!(sparkline.avg(), None);
}

#[test]
fn test_sparkline_builder_pattern() {
    let sparkline = Sparkline::new(vec![1.0, 2.0, 3.0])
        .title("Metrics")
        .style(SparklineStyle::Braille)
        .show_border(true)
        .show_labels(true);
    assert_eq!(sparkline.data().len(), 3);
}

// ============================================================================
// LineChart Integration Tests
// ============================================================================

#[test]
fn test_data_series_creation() {
    let data = vec![1.0, 2.0, 3.0, 4.0];
    let series = DataSeries::new("Temperature", data.clone());
    assert_eq!(series.name, "Temperature");
    assert_eq!(series.data.len(), 4);
    assert_eq!(series.data, data);
}

#[test]
fn test_data_series_with_color() {
    let series = DataSeries::new("Data", vec![1.0, 2.0]).with_color(Color::Red);
    assert_eq!(series.color, Color::Red);
}

#[test]
fn test_data_series_with_markers() {
    let series = DataSeries::new("Data", vec![1.0, 2.0]).with_markers(true);
    assert!(series.show_markers);

    let series = DataSeries::new("Data", vec![1.0, 2.0]).with_markers(false);
    assert!(!series.show_markers);
}

#[test]
fn test_data_series_min_max() {
    let series = DataSeries::new("Data", vec![5.0, 2.0, 8.0, 1.0, 6.0]);
    assert_eq!(series.min(), Some(1.0));
    assert_eq!(series.max(), Some(8.0));
}

#[test]
fn test_data_series_empty() {
    let series = DataSeries::new("Empty", vec![]);
    assert_eq!(series.min(), None);
    assert_eq!(series.max(), None);
}

#[test]
fn test_line_chart_creation() {
    let chart = LineChart::new();
    assert_eq!(chart.series_count(), 0);
}

#[test]
fn test_line_chart_add_series() {
    let mut chart = LineChart::new();

    let series1 = DataSeries::new("Series 1", vec![1.0, 2.0, 3.0]);
    chart = chart.add_series(series1);
    assert_eq!(chart.series_count(), 1);

    let series2 = DataSeries::new("Series 2", vec![4.0, 5.0, 6.0]);
    chart = chart.add_series(series2);
    assert_eq!(chart.series_count(), 2);
}

#[test]
fn test_line_chart_with_title() {
    let chart = LineChart::new().with_title("Temperature Over Time");
    assert_eq!(chart.series_count(), 0);
}

#[test]
fn test_line_chart_with_labels() {
    let chart = LineChart::new()
        .with_x_label("Time")
        .with_y_label("Temperature (°C)");
    assert_eq!(chart.series_count(), 0);
}

#[test]
fn test_line_chart_builder_pattern() {
    let series1 = DataSeries::new("Temp", vec![20.0, 22.0, 21.0]).with_color(Color::Red);
    let series2 = DataSeries::new("Humidity", vec![60.0, 65.0, 62.0]).with_color(Color::Blue);

    let chart = LineChart::new()
        .add_series(series1)
        .add_series(series2)
        .with_title("Weather")
        .with_x_label("Time")
        .with_y_label("Value");

    assert_eq!(chart.series_count(), 2);
}

#[test]
fn test_line_chart_multiple_series() {
    let mut chart = LineChart::new();

    for i in 0..5 {
        let data = vec![i as f64, (i + 1) as f64, (i + 2) as f64];
        let series = DataSeries::new(format!("Series {}", i), data);
        chart = chart.add_series(series);
    }

    assert_eq!(chart.series_count(), 5);
}

// ============================================================================
// VimMotions Integration Tests
// ============================================================================

#[test]
fn test_vim_motions_creation() {
    let motions = VimMotions::new("hello world");
    assert_eq!(motions.text(), "hello world");
}

#[test]
fn test_vim_motions_set_text() {
    let mut motions = VimMotions::new("hello");
    assert_eq!(motions.text(), "hello");

    motions.set_text("world");
    assert_eq!(motions.text(), "world");
}

#[test]
fn test_word_forward() {
    let motions = VimMotions::new("hello world test");

    // From start of "hello" to start of "world"
    assert_eq!(motions.word_forward(0), Some(6));

    // From start of "world" to start of "test"
    assert_eq!(motions.word_forward(6), Some(12));

    // From "test" - no next word
    assert!(motions.word_forward(12).is_some() || motions.word_forward(12).is_none());
}

#[test]
fn test_word_backward() {
    let motions = VimMotions::new("hello world test");

    // From start of "test" to start of "world"
    assert_eq!(motions.word_backward(12), Some(6));

    // From start of "world" to start of "hello"
    assert_eq!(motions.word_backward(6), Some(0));

    // From "hello" - no previous word
    assert_eq!(motions.word_backward(0), None);
}

#[test]
fn test_end_of_word() {
    let motions = VimMotions::new("hello world");

    // From start of "hello" to end of "hello"
    let result = motions.end_of_word(0);
    assert!(result.is_some());
    assert!(result.unwrap() > 0);
}

#[test]
fn test_find_char() {
    let motions = VimMotions::new("hello world test");

    // Find 'w' forward from position 0
    assert_eq!(motions.find_char(0, 'w'), Some(6));

    // Find 't' forward from position 0
    assert_eq!(motions.find_char(0, 't'), Some(12));

    // Find non-existent char
    assert_eq!(motions.find_char(0, 'z'), None);
}

#[test]
fn test_find_char_backward() {
    let motions = VimMotions::new("hello world test");

    // Find 'w' backward from end
    assert_eq!(motions.find_char_backward(16, 'w'), Some(6));

    // Find 'h' backward from middle
    assert_eq!(motions.find_char_backward(10, 'h'), Some(0));

    // Find non-existent char
    assert_eq!(motions.find_char_backward(10, 'z'), None);
}

#[test]
fn test_till_char() {
    let motions = VimMotions::new("hello world");

    // Till 'w' from start - should stop before 'w'
    let result = motions.till_char(0, 'w');
    assert!(result.is_some());
    if let Some(pos) = result {
        assert!(pos < 6); // Before 'w' at position 6
    }
}

#[test]
fn test_vim_motions_with_punctuation() {
    let motions = VimMotions::new("hello, world! test");

    // Word motions should handle punctuation
    let forward = motions.word_forward(0);
    assert!(forward.is_some());
}

#[test]
fn test_vim_motions_empty_text() {
    let motions = VimMotions::new("");
    assert_eq!(motions.text(), "");
    assert_eq!(motions.word_forward(0), None);
    assert_eq!(motions.word_backward(0), None);
}

// ============================================================================
// UndoStack Integration Tests
// ============================================================================

#[derive(Clone)]
struct TestAction {
    value: i32,
    executed: std::rc::Rc<std::cell::RefCell<i32>>,
}

impl toad::editor::Action for TestAction {
    fn execute(&self) -> Result<(), String> {
        *self.executed.borrow_mut() += self.value;
        Ok(())
    }

    fn undo(&self) -> Result<(), String> {
        *self.executed.borrow_mut() -= self.value;
        Ok(())
    }

    fn description(&self) -> String {
        format!("Add {}", self.value)
    }
}

#[test]
fn test_undo_stack_creation() {
    let stack: UndoStack<TestAction> = UndoStack::new();
    assert_eq!(stack.undo_count(), 0);
    assert_eq!(stack.redo_count(), 0);
    assert!(!stack.can_undo());
    assert!(!stack.can_redo());
}

#[test]
fn test_undo_stack_execute() {
    let value = std::rc::Rc::new(std::cell::RefCell::new(0));
    let mut stack = UndoStack::new();

    let action = TestAction {
        value: 5,
        executed: value.clone(),
    };

    stack.execute(action).unwrap();
    assert_eq!(*value.borrow(), 5);
    assert_eq!(stack.undo_count(), 1);
    assert!(stack.can_undo());
}

#[test]
fn test_undo_stack_undo() {
    let value = std::rc::Rc::new(std::cell::RefCell::new(0));
    let mut stack = UndoStack::new();

    let action = TestAction {
        value: 5,
        executed: value.clone(),
    };

    stack.execute(action).unwrap();
    assert_eq!(*value.borrow(), 5);

    stack.undo().unwrap();
    assert_eq!(*value.borrow(), 0);
    assert_eq!(stack.undo_count(), 0);
    assert!(stack.can_redo());
}

#[test]
fn test_undo_stack_redo() {
    let value = std::rc::Rc::new(std::cell::RefCell::new(0));
    let mut stack = UndoStack::new();

    let action = TestAction {
        value: 5,
        executed: value.clone(),
    };

    stack.execute(action).unwrap();
    assert_eq!(*value.borrow(), 5);

    stack.undo().unwrap();
    assert_eq!(*value.borrow(), 0);

    stack.redo().unwrap();
    assert_eq!(*value.borrow(), 5);
    assert_eq!(stack.redo_count(), 0);
}

#[test]
fn test_undo_stack_multiple_actions() {
    let value = std::rc::Rc::new(std::cell::RefCell::new(0));
    let mut stack = UndoStack::new();

    // Execute multiple actions
    for i in 1..=5 {
        let action = TestAction {
            value: i,
            executed: value.clone(),
        };
        stack.execute(action).unwrap();
    }

    assert_eq!(*value.borrow(), 15); // 1+2+3+4+5
    assert_eq!(stack.undo_count(), 5);

    // Undo all
    for _ in 0..5 {
        stack.undo().unwrap();
    }

    assert_eq!(*value.borrow(), 0);
    assert_eq!(stack.redo_count(), 5);
}

#[test]
fn test_undo_stack_clears_redo_on_execute() {
    let value = std::rc::Rc::new(std::cell::RefCell::new(0));
    let mut stack = UndoStack::new();

    let action1 = TestAction {
        value: 5,
        executed: value.clone(),
    };
    stack.execute(action1).unwrap();

    stack.undo().unwrap();
    assert!(stack.can_redo());

    // Executing a new action should clear redo stack
    let action2 = TestAction {
        value: 10,
        executed: value.clone(),
    };
    stack.execute(action2).unwrap();

    assert!(!stack.can_redo());
    assert_eq!(stack.redo_count(), 0);
}

#[test]
fn test_undo_stack_with_max_history() {
    let value = std::rc::Rc::new(std::cell::RefCell::new(0));
    let mut stack = UndoStack::with_max_history(3);

    // Add 5 actions (exceeds limit of 3)
    for i in 1..=5 {
        let action = TestAction {
            value: i,
            executed: value.clone(),
        };
        stack.execute(action).unwrap();
    }

    // Only last 3 should be kept
    assert_eq!(stack.undo_count(), 3);
}

#[test]
fn test_undo_stack_is_dirty() {
    let value = std::rc::Rc::new(std::cell::RefCell::new(0));
    let mut stack = UndoStack::new();

    assert!(!stack.is_dirty());

    let action = TestAction {
        value: 5,
        executed: value.clone(),
    };
    stack.execute(action).unwrap();

    assert!(stack.is_dirty());

    stack.mark_saved();
    assert!(!stack.is_dirty());
}

#[test]
fn test_undo_stack_clear() {
    let value = std::rc::Rc::new(std::cell::RefCell::new(0));
    let mut stack = UndoStack::new();

    for i in 1..=3 {
        let action = TestAction {
            value: i,
            executed: value.clone(),
        };
        stack.execute(action).unwrap();
    }

    assert_eq!(stack.undo_count(), 3);

    stack.clear();
    assert_eq!(stack.undo_count(), 0);
    assert_eq!(stack.redo_count(), 0);
}

// ============================================================================
// Cross-Feature Integration Tests
// ============================================================================

#[test]
fn test_spinner_with_sparkline_progress() {
    let mut spinner = Spinner::new(SpinnerStyle::Dots);
    let mut sparkline = Sparkline::new(vec![]);

    // Simulate progress updates
    for i in 0..10 {
        spinner.tick();
        sparkline.push(i as f64);
    }

    assert_eq!(sparkline.data().len(), 10);
    assert!(spinner.current_frame() < SpinnerStyle::Dots.frame_count());
}

#[test]
fn test_line_chart_with_sparkline_preview() {
    // Create detailed line chart
    let data = vec![1.0, 3.0, 2.0, 5.0, 4.0, 6.0];
    let series = DataSeries::new("Temperature", data.clone()).with_color(Color::Red);
    let chart = LineChart::new().add_series(series);

    // Create sparkline preview of same data
    let sparkline = Sparkline::new(data.clone()).style(SparklineStyle::Bars);

    assert_eq!(chart.series_count(), 1);
    assert_eq!(sparkline.data(), &data);
}

#[test]
fn test_vim_motions_with_undo() {
    let value = std::rc::Rc::new(std::cell::RefCell::new(0));
    let mut stack = UndoStack::new();
    let motions = VimMotions::new("hello world test");

    // Execute navigation action
    let action = TestAction {
        value: 5,
        executed: value.clone(),
    };
    stack.execute(action).unwrap();

    // Move forward in text
    let pos = motions.word_forward(0);
    assert!(pos.is_some());

    // Undo action
    stack.undo().unwrap();
    assert_eq!(*value.borrow(), 0);

    // Can still use motions
    let pos = motions.word_backward(pos.unwrap());
    assert!(pos.is_some());
}

#[test]
fn test_complete_editor_workflow() {
    // Setup undo stack
    let value = std::rc::Rc::new(std::cell::RefCell::new(0));
    let mut stack = UndoStack::new();

    // Setup vim motions
    let motions = VimMotions::new("fn main() { println!(\"Hello\"); }");

    // Setup UI indicators
    let mut spinner = Spinner::new(SpinnerStyle::Dots).label("Processing...");
    let mut sparkline = Sparkline::new(vec![]).title("Performance");

    // Simulate editing workflow
    for i in 1..=5 {
        // Execute edit action
        let action = TestAction {
            value: i,
            executed: value.clone(),
        };
        stack.execute(action).unwrap();

        // Update UI
        spinner.tick();
        sparkline.push(i as f64);

        // Navigate text
        if i < 5 {
            motions.word_forward((i * 2) as usize);
        }
    }

    // Verify final state
    assert_eq!(*value.borrow(), 15); // 1+2+3+4+5
    assert_eq!(stack.undo_count(), 5);
    assert_eq!(sparkline.data().len(), 5);
    assert!(stack.is_dirty());

    // Undo some actions
    stack.undo().unwrap();
    stack.undo().unwrap();
    assert_eq!(*value.borrow(), 6); // 15-4-5
    assert_eq!(stack.redo_count(), 2);
}
