# Remini CLI 功能覆盖矩阵（Coverage Matrix v0.1）

- 项目：`remini-cli`
- 基线：`gemini-cli@0.30.0-nightly.20260210.a2174751d`
- 更新日期：`2026-02-27`
- 目标：矩阵项 `100% = Done`

状态定义：

1. `Not Started`：未开始
2. `In Progress`：开发中
3. `Blocked`：受阻
4. `Done`：功能与测试完成并通过

## A. CLI 运行与参数

| ID | 能力 | 基线参考 | 优先级 | 状态 |
| --- | --- | --- | --- | --- |
| CLI-001 | 交互模式启动（TTY） | `packages/cli/src/config/config.ts` | P0 | In Progress |
| CLI-002 | 非交互模式启动（headless） | `docs/cli/headless.md` | P0 | In Progress |
| CLI-003 | `--prompt/-p` 行为 | `packages/cli/src/config/config.ts` | P0 | In Progress |
| CLI-004 | `--prompt-interactive/-i` 行为 | `packages/cli/src/config/config.ts` | P0 | In Progress |
| CLI-005 | positional query 行为 | `packages/cli/src/config/config.ts` | P0 | In Progress |
| CLI-006 | `--model/-m` 行为 | `docs/cli/cli-reference.md` | P1 | Not Started |
| CLI-007 | `--sandbox/-s` 行为 | `docs/cli/sandbox.md` | P0 | Not Started |
| CLI-008 | `--approval-mode` 行为 | `packages/cli/src/config/config.ts` | P0 | In Progress |
| CLI-009 | `--yolo` 兼容行为 | `packages/cli/src/config/config.ts` | P1 | Not Started |
| CLI-010 | `--resume/-r` 行为 | `packages/cli/src/config/config.ts` | P1 | Not Started |
| CLI-011 | `--include-directories` 行为 | `packages/cli/src/config/config.ts` | P1 | Not Started |
| CLI-012 | `--output-format/-o` 行为 | `packages/cli/src/config/config.ts` | P0 | In Progress |
| CLI-013 | `--help`/`--version` 行为 | `packages/cli/src/config/config.ts` | P0 | Done |
| CLI-014 | `--debug` 与 debug 日志 | `packages/cli/src/config/config.ts` | P1 | Not Started |
| CLI-015 | 返回码对齐（0/1/42/53） | `docs/cli/headless.md` | P0 | In Progress |

## B. Slash 命令系统

