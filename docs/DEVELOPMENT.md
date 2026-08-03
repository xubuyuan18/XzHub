# XzHub 开发指南

## 1. 目标环境

XzHub 首先面向 Windows 10 和 Windows 11。建议在 Windows 11 上完成主要开发和验收。

## 2. 计划技术栈

- Tauri 2
- Rust stable
- Tokio
- React
- TypeScript
- Vite
- Tailwind CSS
- shadcn/ui
- Zustand
- TanStack Query
- SQLite + SQLx
- Apache ECharts

在项目初始化前，新增依赖必须说明用途，不要一次性引入完整生态。

## 3. 开发环境准备

建议安装：

1. Git。
2. Node.js 当前 LTS 版本。
3. pnpm。
4. Rust stable 与 Cargo。
5. Visual Studio Build Tools，包含 Desktop development with C++。
6. WebView2 Runtime。

具体版本在工程初始化后通过版本文件锁定。

## 4. 计划中的常用命令

以下命令将在项目初始化后生效：

```bash
pnpm install
pnpm tauri dev
pnpm lint
pnpm typecheck
pnpm test
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

AI Coding 工具不得声称命令执行成功，除非实际运行并检查退出码。

## 5. 分支与提交

建议使用短生命周期分支：

```text
feat/tray-window
feat/http-provider
fix/window-position
chore/update-dependencies
```

提交信息采用 Conventional Commits：

```text
feat: add tray window toggle
fix: prevent scheduler overlap
refactor: split provider registry
chore: configure rustfmt
```

每个提交应只包含一个可解释的改动主题。

## 6. 模块边界

### 前端负责

- 页面和组件渲染。
- 用户交互与表单校验。
- 展示缓存数据。
- 调用 Tauri commands。

### Rust 负责

- 托盘和窗口生命周期。
- 后台调度。
- 网络请求。
- 数据库与迁移。
- 凭据存储。
- 通知。
- Provider 注册和执行。

不要把关键轮询逻辑放在 React 定时器中。

## 7. 错误处理

- Rust 库代码避免使用 `unwrap()` 和 `expect()`。
- 用户可恢复错误应转化为结构化错误。
- 后台任务错误写入日志并更新服务状态，不应导致进程退出。
- 前端不得展示原始堆栈或包含敏感数据的错误对象。

建议统一错误响应：

```ts
interface AppError {
  code: string
  message: string
  details?: Record<string, unknown>
}
```

## 8. 日志规范

- 使用结构化日志。
- 默认日志级别为 `info`。
- 网络请求仅记录目标主机、耗时、状态码和错误类别。
- Authorization、Cookie、Token、密码必须脱敏。
- 日志文件应滚动保存并限制总大小。

## 9. 测试策略

优先测试容易出错的业务边界：

- 服务状态映射。
- 用量和阈值计算。
- JSON 字段提取。
- 调度器防重入。
- 数据库迁移。
- 凭据引用与配置序列化。
- 不同任务栏方向下的窗口定位算法。

托盘和窗口交互需要在真实 Windows 环境进行手动验收。

## 10. Pull Request 验收

提交 PR 前至少确认：

- 格式化、Lint 和类型检查通过。
- 新行为包含测试或说明无法自动测试的原因。
- 未提交密钥、Token、Cookie、数据库文件或个人服务器地址。
- README 或 docs 在行为发生变化时同步更新。
- 截图中没有泄露个人信息。
