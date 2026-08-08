# Complex Detokenize Example Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox syntax for tracking.

**Goal:** 提供一个可运行的复杂结构体 Tokenizable 往返示例。

**Architecture:** AccountProfile 手写 Tokenizable，顶层使用 Token::Struct，内部字段委托 SDK 已有通用 Tokenizable 实现。main 验证结构体到 Token、Token 到结构体、单输出 Detokenize 三条路径。

**Tech Stack:** Rust 2024、milon_idl_core、标准库 BTreeMap。

## Global Constraints

- 只修改 only-sdk-examples/examples/detokenize_example.rs。
- 示例中的所有注释使用英文。
- 从 malformed Token 返回 InvalidOutputType，不得 panic。
- 使用字段名和固定字段顺序严格校验 Token::Struct。
- 不修改 milon_idl_core 或 IDL 宏。

---

### Task 1: 定义复杂结构体和失败行为测试

**Files:**

- Modify: only-sdk-examples/examples/detokenize_example.rs

**Interfaces:**

- Produces: AccountProfile { address, enabled, display_name, alias, revision, roles, balances }。
- Produces: Tokenizable for AccountProfile。

- [ ] **Step 1: 写入失败断言**

    let token = Token::Struct {
        name: "AccountProfile",
        fields: vec![NamedToken {
            name: "enabled",
            value: Token::Bool(true),
        }],
    };
    assert!(AccountProfile::from_token(token).is_err());

- [ ] **Step 2: 运行示例，确认 AccountProfile 尚不存在而失败**

Run: cargo run --example detokenize_example

Expected: FAIL，提示 AccountProfile 或 Tokenizable 实现未定义。

- [ ] **Step 3: 定义结构体和严格 Tokenizable 实现**

    #[derive(Debug, PartialEq, Eq)]
    struct AccountProfile {
        address: Address,
        enabled: bool,
        display_name: String,
        alias: Option<String>,
        revision: [u8; 4],
        roles: Vec<(u8, String)>,
        balances: BTreeMap<String, u64>,
    }

    impl Tokenizable for AccountProfile {
        fn from_token(token: Token) -> Result<Self, InvalidOutputType> {
            let Token::Struct { name: "AccountProfile", fields } = token else {
                return Err(InvalidOutputType("expected AccountProfile struct".to_owned()));
            };
            let [address, enabled, display_name, alias, revision, roles, balances] =
                fields.try_into().map_err(|fields: Vec<NamedToken>| {
                    InvalidOutputType(format!("expected 7 fields, got {}", fields.len()))
                })?;
            // Verify every field name, then call the matching field type's from_token.
        }

        fn into_token(self) -> Token {
            Token::Struct {
                name: "AccountProfile",
                fields: vec![
                    NamedToken { name: "address", value: self.address.into_token() },
                    NamedToken { name: "enabled", value: self.enabled.into_token() },
                    NamedToken { name: "display_name", value: self.display_name.into_token() },
                    NamedToken { name: "alias", value: self.alias.into_token() },
                    NamedToken { name: "revision", value: self.revision.into_token() },
                    NamedToken { name: "roles", value: self.roles.into_token() },
                    NamedToken { name: "balances", value: self.balances.into_token() },
                ],
            }
        }
    }

- [ ] **Step 4: Run the example**

Run: cargo run --example detokenize_example

Expected: PASS after Task 2 adds the valid round-trip demonstration.

### Task 2: 演示双向转换并验证

**Files:**

- Modify: only-sdk-examples/examples/detokenize_example.rs

**Interfaces:**

- Consumes: Task 1 AccountProfile and Tokenizable implementation.
- Produces: Executable proof of Token and Detokenize round trips.

- [ ] **Step 1: 添加有效往返断言**

    let profile = AccountProfile {
        address: Address::from_bytes(&[7_u8; 20]).unwrap(),
        enabled: true,
        display_name: "Milon validator".to_owned(),
        alias: Some("validator-7".to_owned()),
        revision: [1, 0, 0, 7],
        roles: vec![(1, "operator".to_owned()), (2, "treasury".to_owned())],
        balances: BTreeMap::from([
            ("MIL".to_owned(), 1_000_000),
            ("USDM".to_owned(), 42_000),
        ]),
    };
    let token = profile.clone().into_token();
    assert_eq!(AccountProfile::from_token(token.clone()).unwrap(), profile);
    assert_eq!(AccountProfile::from_tokens(vec![token]).unwrap(), profile);

- [ ] **Step 2: 运行示例验证**

Run: cargo run --example detokenize_example

Expected: PASS，无输出且退出码为 0。

- [ ] **Step 3: 格式化并检查目标文件**

Run: rustfmt --check examples/detokenize_example.rs

Expected: PASS。

