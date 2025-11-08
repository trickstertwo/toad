/// TOAD - Terminal-Oriented Autonomous Developer
/// Milestone 0: Evaluation Framework

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use toad::config::{FeatureFlags, ToadConfig};
use toad::evaluation::{EvaluationHarness, task_loader};
use toad::stats::ComparisonResult;
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
        /// Path to SWE-bench dataset (JSON)
        #[arg(short, long)]
        dataset: Option<PathBuf>,

        /// Number of test tasks to generate (if no dataset)
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
        /// Path to SWE-bench dataset (JSON)
        #[arg(short, long)]
        dataset: Option<PathBuf>,

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
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    let level = if cli.verbose { Level::DEBUG } else { Level::INFO };
    tracing_subscriber::fmt()
        .with_max_level(level)
        .init();

    info!("TOAD v{} - Terminal-Oriented Autonomous Developer", toad::VERSION);

    match cli.command {
        Commands::Eval { dataset, count, milestone, output } => {
            run_eval(dataset, count, milestone, output).await?;
        }

        Commands::Compare { dataset, count, baseline, test, output } => {
            run_compare(dataset, count, baseline, test, output).await?;
        }

        Commands::ShowConfig { milestone } => {
            show_config(milestone);
        }

        Commands::GenerateTestData { count, output } => {
            generate_test_data(count, output)?;
        }
    }

    Ok(())
}

async fn run_eval(
    dataset_path: Option<PathBuf>,
    count: usize,
    milestone: Option<u8>,
    output: PathBuf,
) -> Result<()> {
    info!("Running evaluation...");

    // Load tasks
    let tasks = if let Some(path) = dataset_path {
        info!("Loading tasks from: {:?}", path);
        let loader = task_loader::TaskLoader::new(path);
        loader.load_sample(count)?
    } else {
        info!("Generating {} test tasks", count);
        task_loader::create_test_tasks(count)
    };

    info!("Loaded {} tasks", tasks.len());

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
    count: usize,
    baseline_ms: u8,
    test_ms: u8,
    output: PathBuf,
) -> Result<()> {
    info!("Running A/B comparison...");

    // Load tasks
    let tasks = if let Some(path) = dataset_path {
        info!("Loading tasks from: {:?}", path);
        let loader = task_loader::TaskLoader::new(path);
        loader.load_sample(count)?
    } else {
        info!("Generating {} test tasks", count);
        task_loader::create_test_tasks(count)
    };

    info!("Loaded {} tasks", tasks.len());

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
