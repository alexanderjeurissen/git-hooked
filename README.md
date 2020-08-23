#  WIP: git-hooked

![image](https://github.com/alexanderjeurissen/git-hooked/blob/main/public/assets/logo.png?raw=true)

- WIP: Create symbolic links from your existing git hooks and track them with git version control
- written in Rust for optimal performance

Inspired by:

- [husky](https://github.com/typicode/husky)

### Configuration WIP

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
```


### WIP: Motivation




