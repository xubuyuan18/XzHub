# XzHub Provider 规范

## 1. 目的

Provider 用于把不同服务的数据转换为 XzHub 可以统一展示的状态快照。界面层不应理解特定平台的原始响应格式。

## 2. 核心原则

- 每个 Provider 只负责一种协议或服务类型。
- Provider 不直接操作 UI。
- Provider 不直接决定通知展示方式。
- Provider 必须设置超时。
- Provider 不得在日志中输出敏感凭据。
- Provider 失败时返回结构化错误，不得导致调度器崩溃。

## 3. 建议接口

实际接口可在编码阶段调整，但语义应保持稳定：

```rust
#[async_trait]
pub trait ServiceProvider: Send + Sync {
    fn kind(&self) -> &'static str;

    async fn fetch(
        &self,
        context: &ProviderContext,
        config: &ServiceConfig,
    ) -> Result<ServiceSnapshot, ProviderError>;
}
```

## 4. 输入模型

```rust
pub struct ProviderContext {
    pub http_client: reqwest::Client,
    pub credential_store: CredentialStore,
}

pub struct ServiceConfig {
    pub id: String,
    pub name: String,
    pub provider_type: String,
    pub config: serde_json::Value,
    pub credential_refs: Vec<String>,
    pub timeout_seconds: u64,
}
```

Provider 配置允许使用 JSON 保存可扩展字段，但必须通过类型化结构反序列化后再使用。

## 5. 输出模型

```rust
pub enum ServiceState {
    Online,
    Warning,
    Offline,
    Unknown,
}

pub struct ServiceSnapshot {
    pub service_id: String,
    pub state: ServiceState,
    pub title: String,
    pub subtitle: Option<String>,
    pub message: Option<String>,
    pub usage: Option<UsageMetric>,
    pub metrics: Vec<Metric>,
    pub checked_at: chrono::DateTime<chrono::Utc>,
    pub duration_ms: u64,
}

pub struct UsageMetric {
    pub used: f64,
    pub total: f64,
    pub unit: String,
}

pub struct Metric {
    pub key: String,
    pub label: String,
    pub value: f64,
    pub unit: String,
}
```

## 6. 错误模型

```rust
pub enum ProviderErrorKind {
    InvalidConfig,
    CredentialMissing,
    Timeout,
    Network,
    Authentication,
    InvalidResponse,
    RateLimited,
    Internal,
}

pub struct ProviderError {
    pub kind: ProviderErrorKind,
    pub user_message: String,
    pub technical_message: Option<String>,
    pub retryable: bool,
}
```

`technical_message` 必须经过脱敏后才能写入日志。

## 7. HTTP Provider

最小配置：

- URL。
- 请求方法。
- 期望状态码范围。
- 超时。
- 可选请求头。

行为：

- 状态码匹配时为 `online`。
- 可访问但状态码不匹配时通常为 `offline`。
- 超时或连接失败时为 `offline`。
- 配置错误时为 `unknown`。

## 8. 通用 JSON Provider

支持从 JSON 响应提取：

- 状态。
- 已使用量。
- 总量。
- 到期时间。
- 自定义数值指标。

第一版建议实现简单点分路径，例如：

```text
data.usage.used
data.usage.total
data.expire_at
```

数组和复杂表达式应等基础功能稳定后再增加。

## 9. VPS Agent Provider

建议 Agent 端返回版本化响应：

```json
{
  "schema_version": 1,
  "hostname": "example-vps",
  "cpu_percent": 23.5,
  "memory_used_bytes": 1908408320,
  "memory_total_bytes": 4294967296,
  "disk_used_bytes": 40802189312,
  "disk_total_bytes": 85899345920,
  "load_average": [0.23, 0.31, 0.27],
  "uptime_seconds": 189234,
  "collected_at": "2026-08-03T08:00:00Z"
}
```

客户端必须验证 `schema_version`，不能静默接受未知结构。

## 10. Provider 注册

Provider 通过注册表按 `provider_type` 查找：

```rust
registry.register(HttpProvider::new());
registry.register(JsonProvider::new());
registry.register(VpsAgentProvider::new());
```

不要在调度器中编写大量 `match` 分支处理平台细节。

## 11. 新 Provider 验收清单

- 配置结构有明确类型和校验。
- 成功、超时、鉴权失败和非法响应均有测试。
- 所有网络请求有超时。
- 错误信息经过脱敏。
- Provider 不依赖 UI。
- Provider 输出统一的 `ServiceSnapshot`。
- 文档包含配置示例。
