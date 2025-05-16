# ⚖️ Gitlaw

> **Discipline your Git habits.**  
> A fast, Rust-powered CLI tool that helps you follow consistent commit message rules — every single time.

---

## 🚀 What is Gitlaw?

`Gitlaw` is a personal CLI wrapper around Git’s most common commands:  
`add`, `commit`, and `push`.

Right now, its primary focus is on **commit message enforcement**.  
It requires you to follow the [Conventional Commits](https://www.conventionalcommits.org/) format — or it blocks the commit.

---

## 🧪 How it works

- `gitlaw add .`  
  Behaves like `git add .`  
  _*(Rules may be added in the future)*_

- `gitlaw commit "message"`  
  Wraps `git commit` with all the standard flags — except:
  - ✅ Commit message is **mandatory**
  - 🚫 `-m` does **not** exist — you must pass the message directly

- `gitlaw push ...`  
  Behaves like `git push ...`  
  _*(Rules may be added in the future)*_

---

## 🔧 Future Updates

- Rules will be configurable via a file (e.g. `.gitlaw.toml`)
- i have no idea whats next but chatgpt says ↓ 

| Growth Area                        | How It Adds Value                                                |
| ---------------------------------- | ---------------------------------------------------------------- |
| **Config support**                 | Lets devs tune rules to their style (`.gitlaw.toml`)             |
| **Emoji/gitmoji mode**             | Adds flavor for commit nerds who want `✨ feat:`                  |
| **Interactive mode (`--compose`)** | Helps those who write good commits, but blank out under pressure |
| **Amend protection**               | Prevents devs from amending a good commit with garbage           |
| **Push protection**                | (Later) Prevents `git push origin main` at 2am 😅                |
| **Changelog generation**           | Bonus feature for power users or open source maintainers         |

¯ \  _(ツ) _ / ¯

