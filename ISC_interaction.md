### Interacting With a Smart Contract

- deploy the contract to call `wasp-cli chain deploy-contract wasmtime inccounter "inccounter SC" tools/cluster/tests/wasm/inccounter_bg.wasm`
- wasp-cli chain call-view inccounter getCounter | wasp-cli decode string counter int
- wasp-cli chain post-request inccounter increment

With our append_did contract name `appenddid` :

- `wasp-cli chain call-view appenddid getOwner | wasp-cli decode string owner AgentID`
- `wasp-cli chain post-request appenddid addDid string newDID string testDID`
