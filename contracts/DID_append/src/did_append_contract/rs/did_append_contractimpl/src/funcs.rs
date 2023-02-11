// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0


use wasmlib::*;

use crate::*;

// This was the only function i needed to change after generating the code
pub fn func_add_did(ctx: &ScFuncContext, f: &AddDidContext) {
    if f.params.new_did().exists() {
        f.state.d_id().append(&f.params.new_did().value());
        return;
    }
}

pub fn func_init(ctx: &ScFuncContext, f: &InitContext) {
    if f.params.owner().exists() {
        f.state.owner().set_value(&f.params.owner().value());
        return;
    }
    f.state.owner().set_value(&ctx.request_sender());
}

pub fn func_set_owner(ctx: &ScFuncContext, f: &SetOwnerContext) {
    f.state.owner().set_value(&f.params.owner().value());
}

pub fn view_get_owner(ctx: &ScViewContext, f: &GetOwnerContext) {
    f.results.owner().set_value(&f.state.owner().value());
}
