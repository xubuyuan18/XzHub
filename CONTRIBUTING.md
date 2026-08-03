# 为 XzHub 做贡献

感谢你愿意参与 XzHub。

## 开始之前

- 对较大的功能，请先创建 Issue 描述使用场景和方案。
- Bug 修复可以直接提交 Pull Request，但应说明复现方式。
- 安全漏洞不要创建公开 Issue，请按照 `SECURITY.md` 报告。

## 开发原则

1. 保持 XzHub 轻量、稳定并以 Windows 为优先。
2. 不在没有真实需求的情况下增加抽象层。
3. 不把后台轮询和敏感逻辑放进 React。
4. 不在日志、测试数据或截图中提交真实凭据。
5. 新依赖必须说明用途，并优先选择维护活跃、许可证兼容的项目。

## 提交流程

1. Fork 仓库并创建功能分支。
2. 完成改动并添加必要测试。
3. 运行格式化、Lint、类型检查和测试。
4. 更新受影响的文档。
5. 提交 Pull Request。

建议分支命名：

```text
feat/http-provider
fix/tray-position
refactor/provider-registry
docs/provider-guide
```

## Commit 规范

使用 Conventional Commits：

```text
feat: add generic HTTP provider
fix: hide window when focus is lost
docs: document provider error model
test: cover scheduler retry behavior
```

## Pull Request 要求

PR 描述应包含：

- 改了什么。
- 为什么需要修改。
- 如何验证。
- 是否涉及界面变化。
- 是否涉及配置、数据库迁移或安全行为变化。

请保持 PR 尺寸适中。不要把无关格式化、重构和功能开发混在同一个 PR 中。

## AI 生成代码

允许使用 AI Coding 工具，但提交者需要对代码负责：

- 必须实际运行和验证生成代码。
- 不得提交无法解释的依赖或大段无用抽象。
- 不得伪造测试结果。
- 不得把提示词中的密钥、服务器地址或个人数据提交到仓库。
- AI 生成内容同样必须符合许可证和第三方版权要求。

## 许可证

提交代码即表示你同意贡献内容按照仓库的 MIT License 发布。
