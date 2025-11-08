/// TOAD - Terminal-Oriented Autonomous Developer
/// Milestone 0: Evaluation Framework

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::time::Duration;
use toad::config::{FeatureFlags, ToadConfig};
use toad::evaluation::{EvaluationHarness, task_loader};
use toad::stats::ComparisonResult;
use toad::{App, Tui};
use tracing::{info, Level};
use tracing_subscriber;

#[derive(Parser)]
#[command(name = "toad")]
#[command(about = "Terminal-Oriented Autonomous Developer", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Run evaluation on a dataset
    Eval {
        /// Path to local SWE-bench dataset (JSON/JSONL)
        #[arg(short, long)]
        dataset: Option<PathBuf>,

        /// Auto-download SWE-bench dataset (verified, lite, or full)
        #[arg(long, value_name = "VARIANT")]
        swebench: Option<String>,

        /// Number of test tasks to use
        #[arg(short, long, default_value = "10")]
        count: usize,

        /// Milestone configuration (1, 2, or 3)
        #[arg(short, long)]
        milestone: Option<u8>,

        /// Output directory for results
        #[arg(short, long, default_value = "./results")]
        output: PathBuf,
    },

    /// Compare two configurations (A/B test)
    Compare {
        /// Path to local SWE-bench dataset (JSON/JSONL)
        #[arg(short, long)]
        dataset: Option<PathBuf>,

        /// Auto-download SWE-bench dataset (verified, lite, or full)
        #[arg(long, value_name = "VARIANT")]
        swebench: Option<String>,

        /// Number of test tasks
        #[arg(short = 'n', long, default_value = "20")]
        count: usize,

        /// Baseline milestone (1, 2, or 3)
        #[arg(short = 'a', long, default_value = "1")]
        baseline: u8,

        /// Test milestone (1, 2, or 3)
        #[arg(short = 'b', long, default_value = "2")]
        test: u8,

        /// Output directory for results
        #[arg(short, long, default_value = "./results")]
        output: PathBuf,
    },

    /// Show feature flags for a configuration
    ShowConfig {
        /// Milestone (1, 2, or 3)
        #[arg(short, long)]
        milestone: Option<u8>,
    },

    /// Generate test dataset
    GenerateTestData {
        /// Number of tasks to generate
        #[arg(short, long, default_value = "50")]
        count: usize,

        /// Output path
        #[arg(short, long, default_value = "./test_data.json")]
        output: PathBuf,
    },

    /// Start the interactive TUI
    Tui,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env file if it exists
    let _ = dotenvy::dotenv();
    
    let cli = Cli::parse();

    // Initialize logging
    let level = if cli.verbose { Level::DEBUG } else { Level::INFO };
    tracing_subscriber::fmt()
        .with_max_level(level)
        .init();

    info!("TOAD v{} - Terminal-Oriented Autonomous Developer", toad::VERSION);

    match cli.command {
        Commands::Eval { dataset, swebench, count, milestone, output } => {
            run_eval(dataset, swebench, count, milestone, output).await?;
        }

        Commands::Compare { dataset, swebench, count, baseline, test, output } => {
            run_compare(dataset, swebench, count, baseline, test, output).await?;
        }

        Commands::ShowConfig { milestone } => {
            show_config(milestone);
        }

        Commands::GenerateTestData { count, output } => {
            generate_test_data(count, output)?;
        }

        Commands::Tui => {
            run_tui()?;
        }
    }

    Ok(())
}

async fn run_eval(
    dataset_path: Option<PathBuf>,
    swebench_variant: Option<String>,
    count: usize,
    milestone: Option<u8>,
    output: PathBuf,
) -> Result<()> {
    info!("Running evaluation...");

    // Load tasks with validation
    let tasks = load_tasks_with_validation(dataset_path, swebench_variant, count).await?;

    info!("Loaded {} tasks (requested: {})", tasks.len(), count);
    if tasks.len() < count {
        tracing::warn!("Dataset contains fewer tasks ({}) than requested ({})", tasks.len(), count);
    }

    // Create configuration
    let config = if let Some(m) = milestone {
        info!("Using Milestone {} configuration", m);
        ToadConfig::for_milestone(m)
    } else {
        info!("Using default configuration");
        ToadConfig::default()
    };

    info!("Feature flags: {}", config.features.description());

    // Run evaluation
    let harness = EvaluationHarness::new(tasks, output.clone());
    let results = harness.evaluate(&config).await?;

    // Print and save results
    results.print_summary();
    harness.save_results(&results)?;

    info!("Results saved to: {:?}", output);

    Ok(())
}

