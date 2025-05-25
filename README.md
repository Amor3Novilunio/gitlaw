# 🧑‍⚖️ Gitlaw

**A modern, opinionated Git CLI wrapper that enforces commit structure, enables AI-assisted messages, and auto-documents your code history — one law at a time.**

---

## ✨ Features

- 🔒 **Enforced commit rules** (conventional commits, no WIP, length limits)
- 🤖 **AI-generated commit messages** based on your Git diff
- 📋 **Interactive commit UI** — approve, rewrite, or regenerate messages
- 📁 **Auto-generates law documents** like `gitlaw/law42.md`
- 🧠 **Diff-based changelogs** — every law captures before/after changes
- 🧼 **Passthrough for all non-commit Git commands**

---

## 🛠️ Installation

```bash
git clone https://github.com/your-username/gitlaw.git
cd gitlaw
cargo build --release
```

Then add it to your `PATH`:

```bash
export PATH="$PWD/target/release:$PATH"
```

---

## ⚙️ Usage

### 🧑‍⚖️ Commit (AI + Validation)

```bash
gitlaw commit
```

Gitlaw will:
1. Analyze your staged changes
2. Ask AI to generate a conventional commit message
3. Present options to accept, regenerate, or write your own
4. Automatically commit with the chosen message
5. Save the diff as `gitlaw/law42.md` if the message includes `law42`

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
  └─ law42.md   # includes before/after code blocks per file
```

---

## 🚀 Roadmap

- [x] Basic Git passthrough
- [ ] Commit validation (prefix, WIP block, max length)
- [ ] Extract commit message from args
- [ ] AI-generated commit messages (OpenAI/local model)
- [ ] Interactive CLI prompt for message selection
- [ ] Auto-law doc generation from diffs
- [ ] Customizable commit rules via config(optional? maybe? who knows?)
- [ ] Plugin system for rule extensions(optional? maybe? who knows?)

## ⚙️ AI Modes

Gitlaw supports two modes of AI commit generation:

### Offline Mode
- Runs AI models **locally**
- Requires downloading a model (e.g., `deepseek`)
- No internet required after setup

### Online Mode
- Uses public APIs like OpenAI or Groq
- Requires an API key and an internet connection
- May be subject to rate limits or usage caps
- ⚠️ online models have a token limit per request for free tier
   - 🧠 Think of tokens like words + punctuation:
   - "fix: update docs" = ~5 tokens


# NEW REQUIREMENTS

 - no more online
 - download ph-2 model | these models have a pros n cons
 - we need to install llama.cpp not just phi-2

## I REALLY NEED TO UPDATE THIS README DUE TO NEW REQUIREMENTS


| Level  | Quality       | Size               | Speed         | Notes                                                       |
| ------ | ------------- | ------------------ | ------------- | ----------------------------------------------------------- |
| **Q2** | 🟥 Low        | ✅ Smallest         | ⚡ Fastest     | Often too lossy for good summaries                          |
| **Q3** | 🟧 Medium-low | ✅ Small            | ⚡ Fast        | Okay for basic tasks, but not reliable for commit summaries |
| **Q4** | 🟨 Good       | ✅ Balanced         | ⚡ Fast enough | 👍 Great quality/speed tradeoff (RECOMMENDED)               |
| **Q5** | 🟩 Very good  | ❗ Bigger (\~2.5GB) | 🐢 Slower     | Almost lossless, but higher RAM cost                        |
| **Q6** | 🟩 Best       | ❌ Heavier          | 🐢 Slowest    | Almost like the original model — overkill for Gitlaw        |
