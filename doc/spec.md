# Remini CLI Rust 重写规格说明（Spec v0.1）

- 项目：`remini-cli`
- 重写目标：`gemini-cli`（本地基线目录：`/home/liber/Downloads/rewrite_opensource/gemini-cli`）
- 基线版本：`0.30.0-nightly.20260210.a2174751d`
- Spec 日期：`2026-02-27`
- 执行原则：先完善 Spec，再开始代码实现
- 已确认决策（`2026-02-27`）：
  1. 100% 范围包含：`SDK`、`A2A Server`、`VSCode IDE Companion`
  2. Git 只接受真实提交时间，不允许回填提交日期
  3. 开发分支策略：直接在 `main` 提交
  4. 命令命名：`remini`

## 1. 目标与“100%功能完成”定义

“100%功能完成”定义为：**以用户可观察行为为准**，在相同输入、配置、权限与环境下，Rust 版 `remini-cli` 与基线 `gemini-cli` 在以下方面保持等价：

1. 命令行参数行为（交互/非交互、返回码、输出格式）。
2. Slash 命令行为（含子命令、别名、可用性条件）。
3. `@` 文件注入、`!` shell 执行/切换行为。
4. Tool 调度、确认、拒绝、报错与结果展示流程。
5. 配置加载优先级（默认/系统默认/用户/项目/系统覆盖/env/CLI 参数）。
6. 核心子系统：MCP、Extensions、Skills、Hooks、Policy、Sandbox、Session/Checkpoint/Rewind、Telemetry、IDE 集成。
7. 自动化验证：单测 + 集成测试 + 行为回放（golden）通过。

说明：允许内部实现不同（TypeScript -> Rust），但不允许用户行为回归。

## 2. 基线能力盘点（来自本地源码与文档）

### 2.1 运行模式与输出

1. 交互式 TUI 模式（默认）。
2. 非交互（headless）模式。
3. 输出格式：`text` / `json` / `stream-json`。
4. 典型返回码：`0`（成功）、`1`（通用错误）、`42`（输入错误）、`53`（超限）。

### 2.2 CLI 参数（核心）

关键参数包括：`--model`、`--prompt/-p`、`--prompt-interactive/-i`、`--sandbox/-s`、`--approval-mode`、`--yolo`、`--extensions/-e`、`--resume/-r`、`--include-directories`、`--output-format/-o`、`--debug` 等。

### 2.3 Slash 命令（内置）

来源：`packages/cli/src/services/BuiltinCommandLoader.ts`。

主要命令族（公开能力）：

1. `/about` `/auth` `/bug` `/chat` `/clear` `/commands` `/compress` `/copy`
2. `/directory` `/docs` `/editor` `/extensions` `/help`
3. `/hooks` `/ide` `/init` `/mcp` `/memory` `/model`
4. `/permissions` `/plan` `/policies` `/privacy`
5. `/quit` `/restore` `/rewind` `/resume` `/settings` `/shells`
6. `/setup-github` `/skills` `/stats` `/terminal-setup` `/theme` `/tools` `/vim`

条件/内部命令（需按基线条件实现）：

1. `/agents`（配置开启时）
2. `/oncall`（nightly）
3. `/profile`（development）
4. `/corgi`（内部命令）

### 2.4 Prompt 前缀能力

1. `@path`：触发 `read_many_files`，支持目录、多文件、过滤规则。
2. `!command`：触发 shell 命令执行。
3. `!`：切换 shell mode。

### 2.5 Core Tool 集（内置）

来源：`packages/core/src/tools/definitions/base-declarations.ts` 与 `tool-names.ts`。

1. `glob`
2. `grep_search`
3. `list_directory`
4. `read_file`
5. `read_many_files`
6. `write_file`
7. `replace`
8. `run_shell_command`
9. `google_web_search`
10. `web_fetch`
11. `write_todos`
12. `save_memory`
13. `get_internal_docs`
14. `activate_skill`
15. `ask_user`
16. `enter_plan_mode`
17. `exit_plan_mode`

### 2.6 配置与策略

1. `settings.json` 多层合并与优先级覆盖。
2. 顶层配置域（示例）：`general`、`tools`、`mcpServers`、`extensions`、`hooks`、`skills`、`telemetry`、`security`、`admin` 等。
3. Policy Engine（allow/deny/ask_user，按 tier + priority）。
4. Folder Trust + Approval Mode（`default/auto_edit/plan/yolo`）。

### 2.7 测试体系（需对齐）

1. Unit tests（CLI/Core 子模块）。
2. Integration tests（文件系统、hooks、MCP、plan mode、json output、sandbox 矩阵等）。
3. Eval tests（行为质量验证套件）。

## 3. Rust 重写总体架构（提案）

建议采用 Rust workspace 多 crate 设计，保持职责分离：

1. `crates/remini-bin`
   - CLI 入口与参数解析（`clap`）
2. `crates/remini-core`
   - 对话编排、模型调用、tool 调度、会话状态
3. `crates/remini-config`
   - 配置 schema、层级加载、env 展开、校验