async fn run_compare(
    dataset_path: Option<PathBuf>,
    swebench_variant: Option<String>,
    count: usize,
    baseline_ms: u8,
    test_ms: u8,
    output: PathBuf,
) -> Result<()> {
    info!("Running A/B comparison...");

    // Load tasks with validation
    let tasks = load_tasks_with_validation(dataset_path, swebench_variant, count).await?;

    info!("Loaded {} tasks (requested: {})", tasks.len(), count);
    if tasks.len() < count {
        tracing::warn!("Dataset contains fewer tasks ({}) than requested ({})", tasks.len(), count);
    }

    // Create configurations
    let config_a = ToadConfig::for_milestone(baseline_ms);
    let config_b = ToadConfig::for_milestone(test_ms);

    info!("Config A (M{}): {}", baseline_ms, config_a.features.description());
    info!("Config B (M{}): {}", test_ms, config_b.features.description());

    // Run comparison
    let harness = EvaluationHarness::new(tasks, output.clone());

    info!("Running baseline (M{})...", baseline_ms);
    let (results_a, results_b) = harness.compare(&config_a, &config_b).await?;

    // Analyze comparison
    let comparison = ComparisonResult::compare(&results_a, &results_b);

    // Print results
    results_a.print_summary();
    results_b.print_summary();
    comparison.print_summary();

    // Save results
    harness.save_results(&results_a)?;
    harness.save_results(&results_b)?;

    info!("Results saved to: {:?}", output);
    Ok(())
}

/// Load tasks with validation and smart dataset handling
async fn load_tasks_with_validation(
    dataset_path: Option<PathBuf>,
    swebench_variant: Option<String>,
    count: usize,
) -> Result<Vec<toad::evaluation::Task>> {
    use toad::evaluation::{dataset_manager::{DatasetManager, DatasetSource}, task_loader};

    // Validate conflicting options
    if dataset_path.is_some() && swebench_variant.is_some() {
        anyhow::bail!("Cannot specify both --dataset and --swebench. Choose one.");
    }

    // Load tasks based on source
    let tasks = if let Some(variant) = swebench_variant {
        // Auto-download from HuggingFace
        let source = match variant.to_lowercase().as_str() {
            "verified" => DatasetSource::Verified,
            "lite" => DatasetSource::Lite,
            "full" => DatasetSource::Full,
            _ => anyhow::bail!("Invalid SWE-bench variant: '{}'. Use 'verified', 'lite', or 'full'", variant),
        };

        info!("Loading SWE-bench {} dataset", variant);
        let manager = DatasetManager::default();
        manager.load_sample(source, count).await?
    } else if let Some(path) = dataset_path {
        // Load from local file with validation
        if !path.exists() {
            anyhow::bail!("Dataset file not found: {:?}", path);
        }

        info!("Loading tasks from local file: {:?}", path);
        let loader = task_loader::TaskLoader::new(path);
        
        // Try to load and validate
        let all_tasks = loader.load_all()?;
        info!("Dataset contains {} total tasks", all_tasks.len());
        
        if all_tasks.is_empty() {
            anyhow::bail!("Dataset file is empty or invalid");
        }

        // Take the requested count
        let tasks: Vec<_> = all_tasks.into_iter().take(count).collect();
        
        if tasks.len() < count {
            tracing::warn!(
                "Dataset only has {} tasks, but {} were requested",
                tasks.len(),
                count
            );
        }
        
        tasks
    } else {
        // Generate synthetic test tasks
        info!("No dataset specified, generating {} synthetic test tasks", count);
        task_loader::create_test_tasks(count)
    };

    // Final validation
    if tasks.is_empty() {
        anyhow::bail!("No tasks loaded. Check your dataset or count parameter.");
    }

    Ok(tasks)
}

