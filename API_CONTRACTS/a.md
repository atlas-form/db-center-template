基本对：**请求体大体沿用/贴近公开 Responses API 的结构，但上游地址和请求头换成了 ChatGPT/Codex 内部通道。**

不过也不是“只有头和地址不同”这么绝对。这个项目还会做一些适配：

- 客户端如果打 `/v1/images/generations`，项目会改写成 `/backend-api/codex/responses` 的 `image_generation` tool 请求。
- OAuth 图片请求里，顶层 `model` 固定用 `gpt-5.4-mini`，图片模型放在 `tools[0].model = gpt-image-2`。
- `n > 1` 不会真正传给 tool，基本降级成单张。
- 上传图片会转成 `data:image/...;base64,...` 放进 `input_image.image_url`。
- 响应也会从 Codex Responses SSE 再转换回 OpenAI Images API 风格。

至于它怎么知道这个地址：

```text
POST https://chatgpt.com/backend-api/codex/responses
```

**官方 OpenAI Platform 文档没有公开这个地址。**  
从代码里的命名和注释看，它明确把它当作 “ChatGPT internal API for OAuth accounts”：

```go
chatgptCodexURL = "https://chatgpt.com/backend-api/codex/responses"
```

所以最合理的判断是：**这是通过观察官方 Codex CLI / ChatGPT 客户端流量、逆向/抓包/阅读客户端实现等方式得到的内部接口行为**，而不是来自公开 API 文档。

项目里也有线索支持这一点：

- OAuth client 叫 “Codex CLI official”。
- 上游 header 里设置 `originator`、`OpenAI-Beta: responses=experimental`、`chatgpt-account-id`。
- `User-Agent` 默认伪装成类似 `codex_cli_rs/0.125.0`。
- 代码专门区分 “OpenAI Platform API for API Key accounts” 和 “ChatGPT internal API for OAuth accounts”。

所以可以这样总结：

```text
API Key 模式 = 正规 OpenAI Platform API
OAuth 模式 = 模拟 Codex/ChatGPT 客户端调用 ChatGPT 内部接口
```

这类接口的风险是：**没有公开文档承诺，字段、header、风控、权限、返回事件都可能随 ChatGPT/Codex 客户端变化而变。**