| ID | 能力 | 基线参考 | 优先级 | 状态 |
| --- | --- | --- | --- | --- |
| CMD-001 | Slash 命令解析与分发 | `packages/cli/src/services/BuiltinCommandLoader.ts` | P0 | In Progress |
| CMD-002 | `/about` | `packages/cli/src/ui/commands/aboutCommand.ts` | P1 | In Progress |
| CMD-003 | `/auth` | `packages/cli/src/ui/commands/authCommand.ts` | P1 | Not Started |
| CMD-004 | `/bug` | `packages/cli/src/ui/commands/bugCommand.ts` | P1 | Not Started |
| CMD-005 | `/chat` + 子命令 | `packages/cli/src/ui/commands/chatCommand.ts` | P0 | Not Started |
| CMD-006 | `/clear` | `packages/cli/src/ui/commands/clearCommand.ts` | P1 | Not Started |
| CMD-007 | `/commands` | `packages/cli/src/ui/commands/commandsCommand.ts` | P1 | Not Started |
| CMD-008 | `/compress` | `packages/cli/src/ui/commands/compressCommand.ts` | P1 | Not Started |
| CMD-009 | `/copy` | `packages/cli/src/ui/commands/copyCommand.ts` | P1 | Not Started |
| CMD-010 | `/directory` | `packages/cli/src/ui/commands/directoryCommand.tsx` | P1 | Not Started |
| CMD-011 | `/docs` | `packages/cli/src/ui/commands/docsCommand.ts` | P2 | Not Started |
| CMD-012 | `/editor` | `packages/cli/src/ui/commands/editorCommand.ts` | P2 | Not Started |
| CMD-013 | `/extensions` | `packages/cli/src/ui/commands/extensionsCommand.ts` | P0 | Not Started |
| CMD-014 | `/help` | `packages/cli/src/ui/commands/helpCommand.ts` | P0 | In Progress |
| CMD-015 | `/hooks` | `packages/cli/src/ui/commands/hooksCommand.ts` | P1 | Not Started |
| CMD-016 | `/ide` | `packages/cli/src/ui/commands/ideCommand.ts` | P1 | Not Started |
| CMD-017 | `/init` | `packages/cli/src/ui/commands/initCommand.ts` | P1 | Not Started |
| CMD-018 | `/mcp` | `packages/cli/src/ui/commands/mcpCommand.ts` | P0 | Not Started |
| CMD-019 | `/memory` | `packages/cli/src/ui/commands/memoryCommand.ts` | P0 | Not Started |
| CMD-020 | `/model` | `packages/cli/src/ui/commands/modelCommand.ts` | P1 | In Progress |
| CMD-021 | `/permissions` | `packages/cli/src/ui/commands/permissionsCommand.ts` | P1 | Not Started |
| CMD-022 | `/plan` | `packages/cli/src/ui/commands/planCommand.ts` | P1 | Not Started |
| CMD-023 | `/policies` | `packages/cli/src/ui/commands/policiesCommand.ts` | P1 | Not Started |
| CMD-024 | `/privacy` | `packages/cli/src/ui/commands/privacyCommand.ts` | P2 | Not Started |
| CMD-025 | `/quit` | `packages/cli/src/ui/commands/quitCommand.ts` | P0 | Not Started |
| CMD-026 | `/restore` | `packages/cli/src/ui/commands/restoreCommand.ts` | P1 | Not Started |
| CMD-027 | `/rewind` | `packages/cli/src/ui/commands/rewindCommand.tsx` | P1 | Not Started |
| CMD-028 | `/resume` | `packages/cli/src/ui/commands/resumeCommand.ts` | P1 | Not Started |
| CMD-029 | `/settings` | `packages/cli/src/ui/commands/settingsCommand.ts` | P1 | Not Started |
| CMD-030 | `/shells` | `packages/cli/src/ui/commands/shellsCommand.ts` | P1 | Not Started |
| CMD-031 | `/setup-github` | `packages/cli/src/ui/commands/setupGithubCommand.ts` | P2 | Not Started |
| CMD-032 | `/skills` | `packages/cli/src/ui/commands/skillsCommand.ts` | P0 | Not Started |
| CMD-033 | `/stats` | `packages/cli/src/ui/commands/statsCommand.ts` | P1 | Not Started |
| CMD-034 | `/terminal-setup` | `packages/cli/src/ui/commands/terminalSetupCommand.ts` | P2 | Not Started |
| CMD-035 | `/theme` | `packages/cli/src/ui/commands/themeCommand.ts` | P2 | Not Started |
| CMD-036 | `/tools` | `packages/cli/src/ui/commands/toolsCommand.ts` | P1 | In Progress |
| CMD-037 | `/vim` | `packages/cli/src/ui/commands/vimCommand.ts` | P2 | Not Started |
| CMD-038 | `@path` 注入 | `docs/reference/commands.md` | P0 | In Progress |
| CMD-039 | `!command` 执行 | `docs/reference/commands.md` | P0 | In Progress |
| CMD-040 | `!` shell mode 切换 | `docs/reference/commands.md` | P1 | Not Started |
| CMD-041 | 条件命令 `/agents` | `BuiltinCommandLoader.ts` | P2 | Not Started |
| CMD-042 | 条件命令 `/oncall` | `BuiltinCommandLoader.ts` | P2 | Not Started |
| CMD-043 | 条件命令 `/profile` | `BuiltinCommandLoader.ts` | P2 | Not Started |
| CMD-044 | 内部命令 `/corgi` | `BuiltinCommandLoader.ts` | P2 | Not Started |

## C. 内置 Tool 覆盖

