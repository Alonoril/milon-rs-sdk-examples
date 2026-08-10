# Milon Rust SDK 使用示例

本仓库是 Milon Rust SDK 的可运行示例集合，覆盖账户、密钥、钱包、交易、Token、IDL、Multicall、多签和 BLS 等常见场景。

完整实现都在 [`examples/`](examples/) 中。本文档帮助你理解 SDK 的基本组成、选择合适的示例，并给出可以直接复制执行的命令。

> 警告：部分示例会连接 RPC、领取 faucet、模拟交易或真实发送交易。请先确认 RPC、账户和 signer 配置；不要在生产环境使用示例中的 seed、助记词或公网 RPC。

## 目录

- [1. 环境要求](#1-环境要求)
- [2. 构建和运行](#2-构建和运行)
- [3. SDK 核心概念](#3-sdk-核心概念)
- [4. 快速开始](#4-快速开始)
- [5. 密钥、公钥和 HD 钱包](#5-密钥公钥和-hd-钱包)
- [6. 交易完整流程](#6-交易完整流程)
- [7. Token](#7-token)
- [8. IDL 和自定义类型](#8-idl-和自定义类型)
- [9. Multicall](#9-multicall)
- [10. 多签、投票和 BLS](#10-多签投票和-bls)
- [11. 示例索引](#11-示例索引)
- [12. 常见问题和安全建议](#12-常见问题和安全建议)

## 1. 环境要求

### 1.1 Rust 工具链

项目使用 Rust 2024 edition，并在 [`rust-toolchain.toml`](rust-toolchain.toml) 中固定工具链。进入仓库后检查工具链：

```bash
rustup show
rustc --version
cargo --version
```

### 1.2 目录依赖

`Cargo.toml` 使用本地 path dependency，因此需要保持下面的目录关系：

```text
milon-labs/
├── Milon/
├── milon-rust-sdk/
│   ├── milon-sdk-rs/
│   └── only-sdk-examples/
```

示例仓库依赖：

- `../milon-sdk-rs/{idl-core,idl-macro,local-wallet,provider,primitives,rpc-client,transport,milon-client}`
- `../../Milon/crates/{wallet,crypto}`

如果只复制 `only-sdk-examples`，这些路径会失效。请在上述工作区结构中运行命令。

### 1.3 RPC 和 Chain ID

多数网络示例默认使用：

```text
RPC:      http://47.84.39.153:6280/milon/v1
Chain ID: 900000001
```

部分模拟交易示例默认使用本地 RPC `http://127.0.0.1:6380/milon/v1`。建议显式设置 RPC，而不要依赖示例中的默认值：

```bash
export MILON_RPC_URL='http://127.0.0.1:6280/milon/v1'
```

示例实际读取的环境变量如下：

| 变量 | 使用示例 | 作用 |
| --- | --- | --- |
| `MILON_RPC_URL` | Provider、Multicall、wallet 示例 | 覆盖 RPC 地址。 |
| `MILON_ACCOUNT_METHOD` | `exam_account_provider` | 选择账户 Provider 操作，例如 `create`。 |
| `MILON_ACCOUNT_SIGNER_SEED` | `exam_account_provider` | 设置账户示例的演示 seed。 |
| `MILON_TX_HASH_BS58` | `get_tx_receipt_decode_example` | 以 Base58 传入交易 hash。 |
| `MILON_TX_HASH_HEX` | `get_tx_receipt_decode_example` | 以 hex 传入交易 hash。 |
| `MILON_TOKEN_ADDRESS` | `muticall_mixed_result_example` | 覆盖查询用 Token 地址。 |
| `MILON_ACCOUNT_ADDRESS` | `muticall_mixed_result_example` | 覆盖查询用账户地址。 |
| `MILON_VALIDATOR_ADDRESS` | `muticall_mixed_result_example` | 覆盖查询用 Validator 地址。 |
| `MILON_SUBMIT_TX_JSON` | 两个 simulate/submit 示例 | 直接传入 JSON 交易配置。 |
| `MILON_SUBMIT_TX_JSON_FILE` | 两个 simulate/submit 示例 | 从文件传入 JSON 交易配置。 |

## 2. 构建和运行

### 2.1 检查全部示例

```bash
cargo check --examples
```

构建全部 workspace target：

```bash
cargo build --workspace
```

### 2.2 运行单个示例

Cargo 使用 Rust 文件名作为 example 名称。例如：

```bash
cargo run --example get_local_signer_public_key
cargo run --example public_key_demo
cargo run --example hd_phrase_generation
```

网络示例通常是异步程序：

```bash
MILON_RPC_URL='http://127.0.0.1:6280/milon/v1' \
  cargo run --example wallet_provider_example
```

### 2.3 读示例源码

推荐的学习方式是先运行本地计算示例，再阅读对应网络示例：

1. `get_local_signer_public_key`：生成 signer 并查看公钥/地址。
2. `hd_phrase_generation`、`hd_phrase_to_wallet`：了解助记词和 HD 钱包。
3. `exam_account_provider`：了解 Provider 和账户 API。
4. `wallet_provider_example`：了解填充、模拟、发送和回执解码。
5. `app_test_token_provider`、`idl_app_demo_example`：了解业务指令。

## 3. SDK 核心概念

| 类型 | 作用 |
| --- | --- |
| `Provider` | 通过 RPC 读取链上数据、模拟交易和提交交易。 |
| `LocalSigner` / `Signer` | 持有私钥并提供签名能力。 |
| `LocalWallet` | 管理一个或多个 signer，并为交易选择签名者。 |
| `WalletFiller` | 把 wallet 接入 Provider 的交易填充流程。 |
| `Address` | 链上账户、Token、池等资源的地址。 |
| `PackedInstruction` | 已按 Milon IDL 编码的单条指令。 |
| `TransactionRequest` | 尚未完全填充的交易请求。 |
| `Transaction` | 带有 chain ID、stamp、指令和签名的交易。 |
| `SigningPlan` | 描述 payer、账户授权和每条指令由谁签名。 |
| `Tokenizable` / `Detokenize` | 在 Rust 结构和值编码之间转换，常用于 IDL 数据。 |

一个典型的 SDK 调用链是：

```text
RPC URL
  └─ Provider
      ├─ 读取链上数据
      ├─ simulate_transaction
      └─ submit/send_transaction
          └─ Transaction
              ├─ PackedInstruction
              ├─ LocalWallet / SigningPlan
              └─ signatures
```

## 4. 快速开始

下面的模式与 [`examples/exam_account_provider.rs`](examples/exam_account_provider.rs) 和 [`examples/app_test_account_provider.rs`](examples/app_test_account_provider.rs) 一致。网络操作需要可用 RPC，并且会改变链上状态。

### 4.1 连接 Provider

仓库的公共辅助代码在 [`src/lib.rs`](src/lib.rs) 中：

```rust
use only_sdk_examples::DemoRpc;

let rpc = DemoRpc::connect("http://127.0.0.1:6280/milon/v1")?;
let provider = &rpc.provider;
```

如果你的应用不使用本仓库的辅助 crate，也可以参考 [`wallet_provider_example.rs`](examples/wallet_provider_example.rs) 中的 `build_provider`，自行创建 HTTP transport、RPC client 和 Provider。

### 4.2 创建 signer 和 wallet

示例中的 deterministic seed 只适用于演示，不应当用于真实资产：

```rust
use milon_local_wallet::{LocalWallet, Signer};
use only_sdk_examples::local_ed25519_signer;

let signer = local_ed25519_signer(2)?;
let address = signer.address();
let wallet = LocalWallet::new(signer);
```

将 wallet 接入 Provider：

```rust
use milon_client::WalletFiller;

let provider = rpc.provider.with_wallet_filler(WalletFiller::new(wallet));
```

### 4.3 Faucet、创建账户和查询账户

```rust
let faucet_result = provider
    .claim_faucet_with_cooldown_remaining()
    .await?;
println!("faucet result: {faucet_result:?}");

let tx_hash = provider.create_account().await?;
println!("create account tx_hash: {tx_hash}");

let account = provider.account(address).await?;
println!("account: {account:?}");
```

faucet 可能有冷却时间；交易提交成功也不代表已经完成，生产代码应轮询交易状态，而不是固定假设等待一秒即可完成。

## 5. 密钥、公钥和 HD 钱包

### 5.1 公钥和地址

运行：

```bash
cargo run --example get_local_signer_public_key
cargo run --example public_key_demo
```

`get_local_signer_public_key` 展示 signer 的 bytes、hex、Base58 公钥和地址；`public_key_demo` 展示公钥解析。通常使用：

```rust
let public_key = signer.public_key();
println!("public key: {}", public_key.to_bs58());
println!("address: {}", signer.address());
```

公钥可以公开保存；私钥、seed 和助记词必须视为秘密材料。

### 5.2 助记词和 HD wallet

```bash
cargo run --example hd_phrase_generation
cargo run --example hd_phrase_to_wallet
cargo run --example hd_mnemonic_example
cargo run --example hd_wallet_example
```

这些示例分别覆盖助记词生成、助记词转 wallet、从固定 mnemonic 派生账户和 HD wallet 路径。使用自己的助记词时不要把命令行历史、终端输出、日志或源代码提交到 Git。

## 6. 交易完整流程

一次交易通常包括以下步骤：

1. 选择 chain ID 和未来有效的 stamp。
2. 构造一个或多个 IDL 指令并 `pack()`。
3. 创建 `Transaction` 或 `TransactionRequest`。
4. 用 wallet 和必要的 `SigningPlan` 签名。
5. 先模拟交易，检查执行结果和解码后的指令。
6. 提交交易，保存 tx hash。
7. 轮询交易，最后解码 receipt/history。

### 6.1 手工创建并签名 Transaction

[`sign_transaction_example.rs`](examples/sign_transaction_example.rs) 展示了手工组装账户创建交易：

```rust
let instructions = vec![claim_faucet(account)?, create_account(public_key)?];
let mut transaction = Transaction::new_with_stamp(
    chain_id,
    stamp,
    Some(account),
    instructions,
);
wallet.sign_transaction(&mut transaction)?;

let tx_hash = provider.submit_transaction(transaction).await?;
```

### 6.2 模拟后发送

[`simulate_and_send_trans.rs`](examples/simulate_and_send_trans.rs) 和 [`simulate_send_transfer_decode.rs`](examples/simulate_send_transfer_decode.rs) 展示：

```rust
let response = provider.simulate_transaction(transaction.clone()).await?;
let receipt = milon_client::decode_transaction_response(&response)?;
print_simulate_receipt(&receipt);

let tx_hash = provider.submit_transaction(transaction).await?;
```

模拟只执行检查，不等于写入链；确认结果后再提交相同交易。

### 6.3 WalletFiller 自动填充

[`wallet_provider_example.rs`](examples/wallet_provider_example.rs) 使用 `TransactionRequest` 和 Provider filler：

```rust
let sendable = provider.fill(request.clone()).await?;
let tx_hash = provider.send_transaction(request).await?;
```

填充过程会依次补齐 chain ID、stamp 和 wallet 签名。调试时可以先对 `fill` 的结果打印本地 tx hash，再执行发送。

### 6.4 多 signer 的 SigningPlan

当一笔交易中的不同指令由不同账户授权时，需要注册所有 signer，并明确每个账户负责哪些指令：

```rust
let mut wallet = LocalWallet::new(signer_a);
wallet.register_signer(signer_b)?;

let plan = SigningPlan::new(account_a)
    .authorize(AccountAuthorization::new(account_a, vec![0, 1]).with_payer())
    .authorize(AccountAuthorization::new(account_b, vec![2]));

wallet.sign_transaction_with_plan(&mut transaction, &plan)?;
```

指令索引从 0 开始。链上要求某个 signer 时，本地 wallet 必须同时拥有对应 signer，并且 `SigningPlan` 必须覆盖该指令。

### 6.5 查询并解码回执

```bash
cargo run --example get_tx_receipt_decode_example
```

该示例支持：

```bash
MILON_TX_HASH_BS58='交易 hash' \
  MILON_RPC_URL='http://127.0.0.1:6280/milon/v1' \
  cargo run --example get_tx_receipt_decode_example
```

也支持 `MILON_TX_HASH_HEX`。发送后应保存 tx hash，并使用 RPC 查询原始 bytes，再调用 `decode_transaction_history` 或 `decode_transaction_response` 解码。

### 6.6 JSON 输入

两个交易示例支持通过环境变量传入 JSON：

```bash
export MILON_SUBMIT_TX_JSON='{"rpc_url":"http://127.0.0.1:6380/milon/v1","chain_id":900000001}'
cargo run --example simulate_and_send_trans
```

或者：

```bash
export MILON_SUBMIT_TX_JSON_FILE=/path/to/submit.json
cargo run --example simulate_send_transfer_decode
```

## 7. Token

### 7.1 Token Provider

[`app_test_token_provider.rs`](examples/app_test_token_provider.rs) 集中展示 Token API。运行：

```bash
cargo run --example app_test_token_provider
```

源码中包含以下类别的调用，可按需取消注释对应函数：

- `balance_of`：查询账户余额。
- `create_token`：创建 Token。
- `get_metadata`：查询 Token 元数据。
- `mint`：铸造数量。
- `burn`：销毁数量。
- `transfer`：转账。
- `freeze` / `unfreeze`：冻结或解冻账户。
- `set_metadata`：更新元数据。

Token 写操作需要 signer、足够余额和正确的账户授权；不要直接运行包含写操作的默认入口，先阅读该文件的 `main`，确认要调用的函数。

### 7.2 Token 指令和 IDL

也可以直接构造 Token IDL 指令：

```rust
let instruction = token::Transfer {
    from: InstructionSigner::new(from),
    token: token_address,
    to,
    amount: 1,
}
.pack()?;
```

然后将 `PackedInstruction` 放入 `Transaction`。完整的多账户转账和模拟示例见 [`simulate_send_transfer_decode.rs`](examples/simulate_send_transfer_decode.rs)。

## 8. IDL 和自定义类型

### 8.1 构造 IDL 指令

[`idl_app_demo_example.rs`](examples/idl_app_demo_example.rs) 展示 `demo::InitPool`、`demo::BatchCredit` 等指令的构造、`pack()` 和交易发送；[`idl_app_token_demo.rs`](examples/idl_app_token_demo.rs) 展示 IDL Token 场景。

```rust
let instruction = demo::InitPool {
    pool: Signer::new(pool),
    label: "demo pool".to_owned(),
}
.pack()?;
```

`pack()` 只负责按 IDL 编码指令，不负责签名或发送。

### 8.2 Tokenizable 和 Detokenize

```bash
cargo run --example idl_tokenizable_example
cargo run --example idl_tokenizable_account_profile
```

这两个示例覆盖基础类型、结构体、嵌套结构和自定义 `Tokenizable` 实现。需要和链上 IDL 交互时，应保证字段顺序、字段名称和类型标签与链上定义一致。

### 8.3 IDL 错误码

```bash
cargo run --example idl_error_code
```

该示例展示应用 ID 与错误索引之间的编码和解析。处理 RPC 错误时，保留原始 code 和 data，便于根据 IDL 定位具体业务错误。

## 9. Multicall

### 9.1 同类型批量查询

```bash
cargo run --example muticall_balance_of_example
```

该示例批量调用多个 `balance_of`，并将每个结果解码成 `ViewResult<u64>`。它是只读查询，不需要 wallet 签名。

### 9.2 混合类型结果

```bash
MILON_RPC_URL='http://127.0.0.1:6280/milon/v1' \
  cargo run --example muticall_mixed_result_example
```

此示例展示不同 view 调用的结果如何按 `ViewResult<Token>` 处理。可覆盖的配置包括：

| 变量 | 作用 |
| --- | --- |
| `MILON_RPC_URL` | RPC 地址。 |
| `MILON_TOKEN_ADDRESS` | Token 地址。 |
| `MILON_ACCOUNT_ADDRESS` | 查询账户地址。 |
| `MILON_VALIDATOR_ADDRESS` | Validator 地址。 |

## 10. 多签、投票和 BLS

### 10.1 本地多签账户

[`app_test_account_provider.rs`](examples/app_test_account_provider.rs) 覆盖账户管理操作：

- 创建多签账户：`create_multisig`。
- 添加 signer：`add_signer` / `add_signers`。
- 修改阈值：`set_threshold`。
- 删除 signer：`remove_signer`。
- 修改权重：`set_signer_weight`。
- 查询 signer 列表：`list_signers`。

链上 threshold、signer index 和 weight 必须与本地 `MultisigSlot` 定义一致。修改 threshold 后，后续交易要按新的链上 threshold 重建 wallet。

### 10.2 链上投票聚合

```bash
cargo run --example app_account_multisig_on_chain
```

该示例把流程拆成：

1. 计算待执行指令的 intent hash。
2. 由不同投票 signer 调用 `vote_init` 和 `vote`。
3. 查询 `vote_info`，确认权重达到 threshold。
4. 使用 relayer/payer 提交 `send_voted_transaction`。

这是会真实写链的复杂示例，运行前应确认账户、投票 signer、relayer 和 RPC 状态。

### 10.3 BLS 聚合签名

```bash
cargo run --example bls_aggregator_example
```

该示例主要演示 BLS signer 和签名聚合的本地流程，不应把示例中的固定 seed 当作真实密钥。

## 11. 示例索引

下表覆盖当前 `examples/` 中的全部 Rust 示例。运行命令中的名称就是对应文件去掉 `.rs` 的 stem。

| 示例 | 内容 | 运行 |
| --- | --- | --- |
| `app_account_multisig_on_chain.rs` | 链上投票、多签意图和 voted transaction 提交 | `cargo run --example app_account_multisig_on_chain` |
| `app_test_account_provider.rs` | 账户创建、多签 signer/threshold/weight 管理 | `cargo run --example app_test_account_provider` |
| `app_test_token_provider.rs` | Token 查询、创建、铸造、转账、冻结等 Provider API | `cargo run --example app_test_token_provider` |
| `bls_aggregator_example.rs` | BLS signer 和聚合签名 | `cargo run --example bls_aggregator_example` |
| `exam_account_provider.rs` | 通过环境变量选择账户 Provider 操作 | `cargo run --example exam_account_provider` |
| `get_local_signer_public_key.rs` | 生成本地 signer，打印公钥和地址 | `cargo run --example get_local_signer_public_key` |
| `get_tx_receipt_decode_example.rs` | 查询 tx hash 并解码交易回执 | `cargo run --example get_tx_receipt_decode_example` |
| `hd_mnemonic_example.rs` | 从助记词派生多个 HD 账户 | `cargo run --example hd_mnemonic_example` |
| `hd_phrase_generation.rs` | 生成助记词 | `cargo run --example hd_phrase_generation` |
| `hd_phrase_to_wallet.rs` | 将助记词转换为 wallet | `cargo run --example hd_phrase_to_wallet` |
| `hd_wallet_example.rs` | HD wallet 基础用法 | `cargo run --example hd_wallet_example` |
| `idl_app_demo_example.rs` | Demo IDL 指令构造、发送和解码 | `cargo run --example idl_app_demo_example` |
| `idl_app_token_demo.rs` | IDL Token 应用示例 | `cargo run --example idl_app_token_demo` |
| `idl_error_code.rs` | IDL 错误码编码和解析 | `cargo run --example idl_error_code` |
| `idl_tokenizable_account_profile.rs` | 账户 profile 的 Tokenizable/Detokenize | `cargo run --example idl_tokenizable_account_profile` |
| `idl_tokenizable_example.rs` | 基础和嵌套类型 Tokenizable | `cargo run --example idl_tokenizable_example` |
| `muticall_balance_of_example.rs` | 批量查询多个余额 | `cargo run --example muticall_balance_of_example` |
| `muticall_mixed_result_example.rs` | 混合 view 调用结果解码 | `cargo run --example muticall_mixed_result_example` |
| `public_key_demo.rs` | 公钥解析和格式转换 | `cargo run --example public_key_demo` |
| `sign_transaction_example.rs` | 手工构造、签名和提交交易 | `cargo run --example sign_transaction_example` |
| `simulate_and_send_trans.rs` | 模拟、发送和交易历史解码 | `cargo run --example simulate_and_send_trans` |
| `simulate_send_transfer_decode.rs` | 多 signer transfer、模拟和回执解码 | `cargo run --example simulate_send_transfer_decode` |
| `wallet_provider_example.rs` | Provider filler、wallet 签名、模拟和发送 | `cargo run --example wallet_provider_example` |

`examples/request-his.md` 是历史运行记录和调试输出，不是 Cargo example target。

## 12. 常见问题和安全建议

### RPC 连接失败

检查 URL 是否包含完整路径 `/milon/v1`，并确认节点端口可访问。优先通过 `MILON_RPC_URL` 覆盖默认值：

```bash
MILON_RPC_URL='http://127.0.0.1:6280/milon/v1' \
  cargo run --example exam_account_provider
```

### faucet 返回 cooldown

`claim_faucet_with_cooldown_remaining` 可能返回剩余冷却时间。等待冷却结束后再尝试；不要在循环中高频请求 faucet。

### 缺少 signer 或授权

如果节点报告某条指令需要 signer：

1. 检查该账户是否注册到 `LocalWallet`。
2. 检查 `SigningPlan` 是否把该指令索引授权给正确账户。
3. 多签场景检查 index、weight、threshold 是否与 `list_signers` 返回值一致。

### stamp 过期

交易 stamp 必须落在节点接受的时间窗口内。参考 `src/lib.rs` 和模拟交易示例中的 `next_stamp()`，发送前重新生成 stamp，不要长期复用旧交易。

### 模拟成功但发送失败

模拟结果不保证提交时仍然有效。重新检查：余额、账户状态、stamp、signer、nonce/交易唯一性和链上状态变化。

### 私钥和助记词

- 不要使用示例中的固定 seed 管理真实资产。
- 不要把助记词、私钥、未加密 keystore 放进 Git、日志、issue 或聊天记录。
- 生产环境应使用隔离的密钥管理方案，并限制 signer 权限。
- 发送前打印交易内容和目标地址，避免把调试账户误用于生产。

### 公网 RPC

示例公网 RPC 只用于开发和测试，可能存在延迟、限流、不可用或网络配置变化。生产应用应使用经过认证、具备监控和超时/重试策略的 RPC 服务。

## 相关目录

- SDK 实现：[`../milon-sdk-rs`](../milon-sdk-rs)
- 示例代码：[`examples/`](examples/)
- 公共辅助代码：[`src/lib.rs`](src/lib.rs)
- Cargo 配置：[`Cargo.toml`](Cargo.toml)