4. `crates/remini-policy`
   - 规则加载（TOML）、匹配、优先级计算、决策
5. `crates/remini-tools`
   - 内置 tools（fs/shell/web/memory/todos/plan）
6. `crates/remini-mcp`
   - MCP server 管理、tool discover、OAuth
7. `crates/remini-ext`
   - extensions 生命周期与命令
8. `crates/remini-skills`
   - skills 发现、加载、启停
9. `crates/remini-hooks`
   - hooks 事件总线与生命周期
10. `crates/remini-session`
   - chat 保存/恢复、checkpoint、rewind
11. `crates/remini-tui`
   - 交互界面（建议 `ratatui` + `crossterm`）
12. `crates/remini-headless`
   - `json/stream-json` 输出协议与退出码
13. `crates/remini-telemetry`
   - 遥测与匿名化输出
14. `crates/remini-sdk`（后期）
   - Rust SDK/嵌入式 API（对齐原 sdk 能力）
15. `crates/remini-a2a-server`（后期）
   - A2A server 能力（对齐 a2a-server 包）
16. `crates/remini-ide-companion`（后期）
   - VSCode IDE Companion 对齐能力

## 4. 分阶段实现计划（先文档后编码）

### Phase 0：基线冻结与验收协议

1. 冻结对标版本（当前以本地 `gemini-cli` 快照为准）。
2. 产出功能覆盖矩阵（命令、参数、tool、配置、子系统）。
3. 明确“通过条件”：每项功能必须有测试映射。

DoD：

1. `doc/spec.md` 与覆盖矩阵可审阅。
2. 所有后续任务能映射到矩阵项 ID。

### Phase 1：工程骨架与最小可运行

1. Rust workspace 初始化。
2. 基础日志、错误码、配置读取骨架。
3. `remini --help`、`--version`、`--prompt` 打通。

DoD：

1. 可编译、可运行、CI 基础通过。

### Phase 2：Core 调度与最小 Tool 链路

1. 对话循环、tool 注册/调用协议。
2. 先接入只读工具（`read_file`/`list_directory`/`glob`/`grep_search`）。

DoD：

1. 最小端到端：输入 -> tool 调用 -> 模型续答。

### Phase 3：文件修改与 shell 执行

1. `write_file`/`replace`/`run_shell_command`。
2. approval 与 diff 展示能力。

DoD：

1. 集成测试覆盖“读-改-执行”主路径。

### Phase 4：交互命令系统（Slash/@/!）

1. Slash 命令解析器与命令注册器。
2. `@` 注入与 `!` shell mode。

DoD：

1. 基线高频命令对齐（help/chat/model/memory/tools 等）。

### Phase 5：配置系统与策略引擎

1. settings 多层合并。
2. policy tier + priority + mode。
3. folder trust 与 approval mode 对齐。

DoD：

1. 关键配置路径与策略规则测试通过。

### Phase 6：MCP / Extensions / Skills / Hooks

1. MCP add/remove/list/enable/disable + tool discover。
2. Extensions 生命周期管理。
3. Skills 发现/启停/安装链接。
4. Hooks 系统事件执行。

DoD：

1. 子系统集成测试通过。

### Phase 7：Session / Checkpoint / Rewind

1. session 自动保存、resume、list/delete。
2. checkpoint restore 与 rewind 能力。

DoD：

1. 时间线回滚路径可测试、可恢复。

### Phase 8：Headless 与输出协议

1. `text/json/stream-json` 输出稳定化。
2. 退出码与错误结构统一。

DoD：

1. 自动化脚本模式可稳定使用。

### Phase 9：安全与可运维能力

1. sandbox（none/docker/podman/seatbelt）抽象。
2. telemetry、debug、合规输出。

DoD：

1. 安全场景与观测场景测试通过。

### Phase 10：生态能力收口

1. SDK 对齐（优先核心 API）。
2. A2A server 对齐。
3. VSCode IDE Companion 对齐。

DoD：

1. 发布工件与迁移文档完成。

### Phase 11：100% 功能验收与发布

1. 逐项关闭覆盖矩阵。
2. 回归测试全绿。
3. 发布说明与兼容性声明。

DoD：

1. 覆盖矩阵达到 100%，无 P0/P1 未关闭项。

## 5. Git 提交策略（分次提交 + 每周可见）

## 5.1 基本规则

1. 每周至少 1 次合并到默认分支（建议 2-5 次）。
2. 每次提交必须对应真实可审阅变更（文档、代码、测试、修复）。
3. 提交信息采用约定式：`feat:` `fix:` `refactor:` `test:` `docs:`
4. 禁止把一整阶段压成单提交；按“功能 + 测试”最小闭环提交。

## 5.2 GitHub 可见性注意点

1. 提交需在默认分支（`main`）或通过 PR 合并入默认分支。
2. commit email 必须绑定到 GitHub 账号（否则贡献图不计入）。
3. 推送到 `origin` 后才会在 GitHub 可见。
4. 禁止手工回填 `GIT_AUTHOR_DATE` / `GIT_COMMITTER_DATE` 伪造历史时间。

