# ⚖️ Gitlaw

> **Discipline your Git habits.**  
> A fast, Rust-powered CLI tool that helps where you can apply rules for your own workflow.

---

## 🚀 What is Gitlaw?

`Gitlaw` is a personal CLI wrapper around Git

Right now, its primary focus is on **commit message enforcement**.  
It requires you to follow the [Conventional Commits](https://www.conventionalcommits.org/) format — or it blocks the commit.

---

## 🧪 How it works

- `gitlaw add, push etc..`
  are all equivalent to git add, push etc

  however regarding on the commit

- `gitlaw commit "message"`  
  Wraps `git commit` with all the standard flags — except:
  - ✅ Commit message is **mandatory**
  - 🚫 `-m` does **not** exist — you must pass the message directly

---

## 🔧 Future Updates

- Rules will be configurable via a file (e.g. `.gitlaw.toml`)
- i have no idea whats next but chatgpt says ↓

| Growth Area                        | How It Adds Value                                                |
| ---------------------------------- | ---------------------------------------------------------------- |
| **Config support**                 | Lets devs tune rules to their style (`.gitlaw.toml`)             |
| **Emoji/gitmoji mode**             | Adds flavor for commit nerds who want `✨ feat:`                 |
| **TextGenerator**                  | i have no idea if this is possible but lemme do R&D              |
| **Changelog generation**           | Bonus feature for power users or open source maintainers         |

¯ \ _(ツ) _ / ¯
