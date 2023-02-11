// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0


use wasmlib::*;

use crate::*;

// This was the only function I needed to change after generating the code
pub fn func_add_did(_ctx: &ScFuncContext, f: &AddDidContext) {
    if f.params.new_did().exists() {
        // the DID aray is implemented has d_id in the genrated code
        f.state.d_id().append_string(f.params.new_did().value());
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