## 5.3 历史周里程碑映射（2025-11 到 2026-02）

> 起始按周一计算，从 `2025-11-03` 到当前周 `2026-02-23`。

1. `2025-11-03 ~ 2025-11-09`：项目立项、目标与范围文档
2. `2025-11-10 ~ 2025-11-16`：基线源码盘点、能力矩阵初稿
3. `2025-11-17 ~ 2025-11-23`：架构方案评审（crate 拆分）
4. `2025-11-24 ~ 2025-11-30`：CLI 参数与输出协议设计
5. `2025-12-01 ~ 2025-12-07`：core/tool 调度协议设计
6. `2025-12-08 ~ 2025-12-14`：文件系统 tools 设计与测试计划
7. `2025-12-15 ~ 2025-12-21`：shell/approval/policy 设计
8. `2025-12-22 ~ 2025-12-28`：slash 命令体系设计
9. `2025-12-29 ~ 2026-01-04`：session/checkpoint/rewind 设计
10. `2026-01-05 ~ 2026-01-11`：MCP 子系统设计
11. `2026-01-12 ~ 2026-01-18`：extensions/skills/hooks 设计
12. `2026-01-19 ~ 2026-01-25`：headless/json/stream-json 设计
13. `2026-01-26 ~ 2026-02-01`：sandbox/telemetry 设计
14. `2026-02-02 ~ 2026-02-08`：Rust 工程初始化与基础 CI
15. `2026-02-09 ~ 2026-02-15`：最小 core + 只读 tools 打通
16. `2026-02-16 ~ 2026-02-22`：命令系统与配置系统首版
17. `2026-02-23 ~ 2026-03-01`：spec 收敛、里程碑拆解、开始实现

说明：

1. 以上是历史里程碑映射，用于说明计划与产出，不等同于真实历史 commit。
2. 在 `2026-02-27` 这个当前时间点，无法生成 `2025-11-03` 至 `2026-02-23` 各周的“真实发生时间”commit。
3. 从当前周开始，按真实时间持续每周提交并推送 GitHub。

## 5.4 真实周提交计划（从 2026-03 开始）

1. `2026-03-02 ~ 2026-03-08`：Rust workspace 初始化 + CLI skeleton
2. `2026-03-09 ~ 2026-03-15`：core 调度 + read-only tools
3. `2026-03-16 ~ 2026-03-22`：write/edit/shell + approval
4. `2026-03-23 ~ 2026-03-29`：slash 命令首批 + `@`/`!`
5. `2026-03-30 ~ 2026-04-05`：settings 多层合并 + schema 校验
6. `2026-04-06 ~ 2026-04-12`：policy engine + trusted folder
7. `2026-04-13 ~ 2026-04-19`：MCP 主链路 + mcp 命令
8. `2026-04-20 ~ 2026-04-26`：extensions + skills + hooks
9. `2026-04-27 ~ 2026-05-03`：session/checkpoint/rewind
10. `2026-05-04 ~ 2026-05-10`：headless + json/stream-json
11. `2026-05-11 ~ 2026-05-17`：SDK + A2A server
12. `2026-05-18 ~ 2026-05-24`：VSCode IDE Companion + 回归

执行要求：

1. 每周至少 1 个真实提交（建议 2-5 个）。
2. 每个功能提交必须配套测试或测试占位提交。
3. 每周结束前推送到 `origin/main` 并核对 GitHub 可见性。

## 6. 质量门禁（每阶段必须满足）

1. 功能：对应矩阵项标记为 completed。
2. 测试：新增功能必须有单测；关键路径要有集成测试。
3. 兼容：输入/输出/错误码与基线一致或在 spec 明确差异。
4. 文档：命令、配置、迁移说明同步更新。

## 7. 风险与缓解

1. 风险：基线上游仍迭代，目标漂移。
   - 缓解：冻结基线版本；增设“追主线同步”独立里程碑。
2. 风险：MCP/Extensions/Hooks 跨子系统耦合高。
   - 缓解：先定义统一事件模型与生命周期，再并行开发。
3. 风险：TUI 行为难 1:1 复现。
   - 缓解：把“语义等价”与“视觉细节”等级化验收。
4. 风险：用户要求“历史周真实提交”与当前时间冲突。
   - 缓解：明确历史周只保留里程碑映射；从 `2026-02-27` 起只做真实时间提交。

## 8. 立即执行项（Spec 完成后）

1. 锁定仓库默认分支与提交策略（直推 main 或 PR 合并）。
2. 建立 `Coverage Matrix` 文档（按功能 ID 跟踪）。
3. 创建 Rust workspace 与最小 CLI 可执行入口。
4. 从 Phase 1 开始按周推进并提交。

## 9. 已确认约束

1. 100% 范围：必须包含 `SDK`、`A2A Server`、`VSCode IDE Companion`。
2. 提交时间：必须为真实时间，禁止回填。
3. 分支策略：在 `main` 持续分次提交。
4. 产品命名：统一为 `remini`。
