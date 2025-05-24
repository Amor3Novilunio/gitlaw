# 📦 Gitlaw Modules

This folder contains **SRP-focused modules** that encapsulate key parts of Gitlaw’s logic.

Each module is a **self-contained unit** that handles one feature or responsibility, and is designed to be injected into `main.rs` (or other modules) as needed.

---

## 🧱 Design Philosophy

- ✅ Each module follows the **Single Responsibility Principle (SRP)**
- 🔌 They can be easily imported and composed from `main.rs`
- 📦 Modules group together their **types**, **logic**, and **helpers**
- 🚫 No cross-module imports unless there's a clear dependency

---

## 🧩 Current Modules

| Module      | Description                                                |
|-------------|------------------------------------------------------------|
| `toml/`     | Handles config loading, writing defaults, and type mapping |

---

## 📁 Module Anatomy (Example: `toml/`)

```text
toml/
├── create.rs         # Writes the default config file
├── read.rs           # Top-level TOML parsing logic
├── read_helpers.rs   # Utilities for parsing or validating fields
├── read_tables.rs    # Functions for structured table extraction
├── types.rs          # All related Serde-compatible types
└── mod.rs            # Exposes selected APIs from this module
