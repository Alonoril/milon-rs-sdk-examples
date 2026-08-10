# Milon SDK Examples README Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 在 `only-sdk-examples` 根目录编写一份覆盖全部 SDK 示例的中文入门 README。

**Architecture:** 以学习路径为主线组织文档，以 `examples/*.rs` 为完整实现索引。README 只展示少量稳定 API 片段，并将复杂逻辑和可运行命令指向对应 example，减少文档与代码分叉。

**Tech Stack:** Markdown、Rust 2024、Cargo examples、Milon Rust SDK 本地 path dependencies。

## Global Constraints

- 所有用户可见正文使用中文；Rust API 名称、文件名和命令保持原文。
- 不修改 SDK API、不新增依赖、不修改示例逻辑。
- 不把硬编码公网 RPC、seed、地址或交易 hash 当作生产配置。
- 只记录代码实际读取的环境变量。
- README 中的运行命令必须映射到当前存在的 example target。

---

### Task 1: 建立 README 骨架与环境说明

**Files:**
- Create: `README.md`
- Reference: `Cargo.toml`, `rust-toolchain.toml`, `src/lib.rs`

**Interfaces:**
- Consumes: package name `only-sdk-examples`、Rust/toolchain 版本、默认 RPC 和 Chain ID。
- Produces: README 标题、项目定位、目录说明、环境准备、构建和运行入口。

- [ ] **Step 1: 写入文档标题、定位和快速开始章节**

  明确这是 Milon Rust SDK 的可运行示例集合，给出：

  ```bash
  rustup show
  cargo check --examples
  cargo run --example get_local_signer_public_key
  ```

  说明仓库依赖同级 `milon-rust-sdk` 和上级 `Milon` 的本地路径，不能仅复制目录后脱离原工作区构建。

- [ ] **Step 2: 补充 RPC、Chain ID 和环境变量说明**

  记录默认 Chain ID `900_000_001`、默认示例 RPC、`MILON_RPC_URL`、`MILON_SUBMIT_TX_JSON` 和 `MILON_SUBMIT_TX_JSON_FILE`，并说明公网 RPC、faucet 和真实发送交易的限制。

- [ ] **Step 3: 检查命令和文件引用**

  运行：

  ```bash
  rtk rg -n 'cargo run --example|examples/' README.md
  rtk git diff --check
  ```

  预期：命令和路径均为 Markdown 中的明确引用，没有空链接或空代码块。

### Task 2: 编写核心 API 和基础使用教程

**Files:**
- Modify: `README.md`
- Reference: `examples/exam_account_provider.rs`, `examples/get_local_signer_public_key.rs`, `examples/public_key_demo.rs`, `examples/hd_*.rs`, `examples/app_test_account_provider.rs`

**Interfaces:**
- Consumes: `DemoRpc::connect`、`local_ed25519_signer`、`LocalWallet`、`WalletFiller`、账户 Provider 扩展。
- Produces: Provider/Wallet/Signer/Address/Instruction/Transaction/SigningPlan 概念说明和账户入门示例。

- [ ] **Step 1: 增加核心对象说明**

  用表格简述 `Provider`、`LocalWallet`、`LocalSigner`、`Address`、`PackedInstruction`、`Transaction`、`TransactionRequest` 和 `SigningPlan` 的职责。

- [ ] **Step 2: 增加 signer、public key 和 HD 钱包教程**

  展示 `local_ed25519_signer(seed)`、`public_key().to_bs58()`、`address()` 的关系，并链接四个 HD 示例，强调助记词和私钥不得写入日志或提交仓库。

- [ ] **Step 3: 增加账户查询与创建教程**

  展示连接 Provider、绑定 Wallet、调用 `claim_faucet_with_cooldown_remaining`、`create_account` 和 `account`，明确该流程会写链。

### Task 3: 编写交易、Token 和 IDL 教程

