# 🧭 Gitlaw Commands

This folder contains Gitlaw’s **command entrypoints**, each mapped to a specific subcommand exposed through the CLI.

Each command is responsible for **routing user input**, triggering internal logic (from `modules/` or `helpers/`), and shaping the user-facing behavior of Gitlaw.

---

## 🧱 Design Philosophy

- ✅ Each folder under `commands/` represents a **top-level subcommand**
- ✅ Each folder follows the **Single Responsibility Principle (SRP)**
- 🧩 Each command contains its own `mod.rs` with a `run()` entrypoint

---

## 🧩 Current Commands

| Command       | Description                                                       |
|---------------|-------------------------------------------------------------------|
| `commit/`     | Runs the AI-assisted commit validation and changelog flow         |
| `setup/`      | Prepares necessary environment for Gitlaw (TBD)                   |
| `summon/`     | Downloads models/engine based on TOML config                      |
| `motion/`     | Future command for updating or altering model states (TBD)        |
| `passthrough.rs` | Handles unrecognized commands by delegating to native Git CLI |

---

## 📁 Command Anatomy (Example: `summon/`)

```text
summon/
├── engine.rs          # Logic to download the engine
├── model.rs           # Logic to download the model
├── summon_flags.rs    # Parses and validates CLI flags
├── summon_helper.rs   # Helper functions like `help()`
├── types.rs           # Flag structure definition
└── mod.rs             # Handles top-level CLI entry: `summon::run(args)`
