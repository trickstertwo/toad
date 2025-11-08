# Plugin & Marketplace Architecture
**For Toad AI Coding Terminal**

---

## 🎯 Vision

Create a **VSCode-style marketplace** for Toad where developers can:
- Install themes, plugins, language packs, AI prompts
- Share and monetize extensions
- Extend core functionality without forking
- Build a thriving ecosystem

---

## 🏗️ Plugin Architecture Options

### **Option 1: WebAssembly (WASM) - RECOMMENDED ⭐**

**Why WASM is best for Rust TUI:**
- ✅ **Sandboxed execution** - Plugins can't crash the host
- ✅ **Cross-platform** - Write once, run anywhere
- ✅ **Performance** - Near-native speed
- ✅ **Multi-language** - Plugins can be written in Rust, Go, C++, AssemblyScript
- ✅ **Security** - Can't access filesystem unless explicitly granted
- ✅ **Versioning** - Easy to manage dependencies

**Implementation:**
- Use **`wasmtime`** or **`wasmer`** as the WASM runtime
- Define a **WASI-based plugin API**
- Plugins communicate via function calls and shared memory

**Example Plugin Interface:**
```rust
// Host (Toad) provides these imports to plugins
trait PluginHost {
    fn register_command(&self, name: &str, callback: fn());
    fn get_file_content(&self, path: &str) -> String;
    fn show_notification(&self, message: &str);
    fn get_ui_context(&self) -> UIContext;
}

// Plugin exports these functions
trait Plugin {
    fn init() -> Result<()>;
    fn on_event(event: &Event);
    fn on_command(command: &str, args: &[String]);
    fn render_widget(&self, area: Rect) -> Widget;
}
```

**Rust Crates:**
- `wasmtime` - WASM runtime (by Bytecode Alliance)
- `wit-bindgen` - Generate bindings from WIT (WASM Interface Types)
- `wasm-encoder` / `wasm-decoder` - Low-level WASM tools

---

### **Option 2: Lua Scripts**

**Pros:**
- ✅ Fast to write
- ✅ Lightweight runtime
- ✅ Proven (Neovim, WezTerm use Lua)
- ✅ Easy for users to customize

**Cons:**
- ❌ Less performant than WASM
- ❌ Dynamic typing (harder to catch errors)
- ❌ Limited to Lua ecosystem

**Implementation:**
- Use **`mlua`** crate (Lua 5.4 bindings)
- Expose Toad API via Lua bindings

**Example:**
```lua
-- ~/.config/toad/plugins/my-plugin.lua
local toad = require("toad")

function on_file_open(path)
    toad.notify("Opened: " .. path)
    toad.ai.explain_code(path)
end

toad.register_command("explain", function()
    local content = toad.get_buffer_content()
    toad.ai.ask("Explain this code: " .. content)
end)
```

---

### **Option 3: Native Dynamic Libraries (.so / .dll)**

**Pros:**
- ✅ Maximum performance
- ✅ Full Rust ecosystem access

**Cons:**
- ❌ **Platform-specific** - Need to compile per OS
- ❌ **Unsafe** - Can crash the host
- ❌ **ABI instability** - Rust doesn't have stable ABI
- ❌ **Security risk** - Full system access

**Verdict:** ❌ Not recommended for marketplace (too risky)

---

### **Option 4: LSP-Style External Processes**

**Pros:**
- ✅ Language-agnostic
- ✅ Can't crash the host
- ✅ Easy to debug (separate process)

**Cons:**
- ❌ IPC overhead
- ❌ More complex to implement
- ❌ Startup latency

**Use case:** Good for **language servers**, **AI model backends**, not general plugins

---

## 🎨 What Should Be Pluggable?

### **Tier 1: Essential Extension Points**
1. **Themes** - Color schemes, styles
2. **Keybindings** - Custom key maps
3. **Commands** - New slash commands (e.g., `/format-code`)
4. **AI Prompts** - Reusable prompt templates
5. **Widgets** - Custom UI panels (e.g., Git panel, file tree)

### **Tier 2: Advanced Extensions**
6. **Language Support** - Syntax highlighting, LSP integration
7. **File Type Handlers** - Custom renderers (e.g., Markdown preview)
8. **Git Integrations** - Custom workflows
9. **Fuzzy Finders** - Alternative search implementations
10. **AI Model Providers** - Add support for local LLMs, Ollama, etc.

### **Tier 3: Power User Extensions**
11. **Vim Modes** - Custom modal editing modes
12. **Layout Managers** - Alternative pane layouts
13. **Notification Handlers** - Desktop notifications, sounds
14. **Network Protocols** - Remote editing (SSH, Docker)
15. **Build Tool Integrations** - cargo, npm, make watchers

---

## 🏪 Marketplace Architecture

### **Components**

