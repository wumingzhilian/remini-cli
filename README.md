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
   - `--model`（headless 输出中携带 model 字段）
   - `--approval-mode`
   - `--debug/-d`（支持 `DEBUG=true|1` / `DEBUG_MODE=true|1`）
   - `--resume/-r`（支持 `--resume` 默认恢复 `latest`）
   - `--include-directories`（支持逗号分隔与多次传入，用于 `@path` 查找附加目录）
   - settings 最小加载（`~/.gemini/settings.json` 与 `<workspace>/.gemini/settings.json`，workspace 优先）
   - `--output-format`（`text/json/stream-json`）
3. 只读工具最小实现：
   - `read_file`
   - `read_many_files`
   - `list_directory`
   - `glob_search`
   - `grep_search`
4. 命令最小实现：
   - Slash：`/about`、`/auth`、`/bug`、`/clear`、`/commands`、`/copy`、`/directory`、`/help`、`/model`、`/model set <name>`、`/quit`、`/resume`、`/stats`、`/tools`、`/tools desc|nodesc`
   - At：`@path`（文件/目录最小展开）
   - Bang：`!command`（最小 shell 执行）
5. headless 错误结构与输入错误退出码（`42`）已接入。
6. 测试已接入并持续通过（`cargo test`）。

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

# 指定 model（会在 JSON stats 中显示）
cargo run -p remini-bin -- -m gemini-2.5-flash -p "/tools" -o json

# 交互模式恢复会话（--resume 无值等价 latest）
cargo run -p remini-bin -- --resume

# 开启 debug（stderr 会输出 debug 模式提示）
cargo run -p remini-bin -- --debug -i "hello"

# Stream JSON 输出
cargo run -p remini-bin -- -p "/tools desc" -o stream-json

# @path 示例
cargo run -p remini-bin -- -p "@Cargo.toml summarize"

# 从附加目录解析 @path（可多次传入或逗号分隔）
cargo run -p remini-bin -- --include-directories docs,crates -p "@README.md summarize"

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
