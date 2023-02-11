// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the schema definition file instead

#![allow(dead_code)]
#![allow(unused_imports)]

use wasmlib::*;
use did_append_contract::*;
use crate::*;

const EXPORT_MAP: ScExportMap = ScExportMap {
    names: &[
        FUNC_ADD_DID,
        FUNC_INIT,
        FUNC_SET_OWNER,
        VIEW_GET_OWNER,
    ],
    funcs: &[
        func_add_did_thunk,
        func_init_thunk,
        func_set_owner_thunk,
    ],
    views: &[
        view_get_owner_thunk,
    ],
};

pub fn on_dispatch(index: i32) {
    EXPORT_MAP.dispatch(index);
}

pub struct AddDidContext {
    pub params: ImmutableAddDidParams,
    pub state:  Mutabledid_append_contractState,
}

fn func_add_did_thunk(ctx: &ScFuncContext) {
    ctx.log("did_append_contract.funcAddDid");
    let f = AddDidContext {
        params: ImmutableAddDidParams::new(),
        state:  Mutabledid_append_contractState::new(),
    };
    ctx.require(f.params.new_did().exists(), "missing mandatory param: newDID");
    func_add_did(ctx, &f);
    ctx.log("did_append_contract.funcAddDid ok");
}

pub struct InitContext {
    pub params: ImmutableInitParams,
    pub state:  Mutabledid_append_contractState,
}

fn func_init_thunk(ctx: &ScFuncContext) {
    ctx.log("did_append_contract.funcInit");
    let f = InitContext {
        params: ImmutableInitParams::new(),
        state:  Mutabledid_append_contractState::new(),
    };
    func_init(ctx, &f);
    ctx.log("did_append_contract.funcInit ok");
}

pub struct SetOwnerContext {
    pub params: ImmutableSetOwnerParams,
    pub state:  Mutabledid_append_contractState,
}

fn func_set_owner_thunk(ctx: &ScFuncContext) {
    ctx.log("did_append_contract.funcSetOwner");
    let f = SetOwnerContext {
        params: ImmutableSetOwnerParams::new(),
        state:  Mutabledid_append_contractState::new(),
    };

    let access = f.state.owner();
    ctx.require(access.exists(), "access not set: owner");
    ctx.require(ctx.caller() == access.value(), "no permission");

    ctx.require(f.params.owner().exists(), "missing mandatory param: owner");
    func_set_owner(ctx, &f);
    ctx.log("did_append_contract.funcSetOwner ok");
}

pub struct GetOwnerContext {
    pub results: MutableGetOwnerResults,
    pub state:   Immutabledid_append_contractState,
}

fn view_get_owner_thunk(ctx: &ScViewContext) {
    ctx.log("did_append_contract.viewGetOwner");
    let f = GetOwnerContext {
        results: MutableGetOwnerResults::new(),
        state:   Immutabledid_append_contractState::new(),
    };
    view_get_owner(ctx, &f);
    ctx.results(&f.results.proxy);
    ctx.log("did_append_contract.viewGetOwner ok");
}
