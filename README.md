# remini-cli

Rust 重写版 `gemini-cli` 项目。

当前仓库目标：

1. 按 `doc/spec.md` 完成 100% 功能重写（含 SDK、A2A Server、VSCode IDE Companion）。
2. 按 `doc/coverage-matrix.md` 做逐项验收。
3. 分小步提交到 `main`，并在 GitHub 保持可追踪的时间线。

## 当前进展（已完成到可运行骨架）

1. Rust workspace 已初始化（`remini-bin`、`remini-core`、`remini-config`、`remini-tools`）。
2. CLI 最小链路已打通：
   - 交互/非交互模式判断
   - `--approval-mode`
   - `--output-format`（`text/json/stream-json`）
3. 只读工具最小实现：
   - `read_file`
   - `list_directory`
   - `glob_search`
   - `grep_search`
4. 命令最小实现：
   - Slash：`/about`、`/help`、`/tools`、`/tools desc|nodesc`
   - At：`@path`（文件/目录最小展开）
   - Bang：`!command`（最小 shell 执行）
5. 测试已接入并持续通过（`cargo test`）。

## 快速开始

### 1) 安装依赖并运行测试

```bash
cargo test
```

### 2) 运行 CLI（headless）

```bash
# 文本输出
cargo run -p remini-bin -- -p "/help"

# JSON 输出
cargo run -p remini-bin -- -p "/tools" -o json

# Stream JSON 输出
cargo run -p remini-bin -- -p "/tools desc" -o stream-json

# @path 示例
cargo run -p remini-bin -- -p "@Cargo.toml summarize"

# !command 示例
cargo run -p remini-bin -- -p "!printf hello"
```

### 3) 查看帮助

```bash
cargo run -p remini-bin -- --help
```

## 目录结构

```text
.
├── Cargo.toml
├── crates
│   ├── remini-bin        # CLI 入口
│   ├── remini-config     # 配置解析与合并
│   ├── remini-core       # 命令/编排/调度核心
│   └── remini-tools      # 工具实现（文件、搜索等）
└── doc
    ├── spec.md           # 重写规格与阶段计划
    ├── coverage-matrix.md# 功能覆盖矩阵
    └── weekly            # 周度里程碑日志
```

## 关键文档

1. 规格文档：`doc/spec.md`
2. 覆盖矩阵：`doc/coverage-matrix.md`
3. 周度日志：`doc/weekly/*.md`

## 开发原则

1. 每次只做一小步，功能与测试一起提交。
2. 所有提交必须能映射到覆盖矩阵项。
3. 以用户可观察行为对齐 `gemini-cli`，实现细节允许不同。

## 下一阶段

1. 继续补全 Slash 命令与参数体系。
2. 扩展 ToolRegistry 到更多内置工具与确认流。
3. 逐步接入 settings 多层加载、policy engine、MCP、extensions、skills、hooks。
4. 完成 headless 与交互模式行为对齐测试。
