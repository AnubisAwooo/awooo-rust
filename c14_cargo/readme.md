
# 采用发布配置自定义构建

```

cargo build

cargo build --release

```
优化等级不同
```
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

# 将 crate 发布到 Crates.io
```
cargo doc --open
```
/// 3斜线文档注释 支持 markdown 语法
使用 pub use 导出合适的公有 API

发布版本
```
cargo publish
```

撤回版本
```
cargo yank
```

# Cargo 工作空间

Cargo.toml
```
[workspace]

members = [
    "adder",
]
```
```
cargo run -p adder
```

# 使用 cargo install 从 Crates.io 安装二进制文件

