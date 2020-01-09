# logno

A Rust `log` implementation with **N**o std**o**t.

**ALERT**: this implementation **occupies `stdout`**!

This impl originates from specific TeamCity behaviour.

- [x] Only `Error` level logs go to stderr, others go to stdout
- [ ] Format with https://www.jetbrains.com/help/teamcity/build-script-interaction-with-teamcity.html
  - Detect whether running in TeamCity with env var `TEAMCITY_VERSION`
  - Only with `teamcity` feature flag
