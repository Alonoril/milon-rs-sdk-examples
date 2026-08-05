```text
instructions:
  instruction[0]: {"app_id":2,"app_name":"token","instruction_name":"ClaimFaucet","token":{"method":"ClaimFaucet","fields":[{"name":"claimer","value":"214RxzUxqRR1P4M5Hjw5mstr1Xs8"}]}}
  instruction[1]: {"app_id":2,"app_name":"token","instruction_name":"Transfer","token":{"method":"Transfer","fields":[{"name":"from","value":"214RxzUxqRR1P4M5Hjw5mstr1Xs8"},{"name":"token","value":"M11on1111111111111111111111"},{"name":"to","value":"3WoBgRDRzQ9omYBfXF8H6yFUaKWA"},{"name":"amount","value":1}]}}
  instruction[2]: {"app_id":2,"app_name":"token","instruction_name":"Transfer","token":{"method":"Transfer","fields":[{"name":"from","value":"3WoBgRDRzQ9omYBfXF8H6yFUaKWA"},{"name":"token","value":"M11on1111111111111111111111"},{"name":"to","value":"2tZF3bCJ5CA3EQM9hnsjc22Cv21B"},{"name":"amount","value":1}]}}
  instruction[3]: {"app_id":255,"app_name":"demo","instruction_name":"InitPool","token":{"method":"InitPool","fields":[{"name":"pool","value":"2aV1ZRw3E1Wx5utWL52QTK74QxWf"},{"name":"label","value":"simulate batch credit"}]}}
  instruction[4]: {"app_id":255,"app_name":"demo","instruction_name":"BatchCredit","token":{"method":"BatchCredit","fields":[{"name":"pool","value":"2aV1ZRw3E1Wx5utWL52QTK74QxWf"},{"name":"recipients","value":["9vYsqJf1hQN95XtGNMH98z81tze"]},{"name":"amount","value":42}]}}
  instruction[5]: {"app_id":2,"app_name":"token","instruction_name":"Transfer","token":{"method":"Transfer","fields":[{"name":"from","value":"2tZF3bCJ5CA3EQM9hnsjc22Cv21B"},{"name":"token","value":"M11on1111111111111111111111"},{"name":"to","value":"3gYrSbJdyiTDeQaPzvTrLgmTsbNS"},{"name":"amount","value":1}]}}
  instruction[6]: {"app_id":2,"app_name":"token","instruction_name":"Transfer","token":{"method":"Transfer","fields":[{"name":"from","value":"3gYrSbJdyiTDeQaPzvTrLgmTsbNS"},{"name":"token","value":"M11on1111111111111111111111"},{"name":"to","value":"214RxzUxqRR1P4M5Hjw5mstr1Xs8"},{"name":"amount","value":1}]}}
simulate receipt:
  tx_id: 111111111111
  tx_hash: Hwhg6ARKUctc5pXCVDuf8oeTVxRjUxLwo9TdThEL1zhB
  state: 1 (success)
  access_count: 10
  access[0].resource_id: FixedBytes([2, 226, 30, 180, 197, 127, 152, 170, 177, 8, 147, 29, 78, 140, 178, 10, 224, 88])
  access[0].first_snapshot: none
  access[0].last_written: inline DecodedResource { name: "u64", type_tag: 5563585020063213298, token: U64(1784791260419) }
  access[1].resource_id: FixedBytes([2, 35, 37, 181, 72, 217, 75, 246, 160, 92, 55, 185, 191, 128, 197, 83, 246, 38])
  access[1].first_snapshot: inline DecodedResource { name: "u64", type_tag: 5563585020063213298, token: U64(1000400000000) }
  access[1].last_written: inline DecodedResource { name: "u64", type_tag: 5563585020063213298, token: U64(1000410000000) }
  access[2].resource_id: FixedBytes([2, 34, 255, 10, 170, 190, 31, 28, 71, 150, 190, 220, 214, 68, 213, 11, 45, 74])
  access[2].first_snapshot: none
  access[2].last_written: inline DecodedResource { name: "u64", type_tag: 5563585020063213298, token: U64(9937200) }
  access[3].resource_id: FixedBytes([2, 34, 219, 135, 88, 254, 179, 118, 85, 215, 237, 148, 129, 233, 50, 129, 207, 16])
  access[3].first_snapshot: inline DecodedResource { name: "u64", type_tag: 5563585020063213298, token: U64(1777825) }
  access[3].last_written: inline DecodedResource { name: "u64", type_tag: 5563585020063213298, token: U64(1840625) }
  access[4].resource_id: FixedBytes([2, 34, 137, 224, 68, 189, 172, 80, 49, 188, 203, 77, 4, 136, 157, 135, 66, 25])
  access[4].first_snapshot: none
  access[4].last_written: inline DecodedResource { name: "u64", type_tag: 5563585020063213298, token: U64(0) }
  access[5].resource_id: FixedBytes([2, 34, 198, 32, 141, 179, 177, 245, 205, 234, 44, 204, 20, 148, 99, 74, 99, 179])
  access[5].first_snapshot: none
  access[5].last_written: inline DecodedResource { name: "u64", type_tag: 5563585020063213298, token: U64(0) }
  access[6].resource_id: FixedBytes([255, 237, 37, 55, 201, 157, 238, 49, 205, 39, 232, 157, 25, 203, 184, 15, 72, 63])
  access[6].first_snapshot: none
  access[6].last_written: inline DecodedResource { name: "Address", type_tag: 17438174819379414968, token: Address(2aV1ZRw3E1Wx5utWL52QTK74QxWf) }
  access[7].resource_id: FixedBytes([255, 132, 49, 54, 34, 155, 76, 151, 224, 34, 216, 135, 47, 179, 148, 9, 29, 219])
  access[7].first_snapshot: none
  access[7].last_written: inline DecodedResource { name: "Label", type_tag: 4454442085531989710, token: Struct { name: "Label", fields: [NamedToken { name: "text", value: String("simulate batch credit") }] } }
  access[8].resource_id: FixedBytes([255, 156, 200, 145, 69, 70, 142, 85, 182, 69, 26, 145, 238, 25, 117, 134, 238, 38])
  access[8].first_snapshot: none
  access[8].last_written: inline DecodedResource { name: "u64", type_tag: 5563585020063213298, token: U64(42) }
  access[9].resource_id: FixedBytes([2, 34, 110, 225, 147, 252, 111, 29, 76, 114, 54, 145, 22, 92, 168, 119, 26, 212])
  access[9].first_snapshot: none
  access[9].last_written: inline DecodedResource { name: "u64", type_tag: 5563585020063213298, token: U64(0) }
  event_count: 1
  event[0]: DecodedEvent { app_name: "demo", event_name: "EventCreditApplied", type_tag: 7407037194950745602, token: Struct { name: "EventCreditApplied", fields: [NamedToken { name: "pool", value: Address(2aV1ZRw3E1Wx5utWL52QTK74QxWf) }, NamedToken { name: "recipient", value: Address(9vYsqJf1hQN95XtGNMH98z81tze) }, NamedToken { name: "amount", value: U64(42) }] } }
  error: None
------------------------------------------------------------------------------------------------
submit txn tx_hash: Hwhg6ARKUctc5pXCVDuf8oeTVxRjUxLwo9TdThEL1zhB




--- local:
--- old:
instructions:
  instruction[0]: {"app_id":255,"app_name":"demo","instruction_name":"InitPool","token":{"method":"InitPool","fields":[{"name":"pool","value":"282yBJGHwZU3RPFVzrV3zQqkMyvu"},{"name":"label","value":"simulate batch credit"}]}}
  instruction[1]: {"app_id":255,"app_name":"demo","instruction_name":"BatchCredit","token":{"method":"BatchCredit","fields":[{"name":"pool","value":"282yBJGHwZU3RPFVzrV3zQqkMyvu"},{"name":"recipients","value":["9vYsqJf1hQN95XtGNMH98z81tze"]},{"name":"amount","value":42}]}}
simulate receipt:
  tx_id: 000000000000000000000000
  tx_hash: a55dfc810e8a212374cd9edcb5ad50b3afc85234e762114afe8621c328a2301f
  state: 1 (success)
  access_count: 3
  access[0].resource_id: [255, 237, 135, 171, 240, 33, 35, 110, 176, 46, 176, 158, 215, 168, 178, 72, 67, 127]
  access[0].first_snapshot: inline
  access[0].last_written: inline
  access[1].resource_id: [255, 132, 41, 117, 62, 128, 166, 144, 12, 69, 255, 132, 144, 8, 57, 19, 116, 29]
  access[1].first_snapshot: inline
  access[1].last_written: inline
  access[2].resource_id: [255, 156, 134, 174, 197, 211, 139, 117, 167, 166, 227, 63, 185, 95, 74, 83, 38, 83]
  access[2].first_snapshot: none
  access[2].last_written: inline
  event_count: 1
  event[0]: AnySerializeOwned(EventCreditApplied { pool: 282yBJGHwZU3RPFVzrV3zQqkMyvu, recipient: 9vYsqJf1hQN95XtGNMH98z81tze, amount: 42 })
  error: None

--- new:
instructions:
  instruction[0]: {"app_id":255,"app_name":"demo","instruction_name":"InitPool","token":{"method":"InitPool","fields":[{"name":"pool","value":"xX2pmxzYakWED8c9d3nRcgZDpmZ"},{"name":"label","value":"simulate batch credit"}]}}
  instruction[1]: {"app_id":255,"app_name":"demo","instruction_name":"BatchCredit","token":{"method":"BatchCredit","fields":[{"name":"pool","value":"xX2pmxzYakWED8c9d3nRcgZDpmZ"},{"name":"recipients","value":["9vYsqJf1hQN95XtGNMH98z81tze"]},{"name":"amount","value":42}]}}
simulate receipt:
  tx_id: 000000000000000000000000
  tx_hash: 1e4dd2f3c03a92f415ce4cad353cbfac4977ece1b36a9b4e9c30e1945155e816
  state: 1 (success)
  access_count: 3
  access[0].resource_id: FixedBytes([255, 237, 171, 200, 212, 23, 204, 178, 231, 162, 197, 151, 161, 225, 95, 134, 76, 188])
  access[0].first_snapshot: inline {"name":"Address","token":"xX2pmxzYakWED8c9d3nRcgZDpmZ"}
  access[0].last_written: inline {"name":"Address","token":"xX2pmxzYakWED8c9d3nRcgZDpmZ"}
  access[1].resource_id: FixedBytes([255, 132, 205, 101, 153, 4, 65, 209, 48, 160, 198, 167, 88, 84, 155, 114, 43, 41])
  access[1].first_snapshot: inline {"name":"Label","token":{"method":"Label","fields":[{"name":"text","value":"simulate batch credit"}]}}
  access[1].last_written: inline {"name":"Label","token":{"method":"Label","fields":[{"name":"text","value":"simulate batch credit"}]}}
  access[2].resource_id: FixedBytes([255, 156, 147, 150, 64, 39, 179, 170, 248, 222, 42, 247, 22, 17, 157, 169, 121, 23])
  access[2].first_snapshot: none
  access[2].last_written: inline {"name":"u64","token":42}
  event_count: 1
  event[0]: DecodedEvent { app_name: "demo", event_name: "EventCreditApplied", type_tag: 7407037194950745602, token: Struct { name: "EventCreditApplied", fields: [NamedToken { name: "pool", value: Address(xX2pmxzYakWED8c9d3nRcgZDpmZ) }, NamedToken { name: "recipient", value: Address(9vYsqJf1hQN95XtGNMH98z81tze) }, NamedToken { name: "amount", value: U64(42) }] } }
  error: None

--- 2
instructions:
  instruction[0]: {"app_id":255,"app_name":"demo","instruction_name":"InitPool","token":{"method":"InitPool","fields":[{"name":"pool","value":"3v9A85AYUTE8xr3MSdhXFRYEubJS"},{"name":"label","value":"simulate batch credit"}]}}
  instruction[1]: {"app_id":255,"app_name":"demo","instruction_name":"BatchCredit","token":{"method":"BatchCredit","fields":[{"name":"pool","value":"3v9A85AYUTE8xr3MSdhXFRYEubJS"},{"name":"recipients","value":["9vYsqJf1hQN95XtGNMH98z81tze"]},{"name":"amount","value":42}]}}
simulate receipt:
  tx_id: 111111111111
  tx_hash: 77Q2ZWzAJ2iaTz3N7yicGgAfRcBJ4jNK5Bangz4U2v23
  state: 1 (success)
  access_count: 3
  access[0].resource_id: BcgMree1ktZX9c5MuZkLvWwoc
  access[0].first_snapshot: inline DecodedResource { name: "Address", token: Address(3v9A85AYUTE8xr3MSdhXFRYEubJS) }
  access[0].last_written: inline DecodedResource { name: "Address", token: Address(3v9A85AYUTE8xr3MSdhXFRYEubJS) }
  access[1].resource_id: Bbh7dk2AQSJdvDAUDHW35Zp6A
  access[1].first_snapshot: inline DecodedResource { name: "Label", token: Struct { name: "Label", fields: [NamedToken { name: "text", value: String("simulate batch credit") }] } }
  access[1].last_written: inline DecodedResource { name: "Label", token: Struct { name: "Label", fields: [NamedToken { name: "text", value: String("simulate batch credit") }] } }
  access[2].resource_id: BbutkTiYLxVSMAQTsBQp98Gu9
  access[2].first_snapshot: none
  access[2].last_written: inline DecodedResource { name: "u64", token: U64(42) }
  event_count: 1
  event[0]: DecodedEvent { app_name: "demo", event_name: "EventCreditApplied", type_tag: 7407037194950745602, token: Struct { name: "EventCreditApplied", fields: [NamedToken { name: "pool", value: Address(3v9A85AYUTE8xr3MSdhXFRYEubJS) }, NamedToken { name: "recipient", value: Address(9vYsqJf1hQN95XtGNMH98z81tze) }, NamedToken { name: "amount", value: U64(42) }] } }
  error: None

--- test
instructions:
  instruction[0]: {"app_id":255,"app_name":"demo","instruction_name":"InitPool","token":{"method":"InitPool","fields":[{"name":"pool","value":"3ijyHaRRmUytRMVwoeQQViWyhQfL"},{"name":"label","value":"simulate batch credit"}]}}
  instruction[1]: {"app_id":255,"app_name":"demo","instruction_name":"BatchCredit","token":{"method":"BatchCredit","fields":[{"name":"pool","value":"3ijyHaRRmUytRMVwoeQQViWyhQfL"},{"name":"recipients","value":["9vYsqJf1hQN95XtGNMH98z81tze"]},{"name":"amount","value":42}]}}
simulate receipt:
  tx_id: 111111111111
  tx_hash: HiVryMAVGaPzBWVqp2GiwK2RGDaaz11qLhFx4qU46DZj
  state: 1 (success)
  access_count: 3
  access[0].resource_id: FixedBytes([255, 237, 195, 14, 167, 2, 120, 183, 10, 40, 11, 57, 89, 218, 36, 207, 64, 215])
  access[0].first_snapshot: inline DecodedResource { name: "Address", token: Address(3ijyHaRRmUytRMVwoeQQViWyhQfL) }
  access[0].last_written: inline DecodedResource { name: "Address", token: Address(3ijyHaRRmUytRMVwoeQQViWyhQfL) }
  access[1].resource_id: FixedBytes([255, 132, 139, 229, 4, 25, 25, 15, 40, 128, 172, 9, 216, 153, 38, 151, 53, 183])
  access[1].first_snapshot: inline DecodedResource { name: "Label", token: Struct { name: "Label", fields: [NamedToken { name: "text", value: String("simulate batch credit") }] } }
  access[1].last_written: inline DecodedResource { name: "Label", token: Struct { name: "Label", fields: [NamedToken { name: "text", value: String("simulate batch credit") }] } }
  access[2].resource_id: FixedBytes([255, 156, 195, 147, 222, 20, 29, 227, 28, 171, 94, 11, 246, 120, 168, 235, 44, 121])
  access[2].first_snapshot: none
  access[2].last_written: inline DecodedResource { name: "u64", token: U64(42) }
  event_count: 1
  event[0]: DecodedEvent { app_name: "demo", event_name: "EventCreditApplied", type_tag: 7407037194950745602, token: Struct { name: "EventCreditApplied", fields: [NamedToken { name: "pool", value: Address(3ijyHaRRmUytRMVwoeQQViWyhQfL) }, NamedToken { name: "recipient", value: Address(9vYsqJf1hQN95XtGNMH98z81tze) }, NamedToken { name: "amount", value: U64(42) }] } }
  error: None
tx_hash: HiVryMAVGaPzBWVqp2GiwK2RGDaaz11qLhFx4qU46DZj
trans size: 425

```