| ID | Tool | 基线参考 | 优先级 | 状态 |
| --- | --- | --- | --- | --- |
| TOOL-001 | `glob` | `packages/core/src/tools/glob.ts` | P0 | In Progress |
| TOOL-002 | `grep_search` | `packages/core/src/tools/grep.ts` | P0 | In Progress |
| TOOL-003 | `list_directory` | `packages/core/src/tools/ls.ts` | P0 | In Progress |
| TOOL-004 | `read_file` | `packages/core/src/tools/read-file.ts` | P0 | In Progress |
| TOOL-005 | `read_many_files` | `packages/core/src/tools/read-many-files.ts` | P0 | Not Started |
| TOOL-006 | `write_file` | `packages/core/src/tools/write-file.ts` | P0 | Not Started |
| TOOL-007 | `replace` | `packages/core/src/tools/edit.ts` | P0 | Not Started |
| TOOL-008 | `run_shell_command` | `packages/core/src/tools/shell.ts` | P0 | Not Started |
| TOOL-009 | `google_web_search` | `packages/core/src/tools/web-search.ts` | P1 | Not Started |
| TOOL-010 | `web_fetch` | `packages/core/src/tools/web-fetch.ts` | P1 | Not Started |
| TOOL-011 | `write_todos` | `packages/core/src/tools/write-todos.ts` | P1 | Not Started |
| TOOL-012 | `save_memory` | `packages/core/src/tools/memoryTool.ts` | P0 | Not Started |
| TOOL-013 | `get_internal_docs` | `packages/core/src/tools/get-internal-docs.ts` | P1 | Not Started |
| TOOL-014 | `activate_skill` | `packages/core/src/tools/activate-skill.ts` | P0 | Not Started |
| TOOL-015 | `ask_user` | `packages/core/src/tools/ask-user.ts` | P0 | Not Started |
| TOOL-016 | `enter_plan_mode` | `packages/core/src/tools/enter-plan-mode.ts` | P1 | Not Started |
| TOOL-017 | `exit_plan_mode` | `packages/core/src/tools/exit-plan-mode.ts` | P1 | Not Started |
| TOOL-018 | ToolRegistry（内置+发现+MCP） | `packages/core/src/tools/tool-registry.ts` | P0 | In Progress |
| TOOL-019 | Tool 确认流（allow/deny/ask） | `tools/*` + `policy/*` | P0 | Not Started |
| TOOL-020 | Tool 输出格式（text/json） | `output/*` | P1 | Not Started |

## D. 配置、安全、策略

| ID | 能力 | 基线参考 | 优先级 | 状态 |
| --- | --- | --- | --- | --- |
| CFG-001 | settings 多层加载顺序 | `docs/reference/configuration.md` | P0 | Not Started |
| CFG-002 | schema 默认值与校验 | `schemas/settings.schema.json` | P0 | Not Started |
| CFG-003 | env 变量展开 | `packages/cli/src/config/settings.ts` | P1 | Not Started |
| CFG-004 | trusted folders | `packages/cli/src/config/trustedFolders.ts` | P0 | Not Started |
| CFG-005 | approval mode（default/auto_edit/plan/yolo） | `packages/cli/src/config/config.ts` | P0 | In Progress |
| CFG-006 | policy paths 加载 | `docs/reference/policy-engine.md` | P1 | Not Started |
| CFG-007 | Policy Tier & Priority | `docs/reference/policy-engine.md` | P0 | Not Started |
| CFG-008 | argsPattern / commandPrefix 匹配 | `docs/reference/policy-engine.md` | P1 | Not Started |
| CFG-009 | 模式化策略（default/plan/yolo） | `docs/reference/policy-engine.md` | P1 | Not Started |
| CFG-010 | sandbox none/docker/podman | `docs/cli/sandbox.md` | P0 | Not Started |
| CFG-011 | macOS seatbelt 支持 | `docs/cli/sandbox.md` | P2 | Not Started |
| CFG-012 | 管理员限制（admin controls） | `config + docs/cli/enterprise.md` | P1 | Not Started |
| CFG-013 | telemetry 配置合并 | `packages/core/src/telemetry/*` | P1 | Not Started |
| CFG-014 | 安全输出（raw output 风险控制） | `packages/cli/src/config/config.ts` | P1 | Not Started |

## E. MCP / Extensions / Skills / Hooks

