use wasmlib::*;
use chrono::{DateTime,  Utc, NaiveDateTime};
use serde_with::serde_as;
use serde_json::Value;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

// 'init' is used as a way to initialize a smart contract. It is an optional
// function that will automatically be called upon contract deployment. In this
// case we use it to initialize the 'owner' state variable so that we can later
// use this information to prevent non-owners from calling certain functions.
// The 'init' function takes a single optional parameter:
// - 'owner', which is the agent id of the entity owning the contract.
// When this parameter is omitted the owner will default to the contract creator.
pub fn func_init(ctx: &ScFuncContext, f: &InitContext) {
    // The schema tool has already created a proper InitContext for this function that
    // allows us to access call parameters and state storage in a type-safe manner.

    // First we set up a default value for the owner in case the optional 'owner'
    // parameter was omitted. We use the agent that sent the deploy request.
    let mut owner: ScAgentID = ctx.request_sender();

    // Now we check if the optional 'owner' parameter is present in the params map.
    if f.params.owner().exists() {
        // Yes, it was present, so now we overwrite the default owner with
        // the one specified by the 'owner' parameter.
        owner = f.params.owner().value();
    }

    // Now that we have sorted out which agent will be the owner of this contract
    // we will save this value in the 'owner' variable in state storage on the host.
    // Read the documentation on schema.yaml to understand why this state variable is
    // supported at compile-time by code generated from schema.yaml by the schema tool.
    f.state.owner().set_value(&owner);

    // init DID_vector
    let DID_vector: Vec<String> = Vec::new();

    // update the state with the DID_vector
    ctx.put("did_array", DID_vector);
}

// 'setOwner' is used to change the owner of the smart contract.
// It updates the 'owner' state variable with the provided agent id.
// The 'setOwner' function takes a single mandatory parameter:
// - 'owner', which is the agent id of the entity that will own the contract.
// Only the current owner can change the owner.
pub fn func_set_owner(ctx: &ScFuncContext, f: &SetOwnerContext) {
    let creator = f.state.owner().value();
    let caller = ctx.caller();

    ctx.require(creator == caller, "Only creator can run this function");

    // Save the new owner parameter value in the 'owner' variable in state storage.
    f.state.owner().set_value(&f.params.owner().value());
}

// Append_did is used to add a newly created DID address
pub fn append_did(ctx: &ScFuncContext, f: &AppendDIDContext){
    ctx.require(ctx.state().get("did_array").exist(), "Only creator can run this function");

    // Try to access the "did_array" key in the state
    let DID_array =  ctx.state().get("did_array")

    // Check if the "did_array" key is an array in the state
    if let Some(array) = did_array {
        // Convert the value to a vector of String
        let DID_vector: Vec<String> = array.as_array().unwrap();

        // get the new DID from the function context params
        let new_DID = f.params.DID().value().to_string();

        // add the new DID into the DID_vector
        DID_vector.push(new_DID);

        // update the state
        // Convert the updated vector to a JSON value
        let updated_DID_vector = Value::from(DID_vector);

        // Save the updated vector back to the state
        ctx.state.put("did_array", updated_DID_vector);

    } else {
        // Return an error code or a default value if the "my_array" key does not exist
        -1
    }
}