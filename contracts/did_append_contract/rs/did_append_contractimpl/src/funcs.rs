// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0


use wasmlib::*;

use crate::*;

pub fn func_init(ctx: &ScFuncContext, f: &InitContext) {
    if f.params.owner().exists() {
        f.state.owner().set_value(&f.params.owner().value());
        return;
    }
    f.state.owner().set_value(&ctx.request_sender());
}

// This was the only function I needed to change after generating the code
pub fn func_add_did(_ctx: &ScFuncContext, f: &AddDidContext) {
    // we check if the new_did parameter exists
    if f.params.new_did().exists() {
        // We get the DID parameters provided to the function call
        let did : &str = &f.params.new_did().value().to_string();

        // the DID aray is implemented has d_id in the genrated code
        // Get the proxy to the 'DID' array in the state storage.
        let did_list = f.state.d_id();

        // Now we will append the new did to the did_list array.
        // We create an ScMutableString proxy to a string value that lives
        // at the end of the did_list array (no value yet, since we're appending).
        let new_did: ScMutableString = did_list.append_string();

        // And finally we append the new string to the array by telling the proxy
        // to update the value it refers to with the 'did' parameter.
        new_did.set_value(did);
    }
}

// function to delete a did, by giving the did to delete we go trough all the existing did and check if one exist.
// if yes we update it's string to "DELETED"
pub fn func_delete_did(ctx: &ScFuncContext, f: &DeleteDidContext) {
    if f.params.to_delete_did().exists() {
        // get the did to delete from the params
        let to_delete_did = f.params.to_delete_did().value().to_string();
        // the did list
        let did_list = f.state.d_id();
        // get the did list length
        let did_list_len = did_list.length()-1;

        // go through all the did in the list
        for idx in 0..did_list_len {
            // get the did at index "idx"
            let did_at_index = did_list.get_string((idx) as u32);
            // check if the did given in params is the same as the current did
            if(to_delete_did == did_at_index.to_string()){
                // if yes then delete it by overwritting it
                did_at_index.set_value(&("DELETED") as &str);
            }
        }
    }
}

// functions to update a did, by giving as params, the String of the did to update and the string of the new DID
pub fn func_update_did(ctx: &ScFuncContext, f: &UpdateDidContext) {
    if f.params.to_update_did().exists() {
        // get the did to update from the params
        let to_update_did = f.params.to_update_did().value().to_string();
        // get the did to update from the params
        let value_to_update_to = f.params.value().value().to_string();
        // the did list
        let did_list = f.state.d_id();
        // get the did list length
        let did_list_len = did_list.length()-1;

        // go through all the did in the list
        for idx in 0..did_list_len {
            // get the did at index "idx"
            let did_at_index = did_list.get_string((idx) as u32);
            // check if the did given in params is the same as the current did
            if(to_update_did == did_at_index.to_string()){
                // if yes then update it by overwritting it
                did_at_index.set_value(&(value_to_update_to.to_string()) as &str);
            }
        }
    }
}

pub fn func_set_owner(ctx: &ScFuncContext, f: &SetOwnerContext) {
    f.state.owner().set_value(&f.params.owner().value());
}

pub fn view_get_owner(ctx: &ScViewContext, f: &GetOwnerContext) {
    f.results.owner().set_value(&f.state.owner().value());
}

pub fn view_get_did(ctx: &ScViewContext, f: &GetDIDContext) {
    let did_list = f.state.d_id();
    let did_viewed = did_list.get_string((f.params.index().value()) as u32);
    f.results.indexed_did().set_value(&(did_viewed.to_string()) as &str);
}

pub fn view_get_length(ctx: &ScViewContext, f: &GetLengthContext) {
    // the did list
    let did_list = f.state.d_id();
    // get the did list length
    let did_list_len = did_list.length();
    f.results.length().set_value((did_list_len) as u8);
}