```
┌─────────────────────────────────────────────┐
│          Toad Marketplace Web               │
│  (marketplace.toad.dev)                     │
│                                             │
│  - Browse plugins                           │
│  - Search & filter                          │
│  - User reviews & ratings                   │
│  - Install buttons                          │
└─────────────────┬───────────────────────────┘
                  │
                  │ HTTPS API
                  │
┌─────────────────▼───────────────────────────┐
│       Marketplace Backend (Rust)            │
│  - Registry API                             │
│  - Package hosting (S3/R2)                  │
│  - Version management                       │
│  - Security scanning                        │
│  - Analytics                                │
└─────────────────┬───────────────────────────┘
                  │
                  │ Downloads WASM/Assets
                  │
┌─────────────────▼───────────────────────────┐
│          Toad CLI (local)                   │
│  $ toad install <plugin-name>               │
│  $ toad list                                │
│  $ toad update                              │
│                                             │
│  ~/.config/toad/plugins/                    │
│    ├── theme-catppuccin.wasm                │
│    ├── ai-templates.wasm                    │
│    └── git-enhanced.wasm                    │
└─────────────────────────────────────────────┘
```

### **Registry Format** (inspired by crates.io)

**Plugin Manifest** (`toad.toml`):
```toml
[package]
name = "theme-catppuccin"
version = "1.2.0"
authors = ["Jane Doe <jane@example.com>"]
description = "Beautiful pastel theme for Toad"
license = "MIT"
repository = "https://github.com/user/toad-catppuccin"

[dependencies]
toad-api = "0.1"

[plugin]
type = "theme"
entry = "theme.wasm"
```

**API Endpoints:**
```
GET  /api/v1/plugins              # List all plugins
GET  /api/v1/plugins/:name        # Get plugin details
GET  /api/v1/plugins/:name/versions  # List versions
POST /api/v1/plugins              # Publish (auth required)
GET  /api/v1/search?q=theme       # Search
GET  /api/v1/download/:name/:ver  # Download .wasm file
```

---

## 🔐 Security Considerations

### **Sandboxing (WASM Plugins)**
- Plugins run in isolated WASM instances
- Can only call **explicitly exported host functions**
- No direct filesystem access (must go through host API)
- Resource limits (CPU time, memory)

### **Permissions System** (like browser permissions)
```toml
[permissions]
filesystem.read = ["/home/user/code"]
filesystem.write = false
network = ["api.openai.com"]
commands = ["git"]
```

User prompted on install:
```
⚠️  Plugin "git-enhanced" requests:
  ✓ Read files in current directory
  ✓ Run "git" commands
  ✗ Network access (denied)

Install? [Y/n]
```

### **Code Signing**
- Plugins signed by authors (GPG/SSH keys)
- Marketplace verifies signatures
- Users can trust/block publishers

### **Automatic Security Scanning**
- Scan WASM for suspicious patterns
- Dependency vulnerability checks
- Community reporting

---

## 📦 Distribution & Versioning

### **Semantic Versioning**
- `1.2.3` = Major.Minor.Patch
- Breaking changes = major bump
- Toad API versioning (e.g., `toad-api = "0.1"`)

### **Auto-Updates**
```bash
$ toad update                 # Update all plugins
$ toad update theme-nord      # Update specific plugin
$ toad pin theme-nord@1.0.0   # Pin to version
```

### **Dependency Resolution**
- Plugins can depend on other plugins
- Use Cargo-style dependency resolver
- Lock file for reproducible installs

---

## 💰 Revenue Model (Optional)

### **Free & Open Core**
- Core Toad is free and open-source
- Marketplace is free for free plugins

### **Paid Plugins** (optional feature)
- Developers can charge for premium plugins
- Toad takes 15% cut (like Apple App Store)
- Payment via Stripe/LemonSqueezy
- Monthly subscriptions or one-time purchase

### **Verified Publishers**
- "Verified" badge for trusted developers
- Annual fee ($99?) for verification
- Helps fund marketplace infrastructure

---

## 🛠️ Implementation Roadmap

### **Phase 1: Plugin System Foundation** (v0.2.0)
- [ ] Design plugin API (WIT interface)
- [ ] Implement WASM runtime (wasmtime)
- [ ] Create plugin loader
- [ ] Basic permission system
- [ ] Example theme plugin

### **Phase 2: Plugin SDK** (v0.3.0)
- [ ] `toad-plugin-sdk` crate
- [ ] Code generation for plugin scaffolding
- [ ] Developer documentation
- [ ] Hot-reload support for dev
- [ ] Plugin testing framework

### **Phase 3: Local Plugin Management** (v0.4.0)
- [ ] `toad install <path>` for local plugins
- [ ] Plugin discovery in `~/.config/toad/plugins`
- [ ] `toad list`, `toad enable`, `toad disable`
- [ ] Configuration UI in TUI

