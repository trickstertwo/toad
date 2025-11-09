# Ollama Setup for M4 Cascading Routing

This guide explains how to set up Ollama with local models for **M4 Cascading Routing** - enabling 70% cost reduction by running easy/medium tasks locally.

## Quick Start

```bash
# 1. Install Ollama
curl -fsSL https://ollama.com/install.sh | sh

# 2. Pull required models
ollama pull qwen2.5-coder:7b   # For easy tasks (~4 GB)
ollama pull qwen2.5-coder:32b  # For medium tasks (~19 GB)

# 3. Verify Ollama is running
ollama list

# 4. Run TOAD with M4 cascading routing
cargo run --release -- eval --count 10 --milestone 4
```

## What is Ollama?

[Ollama](https://ollama.com) is a local LLM runtime that lets you run open-source language models on your machine. It provides:

- **Local execution**: Models run on your machine (no API costs)
- **Fast inference**: Optimized for CPU/GPU acceleration
- **Simple API**: Compatible with OpenAI-style endpoints
- **Model management**: Easy download and version control

## System Requirements

### Minimum (7B model only)
- **RAM**: 8 GB
- **Disk**: 5 GB free
- **OS**: Linux, macOS, Windows (WSL2)

### Recommended (7B + 32B models)
- **RAM**: 32 GB
- **Disk**: 25 GB free
- **GPU**: 8+ GB VRAM (optional, for faster inference)
- **OS**: Linux, macOS, Windows (WSL2)

## Installation

### Linux/macOS

```bash
# One-line install
curl -fsSL https://ollama.com/install.sh | sh

# Start Ollama service (runs in background)
ollama serve
```

### Windows

1. Download installer from https://ollama.com/download/windows
2. Run the installer (Ollama runs as a service automatically)
3. Open PowerShell and verify: `ollama list`

### Docker (Alternative)

```bash
# Run Ollama in Docker
docker run -d -v ollama:/root/.ollama -p 11434:11434 --name ollama ollama/ollama

# Pull models
docker exec -it ollama ollama pull qwen2.5-coder:7b
docker exec -it ollama ollama pull qwen2.5-coder:32b
```

## Model Selection

### Recommended Models for TOAD M4

| Model | Size | Use Case | Expected Performance |
|-------|------|----------|---------------------|
| `qwen2.5-coder:7b` | 4.7 GB | Easy tasks (< 50 lines) | HumanEval: ~60% |
| `qwen2.5-coder:32b` | 19 GB | Medium tasks (< 200 lines) | HumanEval: ~75% |

**Evidence**: DavaJ research showed qwen3-coder:30b achieved 75.5% baseline, 84.7% with ARCS feedback.

### Download Models

```bash
# Essential for M4
ollama pull qwen2.5-coder:7b
ollama pull qwen2.5-coder:32b

# Optional alternatives
ollama pull deepseek-coder-v2:16b  # Alternative 16B model
ollama pull codellama:34b          # Meta's CodeLlama (older)
```

### List Downloaded Models

```bash
ollama list
```

Example output:
```
NAME                    ID              SIZE    MODIFIED
qwen2.5-coder:7b        abc123          4.7 GB  2 hours ago
qwen2.5-coder:32b       def456          19 GB   1 hour ago
```

## Configuration

### Default Configuration (M4)

TOAD M4 uses these defaults:
- **Easy tasks** → `qwen2.5-coder:7b` (local, free)
- **Medium tasks** → `qwen2.5-coder:32b` (local, free)
- **Hard tasks** → `claude-sonnet-4` (cloud, $$)

### Verify Ollama is Running

```bash
# Check Ollama service status
curl http://localhost:11434/api/version

# Should return: {"version":"0.x.x"}
```

### Test a Model

```bash
# Quick test
ollama run qwen2.5-coder:7b "Write a Python function to reverse a string"
```

## Running TOAD with M4

### With Ollama Only (No API Key)

```bash
# Local-only mode (no cloud fallback)
cargo run --release -- eval --count 10 --milestone 4

# Hard tasks will use qwen2.5-coder:32b instead of cloud
```

### With Ollama + Cloud Fallback (Recommended)

```bash
# Use local for easy/medium, cloud for hard
export ANTHROPIC_API_KEY="sk-ant-..."
cargo run --release -- eval --count 10 --milestone 4
```

### Monitor Routing Decisions

```bash
# Enable verbose logging to see routing
RUST_LOG=info cargo run --release -- eval --count 10 --milestone 4
```

Example log output:
```
[INFO] Cascading router: Task test-001 classified as Easy, routing to Local7B (est. cost: $0.00)
[INFO] Cascading router: Task test-042 classified as Hard, routing to CloudPremium (est. cost: $2.00)
```

## Cost Comparison

### Scenario: 100 SWE-bench Tasks

**Cloud-only (M1/M2/M3):**
```
100 tasks × $2.00/task = $200.00
```

**M4 Cascading (with Ollama):**
```
40 easy tasks × $0 (local 7B)   = $0
40 medium tasks × $0 (local 32B) = $0
20 hard tasks × $2 (cloud)       = $40
-------------------------------------------
Total: $40 (80% savings)
```

**Evidence**: DavaJ achieved 70% cost reduction with this approach.

## Troubleshooting

### Ollama Not Found

```bash
# Linux/macOS: Add to PATH
echo 'export PATH="/usr/local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# Or install manually
curl -L https://ollama.com/download/ollama-linux-amd64 -o /usr/local/bin/ollama
chmod +x /usr/local/bin/ollama
```

### Port Already in Use

```bash
# Check what's using port 11434
lsof -i :11434

# Kill the process or change Ollama port
OLLAMA_HOST=0.0.0.0:11435 ollama serve
```

Then update TOAD config to use custom port:
```rust
// In cascade.rs, change base_url
base_url: Some("http://localhost:11435".to_string()),
```

### Model Download Fails

```bash
# Clear Ollama cache and retry
rm -rf ~/.ollama/models
ollama pull qwen2.5-coder:7b
```

### Out of Memory

If you run out of RAM:

1. **Close other applications**
2. **Use smaller model**:
   ```bash
   ollama pull qwen2.5-coder:7b  # Use 7B for all tasks
   ```
3. **Enable swap** (Linux):
   ```bash
   sudo fallocate -l 16G /swapfile
   sudo chmod 600 /swapfile
   sudo mkswap /swapfile
   sudo swapon /swapfile
   ```

### Slow Inference

If models are slow (>30s per task):

1. **Check CPU usage**: `top` or `htop`
2. **Use GPU acceleration** (if available):
   ```bash
   # NVIDIA GPU
   ollama run qwen2.5-coder:7b --gpu 0
   ```
3. **Use smaller model for all tasks**:
   ```rust
   // In cascade.rs, change Medium tier
   Difficulty::Medium => ModelTier::Local7B,  // Instead of Local32B
   ```

## Performance Expectations

### HumanEval Benchmarks

| Model | Pass@1 | Latency | Notes |
|-------|--------|---------|-------|
| qwen2.5-coder:7b | ~60% | 5-10s | Fast, good for simple tasks |
| qwen2.5-coder:32b | ~75% | 15-25s | Slower, better accuracy |
| claude-sonnet-4 | ~75% | 3-5s | Cloud, consistent quality |

### SWE-bench (Expected)

**Note**: HumanEval ≠ SWE-bench. Real PRs are much harder.

| Configuration | Expected Accuracy | Cost per 100 Tasks |
|--------------|-------------------|-------------------|
| M1 (cloud-only) | 55-60% | $200 |
| M4 (cascading) | 55-65% | $40-60 |

## Next Steps

1. **Verify setup**: `ollama list` shows both models
2. **Run quick test**: `cargo run -- eval --count 5 --milestone 4`
3. **Check logs**: Verify routing decisions (Local7B/Local32B/Cloud)
4. **Run quality gate**: `cargo run -- eval --count 50 --milestone 4`
5. **Compare costs**: M1 vs M4 (should see 60-80% reduction)

## Additional Resources

- **Ollama Docs**: https://github.com/ollama/ollama
- **Model Library**: https://ollama.com/library
- **DavaJ Research**: See project root for proof-of-concept report
- **TOAD M4 Docs**: See `TODO_AI.md` for M4 implementation details

## FAQ

**Q: Do I need both 7B and 32B models?**
A: No, you can use just 7B for everything. Change `cascade.rs` to route Medium → Local7B.

**Q: Can I use different models?**
A: Yes! Edit `src/ai/routing/cascade.rs` and change `model_name()` to use `deepseek-coder-v2:16b` or others.

**Q: Does TOAD require Ollama?**
A: No, Ollama is optional. M1/M2/M3 work fine with cloud-only (Anthropic API).

**Q: What if I don't have 32 GB RAM?**
A: Use only the 7B model, or rent a cloud VM with more RAM.

**Q: Can I run on AWS/GCP instead of local?**
A: Yes, Ollama works on cloud VMs. Just ensure port 11434 is accessible.

---

**Pro Tip**: Start with `qwen2.5-coder:7b` only. Test M4 on 10 tasks. If accuracy is good, then download 32B.
