### Interacting With a Smart Contract

##### here is an example with inccounter
- deploy the contract to call `wasp-cli chain deploy-contract wasmtime inccounter "inccounter SC" tools/cluster/tests/wasm/inccounter_bg.wasm`
- `wasp-cli chain call-view inccounter getCounter | wasp-cli decode string counter int`
- `wasp-cli chain post-request inccounter increment`

##### here is with our contract

- `wasp-cli chain call-view appenddid getOwner | wasp-cli decode string owner agentid`
- `wasp-cli chain post-request appenddid addDid string newDID string "testDID"`

here is the template of a wasp-cli interraction in general :
- `wasp-cli chain post-request <contract_name> <function_name> <type> <key> <type> <value>`
    - <b>contract_name</b> : the name of the contract referenced when you deployed it on the chain
    - <b>function_name</b> : the functions name definned in the `schema.yaml` file
    - <b>type</b> : 
    - <b>key</b> : the key of your argument located in the `schema.yaml` file 
    - <b>type</b> : the type of your argument located in the `schema.yaml` file 
    - <b>value</b> : the value for your parameter