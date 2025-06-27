# study_rust 记录学习

---

> 关于我
> 博客：[无](http://) 

仓库主要用于学习，随便记录一些东西

- design_patterns : 设计模式
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

### 添加依赖
```shell
# 方案一
# 添加依赖到 study_other 子项目
cargo add rayon --manifest-path study_other/Cargo.toml

# 方案二
# 根目录 Cargo.toml
[workspace.dependencies]
serde = "1.0"

# 在子项目中引用
[dependencies]
serde.workspace = true
```



### 新增三个可执行项目

```shell
cargo new study_inner
cargo new study_other
cargo new study_other
# 也可以创建lib项目
cargo new study_other --lib
```
