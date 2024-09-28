
---
# study_rust 记录学习
-------------

> 关于我  
博客：[无](http://) 

仓库主要用于学习，随便记录一些东西
- study_inner : 学习rust内置包
- study_async : 学习异步操作
- study_other : 学习第三方包


## Cargo工作空间（workspaces）创建
```shell
mkdir study_rust && cd study_rust
vim Cargo.toml
```
### 输入配置信息到Cargo.toml
```toml
[workspace]
members = []
resolver = "2"

[profile.release]
codegen-units = 1
lto = true
opt-level = "z"
panic = "abort"
```
### 新增三个可执行项目
```shell
cargo new study_inner
cargo new study_other
cargo new study_other
# 也可以创建lib项目
cargo new study_other --lib
```


