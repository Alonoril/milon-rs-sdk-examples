# Milon SDK Examples README Design

## Goal

在 `only-sdk-examples` 根目录增加一份中文 `README.md`，依据 `examples/` 中的真实代码，为首次使用 Milon Rust SDK 的开发者提供从环境准备到进阶交易能力的完整入门路径。

## Audience and scope

- 面向第一次使用本仓库和 Milon Rust SDK 的 Rust 开发者。
- 覆盖当前 `examples/` 中全部示例类别：环境与 Provider、账户、密钥与 HD 钱包、交易签名/模拟/发送/回执、Token、IDL、Multicall、多签、投票和 BLS。
- README 的完整实现以示例文件为准；文档中的代码只保留可帮助理解 API 的最小片段，并链接到对应示例。
- 不修改 SDK API、不新增依赖、不修改示例逻辑。

## Information architecture

1. 项目定位与目录说明。
2. 环境要求、构建与运行命令。
3. RPC 地址、Chain ID、交易 stamp、环境变量和测试网络注意事项。
4. 核心对象说明：Provider、Wallet、Signer、Address、PackedInstruction、Transaction、SigningPlan。
5. 快速开始：连接 RPC、创建本地 signer/wallet、领取 faucet、查询账户、构造并发送一笔交易。
6. 按学习路径介绍账户、密钥/HD 钱包、交易生命周期、Token、IDL、Multicall 和进阶多签/BLS。
7. 示例索引：每个 `examples/*.rs` 对应用途和运行命令。
8. 故障排查和安全建议：RPC 不可用、交易 stamp、faucet 冷却、签名者授权、私钥/助记词保护。

## Documentation conventions

- 所有面向用户的正文使用中文；Rust API 名称、类型名、文件名和命令保持原文。
- 运行命令使用 `cargo run --example <name>`，并明确仅本地计算、只读 RPC、模拟交易和真实发送交易的区别。
- 不把示例中硬编码的公网 RPC、seed、地址或交易 hash 描述为生产配置；对真实发送交易的示例加醒目提示。
- 环境变量只记录代码中实际读取的变量，包括 `MILON_RPC_URL`、`MILON_SUBMIT_TX_JSON`、`MILON_SUBMIT_TX_JSON_FILE` 及示例自身读取的配置。
- 文档必须与当前 `Cargo.toml`、`src/lib.rs` 和 `examples/` 的文件名保持一致。

## Acceptance criteria

- 根目录存在 `README.md`，包含上述章节和全部示例文件索引。
- README 中出现的每个运行命令都能映射到当前存在的 example target。
- 不引入失效链接、未定义环境变量或与示例实现矛盾的 API 描述。
- 通过 Markdown 链接和代码块检查，并运行 `cargo check --examples` 验证示例仍可编译。
