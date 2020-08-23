![image](https://github.com/alexanderjeurissen/git-hooked/blob/main/public/assets/logo.png?raw=true)

- Create symbolic links for your existing git hooks and track them with git version control.
- Full control over what git hooks should be tracked, and how file / symlink conflicts should be handled.
- [WIP] Presets allow for defining environment specific rules for tracking / setting git hooks.
- written in Rust for optimal performance

Inspired by:

- [husky](https://github.com/typicode/husky)
- [githooks](https://github.com/rycus86/githooks)
- [git-hooks](https://github.com/git-hooks/git-hooks)
- [two ways to share git hooks with your team](https://www.viget.com/articles/two-ways-to-share-git-hooks-with-your-team/)

### Configuration WIP

*git-hooked* can be configured by means of a `git_hooked.config.toml` configuration file.

*git-hooked* will search for this configuration file in the following locations:

1. Current working directory
2. Home Directory

It's also possible to provide a config path at runtime using the `--config` option.

The configuration file contains a section for each hook that you are interested in tracking.
For each hook the following fields can be defined:

| field | default value | description |
|-------|---------------|-------------|
| name  | N/A           | The actual git hook name in CamelCase. |
| create | true | Wether *git-hooked* should create a symlink for this hook when running the `push` command |
| relink | true | Specifies if incorrect symbolic links should be automatically overwritten.|
| force | false | Specifies if the git hook should be forcibly linked. This can cause irreversible data loss! Use with caution! |
| track | true | Wether *git-hooked* should track this hook when running the `pull` command.|


Here is an example of what a configuration could look like.

**git-hooked.config.toml**

```toml
[[hooks]]
name="PreCommit"
create=true
relink=true

[[hooks]]
name="PrePush"
create=true
relink=true
force=true

[[hooks]]
name="PostCheckout"
create=true
relink=true
track=false
```

### WIP: Motivation




