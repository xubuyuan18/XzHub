# XzHub 系统架构设计

## 目标

XzHub 是一个本地优先的 Windows 桌面服务状态面板，用于统一查看个人服务、API、VPS 和基础设施状态。

## 总体结构

```
Desktop UI
    |
Application Core
    |
Provider Runtime
    |
Storage Layer
    |
External Services / VPS Agent
```

## Desktop UI

职责：

- 系统托盘入口。
- 状态卡片展示。
- 设置管理。
- 历史趋势展示。

UI 不直接访问外部服务。

## Application Core

职责：

- 生命周期管理。
- 后台任务调度。
- Provider 管理。
- 状态聚合。
- 错误处理。

## Provider Runtime

所有数据源通过统一 Provider 接口接入。

支持方向：

- HTTP Health Check。
- JSON API。
- VPS Agent。
- Ping 检测。

## Storage Layer

本地 SQLite 保存：

- 服务配置。
- 用户设置。
- 有限历史快照。

敏感信息必须使用操作系统安全存储，不允许明文保存。

## VPS Agent

Agent 仅负责采集明确授权的系统指标：

- CPU。
- 内存。
- 磁盘。
- 负载。
- 运行时间。

禁止远程任意命令执行。

## MVP 原则

1. 稳定优先。
2. 本地优先。
3. 安全优先。
4. 保持模块可替换。
