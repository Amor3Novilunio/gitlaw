# Gitlaw

**Gitlaw** is a command-line tool that enforces structured, meaningful commit practices by validating commit messages and pairing them with self-documented change files.

## 🚀 Purpose

Gitlaw helps developers write clear, consistent commit messages and generate documentation that explains each commit's intent and purpose. This creates a readable, maintainable Git history and encourages intentional development.

## 🔧 How It Works

* Developers run `gitlaw commit "<message>"`
* Gitlaw validates the commit message based on custom rules
* Gitlaw parses a unique identifier (e.g., `law42`) from the message
* Each law ID must be unique (e.g., no duplicate `law42`)
* If a corresponding `.gitlaw/law42.md` file doesn't exist:

  * Gitlaw creates a template Markdown file for the user to fill in
  * The commit is blocked until the file is edited
* Once the `.md` is completed and the message passes validation, Gitlaw forwards the commit to Git

## 🔲 Features

* Commit message validation (prefixes, length, empty content, etc.)
* Auto-generates structured `.md` files for each commit reference
* Enforces documentation before allowing commits
* Configurable rules and templates via `gitlaw.toml`
* Local-first, zero-dependency workflow

## 📖 Commit Message Format

Example:

```bash
feat(auth):law42
```

This will trigger Gitlaw to:

* Validate that `feat(auth):` is a valid prefix
* Look for or generate `.gitlaw/law42.md`

## 🖄 Generated File Template

```markdown
# feat(auth):law42

## What this change does
- [ ] Describe the feature or fix

## Why it matters
- [ ] Explain the problem being solved or the value added

## Notes (optional)
...
```

## ⚖️ Philosophy

Gitlaw is built on the idea that better commit messages create better software history. By forcing intention and documentation at the moment of commit, it enables:

* Easier collaboration
* More informed reviews
* Traceable decision making
* Human-readable changelogs

## ✅ Why Use Gitlaw?

* You want your commits to be meaningful and traceable
* You’re tired of vague messages like `fix bug` or `update code`
* You want local documentation without using Jira, Notion, or GitHub PRs

---

Built with Rust. Inspired by discipline. Driven by clarity.