**Files:**
- Modify: `README.md`
- Reference: `examples/sign_transaction_example.rs`, `examples/simulate_and_send_trans.rs`, `examples/simulate_send_transfer_decode.rs`, `examples/wallet_provider_example.rs`, `examples/get_tx_receipt_decode_example.rs`, `examples/app_test_token_provider.rs`, `examples/idl_*.rs`

**Interfaces:**
- Consumes: `Transaction::new_with_stamp`、`sign_transaction`、`simulate_transaction`、`submit_transaction`、`decode_transaction_response`、`decode_transaction_history`、Token/IDL `pack` API。
- Produces: 从构造指令到查询回执的完整生命周期说明，以及 Token/IDL 分类教程。

- [ ] **Step 1: 描述交易生命周期**

  按“构造 instruction → 组装 transaction/request → 签名或 WalletFiller 填充 → simulate → submit → 按 tx hash 轮询 → decode receipt/history”顺序写出最小代码片段，并解释 stamp 的作用。

- [ ] **Step 2: 补充多签名计划和批量交易**

  根据 `simulate_send_transfer_decode.rs` 说明 `SigningPlan::authorize` 如何把不同 instruction 分配给不同 signer，指出每个 signer 必须注册到 wallet。

- [ ] **Step 3: 补充 Token 与 IDL**

  按只读查询、Token 写操作、IDL 指令 `pack`、复杂结构 `Tokenizable/Detokenize`、IDL 错误码五类组织，并为每类列出准确 example 命令。

### Task 4: 编写进阶能力、示例总表和排障

**Files:**
- Modify: `README.md`
- Reference: `examples/muticall_*.rs`, `examples/app_account_multisig_on_chain.rs`, `examples/bls_aggregator_example.rs`, all files in `examples/`

**Interfaces:**
- Consumes: 当前全部 23 个 Rust example 和 `examples/request-his.md`。
- Produces: Multicall、多签/投票、BLS、完整示例索引、故障排查和安全清单。

- [ ] **Step 1: 增加 Multicall、多签、投票和 BLS 说明**

  区分只读 multicall、普通链上多签管理和 vote-based submit；说明 `bls_aggregator_example` 是本地聚合签名示例。

- [ ] **Step 2: 生成完整示例索引表**

  对 `examples/*.rs` 每个文件列出用途和 `cargo run --example <stem>` 命令；对 `idl_tokenizable_account_profile.rs`、`idl_tokenizable_example.rs` 等无网络写入的示例单独标注。

- [ ] **Step 3: 补充排障和安全章节**

  覆盖 RPC 连接失败、faucet cooldown、缺少 signer authorization、stamp 过期、交易提交后需轮询、助记词/私钥保护和公网 RPC 不适合生产等问题。

- [ ] **Step 4: 做静态一致性检查**

  运行：

  ```bash
  rtk rg --files examples -g '*.rs' | sort
  rtk rg -o 'cargo run --example [A-Za-z0-9_+-]+' README.md | sort -u
  rtk git diff --check
  ```

  预期：README 中每个 example 命令的 stem 都对应一个 `examples/<stem>.rs` 文件。

### Task 5: 验证文档与示例构建

**Files:**
- Verify: `README.md`, `Cargo.toml`, `examples/*.rs`

**Interfaces:**
- Consumes: 完成的 README 和当前 workspace。
- Produces: 可交付的文档变更与验证结果。

- [ ] **Step 1: 验证 Markdown 引用和示例覆盖**

  运行：

  ```bash
  rtk git diff --check
  rtk rg -n '\]\([^)]*\)' README.md
  ```

  预期：本地文件链接目标存在，代码围栏闭合，文档没有未完成标记或未定义变量。

- [ ] **Step 2: 编译全部 examples**

  运行：

  ```bash
  rtk cargo check --examples
  ```

  预期：命令成功；若受外部 RPC、工具链或已有仓库问题影响，记录具体失败原因，不修改无关代码。

- [ ] **Step 3: 检查最终变更**

  运行：

  ```bash
  rtk git diff --stat
  rtk git status --short
  ```

  预期：只包含 README 和本次文档流程文件，不包含构建产物或无关修改。