### **Phase 4: Marketplace Backend** (v0.5.0)
- [ ] Registry API (Rust + Axum)
- [ ] Package storage (S3/Cloudflare R2)
- [ ] Authentication (GitHub OAuth)
- [ ] Security scanning pipeline
- [ ] CI/CD for publishing

### **Phase 5: Marketplace Web UI** (v0.6.0)
- [ ] Browse & search UI
- [ ] Plugin detail pages
- [ ] User reviews & ratings
- [ ] Author profiles
- [ ] Analytics dashboard

### **Phase 6: Ecosystem Growth** (v1.0.0+)
- [ ] Featured plugins
- [ ] "Plugin of the Week"
- [ ] Monetization (if needed)
- [ ] API stability guarantees
- [ ] Long-term support (LTS) versions

---

## 📝 Example: Creating a Theme Plugin

### **1. Scaffold Plugin**
```bash
$ cargo new --lib toad-theme-nord
$ cd toad-theme-nord
```

### **2. Add Dependencies** (`Cargo.toml`)
```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
toad-plugin-sdk = "0.1"
serde = { version = "1.0", features = ["derive"] }

[build-dependencies]
wit-bindgen = "0.1"
```

### **3. Implement Plugin** (`src/lib.rs`)
```rust
use toad_plugin_sdk::*;

#[derive(Debug)]
pub struct NordTheme;

impl ThemePlugin for NordTheme {
    fn name(&self) -> &str {
        "Nord"
    }

    fn colors(&self) -> ColorScheme {
        ColorScheme {
            background: Color::Rgb(46, 52, 64),
            foreground: Color::Rgb(216, 222, 233),
            primary: Color::Rgb(136, 192, 208),
            secondary: Color::Rgb(129, 161, 193),
            accent: Color::Rgb(191, 97, 106),
            // ... more colors
        }
    }
}

export_plugin!(NordTheme);
```

### **4. Build to WASM**
```bash
$ cargo build --target wasm32-wasi --release
$ cp target/wasm32-wasi/release/toad_theme_nord.wasm theme-nord.wasm
```

### **5. Test Locally**
```bash
$ toad install ./theme-nord.wasm
$ toad theme set nord
```

### **6. Publish to Marketplace**
```bash
$ toad login
$ toad publish
✅ Published theme-nord@1.0.0
```

---

## 🎯 Key Differentiators from Competitors

| Feature | Toad | Neovim | VSCode | Zed |
|---------|------|--------|--------|-----|
| **Language** | Rust | Vimscript/Lua | TypeScript | Rust |
| **Plugin Sandboxing** | ✅ WASM | ❌ | ✅ WebWorker | ⚠️ (native) |
| **Marketplace** | ✅ Planned | ❌ Manual | ✅ Yes | ❌ |
| **TUI-native** | ✅ | ✅ | ❌ (Electron) | ❌ (GPU) |
| **AI-first** | ✅ | ⚠️ (plugins) | ⚠️ (Copilot) | ✅ |
| **Security Model** | ✅ Permissions | ❌ | ⚠️ | ⚠️ |

**Toad's unique position:**
- Only **Rust TUI** with **sandboxed WASM plugins**
- Only **AI-first terminal** with a **curated marketplace**
- Security by default (unlike Neovim's `:!rm -rf /`)

---

## 🚀 Success Metrics

### **Phase 1-3** (Foundation)
- 5+ core plugins (themes, keybindings)
- Plugin hot-reload works
- Documentation complete

### **Phase 4-5** (Marketplace)
- 50+ published plugins
- 1,000+ downloads
- 10+ active plugin authors

### **Phase 6** (Ecosystem)
- 500+ plugins
- 50,000+ users
- Self-sustaining community

---

## 📚 Recommended Reading

- **WASM Component Model**: https://component-model.bytecodealliance.org/
- **wit-bindgen**: https://github.com/bytecodealliance/wit-bindgen
- **wasmtime**: https://docs.wasmtime.dev/
- **VSCode Extension API**: https://code.visualstudio.com/api (inspiration)
- **Neovim Plugin Architecture**: https://neovim.io/doc/user/lua.html
- **Zed Extension System**: https://zed.dev/docs/extensions

---

## 🎬 Next Steps

1. ✅ Build basic TUI (done!)
2. ⬜ Define plugin API (WIT interface)
3. ⬜ Create proof-of-concept theme plugin
4. ⬜ Implement WASM loader
5. ⬜ Test end-to-end workflow
6. ⬜ Open-source plugin SDK
7. ⬜ Build marketplace MVP

---

**Decision:** Use **WASM for plugins**, **Marketplace for distribution**, **Permissions for security**. This positions Toad as the **most secure, extensible AI coding terminal** in the Rust ecosystem.