fn run_tui() -> Result<()> {
    // Initialize error handling
    install_tui_panic_hook();

    info!("Starting Toad TUI");

    // Init: Create initial state
    let mut tui = Tui::new().map_err(|e| anyhow::anyhow!("{}", e))?;
    let mut app = App::new();
    let event_handler = toad::event::EventHandler::new(Duration::from_millis(250));

    info!("TUI initialized, entering main loop");

    // Main event loop
    while !app.should_quit() {
        // View: Render the current state
        tui.draw(|frame| {
            toad::ui::render(&mut app, frame);
        }).map_err(|e| anyhow::anyhow!("{}", e))?;

        // Wait for event (blocking)
        let event = event_handler.next().map_err(|e| anyhow::anyhow!("{}", e))?;

        // Update: Process event and update state
        app.update(event).map_err(|e| anyhow::anyhow!("{}", e))?;
    }

    info!("Exiting main loop");
    Ok(())
}

fn install_tui_panic_hook() {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        // Attempt to restore terminal
        let _ = crossterm::terminal::disable_raw_mode();
        let _ = crossterm::execute!(
            std::io::stdout(),
            crossterm::terminal::LeaveAlternateScreen,
            crossterm::event::DisableMouseCapture
        );
        original_hook(panic_info);
    }));
}

fn show_config(milestone: Option<u8>) {
    let flags = if let Some(m) = milestone {
        println!("=== Milestone {} Configuration ===\n", m);
        match m {
            1 => FeatureFlags::milestone_1(),
            2 => FeatureFlags::milestone_2(),
            3 => FeatureFlags::milestone_3(),
            _ => {
                println!("Invalid milestone. Use 1, 2, or 3.");
                return;
            }
        }
    } else {
        println!("=== Default Configuration ===\n");
        FeatureFlags::default()
    };

    println!("Enabled features: {}/13", flags.enabled_count());
    println!();
    println!("Context Strategies:");
    println!("  AST-based context:        {}", flags.context_ast);
    println!("  Vector embeddings:        {}", flags.context_embeddings);
    println!("  Code graph analysis:      {}", flags.context_graph);
    println!("  Re-ranking:               {}", flags.context_reranking);
    println!();
    println!("Routing Strategies:");
    println!("  Semantic router:          {}", flags.routing_semantic);
    println!("  Multi-model ensemble:     {}", flags.routing_multi_model);
    println!("  Speculative execution:    {}", flags.routing_speculative);
    println!();
    println!("Intelligence Features:");
    println!("  Smart test selection:     {}", flags.smart_test_selection);
    println!("  Failure memory:           {}", flags.failure_memory);
    println!("  Opportunistic planning:   {}", flags.opportunistic_planning);
    println!();
    println!("Optimizations:");
    println!("  Prompt caching:           {}", flags.prompt_caching);
    println!("  Semantic caching:         {}", flags.semantic_caching);
    println!("  Tree-sitter validation:   {}", flags.tree_sitter_validation);
    println!();
    println!("Description: {}", flags.description());
}

fn generate_test_data(count: usize, output: PathBuf) -> Result<()> {
    info!("Generating {} test tasks", count);

    let tasks = task_loader::create_test_tasks(count);
    let json = serde_json::to_string_pretty(&tasks)?;
    std::fs::write(&output, json)?;

    info!("Test data saved to: {:?}", output);
    println!("Generated {} tasks", tasks.len());
    Ok(())
}

/// Initialize logging to a file
///
/// Logs are written to `toad.log` in the current directory.
#[allow(dead_code)]
fn init_logging() -> Result<()> {
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};

    // Create log file
    let log_file = std::fs::File::create("toad.log")?;

    // Set up logging to file
    tracing_subscriber::registry()
        .with(
            fmt::layer()
                .with_writer(log_file)
                .with_ansi(false)
                .with_target(true)
                .with_line_number(true),
        )
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("toad=debug,info")),
        )
        .init();

    Ok(())
}
