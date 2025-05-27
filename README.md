# 🧑‍⚖️ Gitlaw

**A modern, opinionated Git CLI wrapper that enforces commit structure, enables AI-assisted messages, and auto-documents your code history — one law at a time.**

Gitlaw combines strong commit discipline with offline AI tooling and changelog automation. It is modular by design: you can plug in any LLM or engine you like — but by default, it uses **`llama.cpp`** and **`phi-2`** for local inference.

---

## ✨ Features

- 🔒 **Commit validation** — conventional commit format, block "WIP", length enforcement
- 🧠 **AI-powered commit suggestions** — generated from staged Git diffs
- 📋 **Interactive commit flow** — approve, regenerate, or rewrite AI suggestions
- 📁 **Auto-documentation** — commit messages with `lawXX` tag produce changelogs
- 🧼 **Transparent Git passthrough** — works like Git for everything else

---

## 🛠️ Installation

### 1. Clone and build Gitlaw

```bash
git clone https://github.com/your-username/gitlaw.git
cd gitlaw
cargo build --release
````

Then add Gitlaw to your `PATH`:

```bash
export PATH="$PWD/target/release:$PATH"
```

### 2. Install the default AI engine (llama.cpp)

```bash
git clone https://github.com/ggerganov/llama.cpp
cd llama.cpp
make
```

Ensure the built binary (e.g., `main`) is added to your path or referenced in `gitlaw.toml`.

### 3. Download a Phi-2 model (Recommended: Q4)

```bash
wget https://huggingface.co/TheBloke/phi-2-GGUF/resolve/main/phi-2.Q4_K_M.gguf
mv phi-2.Q4_K_M.gguf path/to/your/models/
```

---

## ⚙️ Configuration

Gitlaw is fully configurable via a TOML file (`gitlaw.toml`). Example:

```toml
[model]
file_name = "phi-2.Q4_K_M.gguf"
path = "path/to/your/models"
temperature = 0.7

[engine]
file_name = "llama.cpp"
path = "~/.gitlaw/bin/llama"

[download]
engine = "https://github.com/ggml-org/llama.cpp/releases/download/b5476/llama-b5476-bin-ubuntu-x64.zip"
model = "https://huggingface.co/TheBloke/phi-2-GGUF/resolve/main/phi-2.Q4_K_M.gguf?download=true"
```

💡 *A future `gitlaw init` command will automate this setup: download defaults, apply Git config, and scaffold the repo.*

---

## ⚙️ Usage

### 🧑‍⚖️ AI-Backed Commit

```bash
gitlaw commit
```

This will:

1. Analyze staged changes
2. Run the local AI model to suggest a commit message
3. Prompt you to accept, regenerate, or write your own
4. Commit with the selected message
5. If message includes `lawXX`, generate `gitlaw/lawXX.md` changelog

### 🧪 Example

```bash
gitlaw commit
```

Produces:

```bash
feat(law42): fix concurrent config handling
```

And creates:

```
gitlaw/
  └─ law42.md   # includes before/after diffs
```

---

## 🤖 Model Quality Reference

Choose the model quantization based on your speed/quality needs:

| Level  | Quality       | Size               | Speed         | Notes                                       |
| ------ | ------------- | ------------------ | ------------- | ------------------------------------------- |
| **Q2** | 🟥 Low        | ✅ Smallest         | ⚡ Fastest     | Often too lossy for good summaries          |
| **Q3** | 🟧 Medium-low | ✅ Small            | ⚡ Fast        | Okay for basic tasks, not great for commits |
| **Q4** | 🟨 Good       | ✅ Balanced         | ⚡ Fast enough | 👍 Recommended balance of quality and speed |
| **Q5** | 🟩 Very good  | ❗ Larger (\~2.5GB) | 🐢 Slower     | High quality, more RAM usage                |
| **Q6** | 🟩 Best       | ❌ Heavy            | 🐢 Slowest    | Overkill for Gitlaw; nearly full-precision  |

---

## 🚀 Roadmap

* [x] Git passthrough for non-commit commands
* [x] Modular AI model + engine config
* [x] Offline-only AI (default: llama.cpp + phi-2)
* [x] AI-generated commit messages
* [x] AI temperature control
* [ ] `gitlaw init` for setup + scaffolding
* [ ] Arg-based commit message override (`gitlaw commit -m`)
* [ ] Fully customizable commit rules via config
* [ ] Plugin system for rule and generator extensions

```

---

Let me know if you want a minimal or light version of this README (e.g., for crates.io or GitHub summary).
```
