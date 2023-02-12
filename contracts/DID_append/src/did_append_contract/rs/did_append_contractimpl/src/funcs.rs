// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0


use wasmlib::*;

use crate::*;

// This was the only function I needed to change after generating the code
pub fn func_add_did(_ctx: &ScFuncContext, f: &AddDidContext) {
    if f.params.new_did().exists() {
        // We get the DID parameters provided to the function call
        let did : ScMutableString = f.params.new_did.value();

        // the DID aray is implemented has d_id in the genrated code
        // Get the proxy to the 'DID' array in the state storage.
        let did_list: ArrayOfMutableString = f.state.d_id();
        // f.state.d_id().append_string(f.params.new_did().value());

        // Now we will append the new did to the did_list array.
        // We create an ScMutableString proxy to a string value that lives
        // at the end of the did_list array (no value yet, since we're appending).
        let new_did: ScMutableString = member_list.append_string();

        // And finally we append the new string to the array by telling the proxy
        // to update the value it refers to with the 'did' parameter.
        new_did.set_value(&did);
    }
}

pub fn func_init(_ctx: &ScFuncContext, f: &InitContext) {
    if f.params.owner().exists() {
        f.state.owner().set_value(&f.params.owner().value());
        return;
    }
    f.state.owner().set_value(&_ctx.request_sender());
}

pub fn func_set_owner(_ctx: &ScFuncContext, f: &SetOwnerContext) {
    f.state.owner().set_value(&f.params.owner().value());
}

pub fn view_get_owner(_ctx: &ScViewContext, f: &GetOwnerContext) {
    f.results.owner().set_value(&f.state.owner().value());
}