| ID | 能力 | 基线参考 | 优先级 | 状态 |
| --- | --- | --- | --- | --- |
| EXT-001 | MCP 配置存储与加载 | `packages/cli/src/commands/mcp/*.ts` | P0 | Not Started |
| EXT-002 | MCP add/remove/list | `packages/cli/src/commands/mcp/*.ts` | P0 | Not Started |
| EXT-003 | MCP enable/disable | `packages/cli/src/commands/mcp/enableDisable.ts` | P1 | Not Started |
| EXT-004 | MCP OAuth 流程 | `packages/core/src/mcp/*` | P1 | Not Started |
| EXT-005 | MCP tool discover | `packages/core/src/tools/mcp-tool.ts` | P0 | Not Started |
| EXT-006 | Extensions install/link/uninstall | `packages/cli/src/commands/extensions/*.ts` | P0 | Not Started |
| EXT-007 | Extensions enable/disable/update | `packages/cli/src/commands/extensions/*.ts` | P1 | Not Started |
| EXT-008 | Extensions 配置注入 | `packages/cli/src/config/extension-manager.ts` | P1 | Not Started |
| EXT-009 | Skills list/install/link/uninstall | `packages/cli/src/commands/skills/*.ts` | P0 | Not Started |
| EXT-010 | Skills enable/disable/reload | `packages/cli/src/ui/commands/skillsCommand.ts` | P0 | Not Started |
| EXT-011 | Hooks 注册与执行 | `packages/core/src/hooks/*` | P1 | Not Started |
| EXT-012 | hooks CLI 管理 | `packages/cli/src/ui/commands/hooksCommand.ts` | P1 | Not Started |

## F. 会话与恢复能力

| ID | 能力 | 基线参考 | 优先级 | 状态 |
| --- | --- | --- | --- | --- |
| SES-001 | 会话自动保存 | `packages/core/src/services/chatRecordingService.ts` | P1 | Not Started |
| SES-002 | 会话恢复 `resume` | `ui/commands/resumeCommand.ts` | P1 | Not Started |
| SES-003 | `/chat save/list/resume/delete` | `ui/commands/chatCommand.ts` | P1 | Not Started |
| SES-004 | checkpointing | `docs/cli/checkpointing.md` | P1 | Not Started |
| SES-005 | `/restore` 回滚 | `ui/commands/restoreCommand.ts` | P1 | Not Started |
| SES-006 | `/rewind` 时间线回退 | `ui/commands/rewindCommand.tsx` | P1 | Not Started |
| SES-007 | session retention 清理策略 | `settings schema + config` | P2 | Not Started |

## G. 生态能力（必须纳入 100%）

| ID | 能力 | 基线参考 | 优先级 | 状态 |
| --- | --- | --- | --- | --- |
| ECO-001 | SDK 核心 Agent API 对齐 | `packages/sdk/src/*` | P1 | Not Started |
| ECO-002 | SDK 流式输出接口对齐 | `packages/sdk/README.md` | P1 | Not Started |
| ECO-003 | A2A server 启动与路由 | `packages/a2a-server/src/*` | P1 | Not Started |
| ECO-004 | A2A server 配置与存储 | `packages/a2a-server/src/*` | P2 | Not Started |
| ECO-005 | VSCode IDE Companion 命令对齐 | `packages/vscode-ide-companion/src/*` | P1 | Not Started |
| ECO-006 | VSCode diff accept/cancel 流程 | `packages/vscode-ide-companion/package.json` | P2 | Not Started |

## H. 测试与发布门禁

| ID | 能力 | 基线参考 | 优先级 | 状态 |
| --- | --- | --- | --- | --- |
| QA-001 | Unit 测试框架（Rust） | `packages/*/*.test.ts` | P0 | In Progress |
| QA-002 | Integration 测试框架（Rust） | `integration-tests/*` | P0 | Not Started |
| QA-003 | JSON 输出回归测试 | `integration-tests/json-output.test.ts` | P0 | Not Started |
| QA-004 | 文件系统工具链回归测试 | `integration-tests/file-system*.test.ts` | P0 | Not Started |
| QA-005 | shell 工具链回归测试 | `integration-tests/run_shell_command.test.ts` | P0 | Not Started |
| QA-006 | MCP 回归测试 | `integration-tests/simple-mcp-server.test.ts` | P1 | Not Started |
| QA-007 | Hooks 回归测试 | `integration-tests/hooks-system.test.ts` | P1 | Not Started |
| QA-008 | plan mode 回归测试 | `integration-tests/plan-mode.test.ts` | P1 | Not Started |
| QA-009 | sandbox 矩阵测试 | `docs/integration-tests.md` | P1 | Not Started |
| QA-010 | 发布工件与版本流程 | `docs/releases.md` | P1 | Not Started |

## I. 追踪规则

1. 每次提交必须至少将 1 个矩阵项从 `Not Started` 推进到 `In Progress` 或 `Done`。
2. 标记 `Done` 的项必须附带对应测试提交（同提交或后续紧邻提交）。
3. 每周汇总更新本文件，记录本周完成项与阻塞项。
