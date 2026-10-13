# rust-cs

基于 Rust 编程语言的个人学习项目：一边写一边记录 Windows 原生界面与 Rust 的结合方式。

仓库地址：<https://github.com/no-codetime/rust-cs>

## 项目内容

本工程是一个 Cargo 包，包含三个可执行目标：

| 可执行文件 | 源码 | 说明 |
| --- | --- | --- |
| `rust-cs` | [src/main.rs](src/main.rs) | 控制台示例：打印提示、读取标准输入并回显，再用 `webbrowser` 打开本仓库主页 |
| `guessing_game` | [src/bin/guessing_game.rs](src/bin/guessing_game.rs) | 猜数游戏练习：`rand` 生成 1~100 的秘密数字，输入用 `parse` 转成数字后由 `cmp` + `match` 判断猜大、猜小还是猜中 |
| `winui3_window` | [src/bin/winui3_window.rs](src/bin/winui3_window.rs) | WinUI 3 最小窗口示例：Mica 背景、`TextBox`、`Button`、`InfoBar`，演示 `windows-reactor` 的 Component / View 模型 |

## 环境要求

- **系统**：Windows 10 1809（17763）及以上，或 Windows 11 —— WinUI 3 的平台要求
- **Rust**：1.95 或更高版本。本工程使用 2024 edition，且依赖的 `windows-reactor` 声明了 `rust-version = "1.95"`
- **工具链**：MSVC 目标（`x86_64-pc-windows-msvc`），需安装 Visual Studio 生成工具中的“使用 C++ 的桌面开发”
- **运行 `winui3_window`**：需要系统中已安装 Windows App SDK 运行时，这是 WinUI 3 应用的运行依赖

## 构建与运行

```powershell
# 控制台示例（bin 名与包名相同）
cargo run --bin rust-cs

# 猜数游戏
cargo run --bin guessing_game

# WinUI 3 窗口示例
cargo run --bin winui3_window

# 发布构建：三个可执行文件都会生成到 target/release/
cargo build --release
```

`winui3_window.rs` 顶部带有 `#![windows_subsystem = "windows"]`，因此它是纯 GUI 程序，启动时不会额外弹出控制台窗口。

## 依赖说明

| crate | 版本 | 用途 |
| --- | --- | --- |
| [`rand`](https://crates.io/crates/rand) | 0.10.3 | 生成随机数，猜数游戏用 `rand::rng().random_range(1..101)` 取 1~100 的秘密数字 |
| [`webbrowser`](https://crates.io/crates/webbrowser) | 1.2.4 | 调用系统默认浏览器打开链接 |
| [`windows-reactor`](https://crates.io/crates/windows-reactor) | 0.100 | 声明式 WinUI 3 库：`Component` 持有状态、处理类型化消息并返回 `View`，由 Reactor 把视图差异同步到原生控件树 |
| [`winres`](https://crates.io/crates/winres) | 0.1.12 | 构建期依赖，把 `icon.ico` 编译进 exe 资源（见 [build.rs](build.rs)） |

## windows-reactor 的组件循环

`winui3_window.rs` 里的模式可以概括为一条单向数据流：

1. 原生控件触发事件（如 `on_text_changed`、`on_click`），通过 `context.callback(...)` / `context.message(...)` 转成自定义的 `Message`；
2. `update(&mut self, message, ...)` 接收消息并修改自身状态，这里是唯一的写状态入口；
3. `view(&self, ...)` 根据当前状态重新描述整个界面，返回 `View`；
4. Reactor 把新视图与原生控件树做对比，只应用必要的 WinUI 改动。

因此界面是状态的函数，不需要手工去改控件属性。

## 目录结构

```
rust-cs/
├── build.rs                  # 构建脚本：写入可执行文件图标（仅 Windows）
├── Cargo.toml
├── Cargo.lock                # 可执行程序，锁文件需要提交
├── icon.ico                  # 被 build.rs 引用，不可删除
├── src/
│   ├── main.rs               # 控制台示例
│   └── bin/
│       ├── guessing_game.rs  # 猜数游戏练习
│       └── winui3_window.rs  # WinUI 3 窗口示例
└── README.md
```

## 备注

- 本项目是可执行程序而非库，`Cargo.lock` 一并提交以保证依赖版本可复现。
- `icon.ico` 被 [build.rs](build.rs) 引用，缺少它会导致构建失败。
