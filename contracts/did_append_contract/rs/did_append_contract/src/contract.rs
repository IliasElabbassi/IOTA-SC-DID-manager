// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the schema definition file instead

#![allow(dead_code)]

use wasmlib::*;

use crate::*;

pub struct AddDidCall {
    pub func:   ScFunc,
    pub params: MutableAddDidParams,
}

pub struct DeleteDidCall {
    pub func:   ScFunc,
    pub params: MutableDeleteDidParams,
}

pub struct InitCall {
    pub func:   ScInitFunc,
    pub params: MutableInitParams,
}

pub struct SetOwnerCall {
    pub func:   ScFunc,
    pub params: MutableSetOwnerParams,
}

pub struct UpdateDidCall {
    pub func:   ScFunc,
    pub params: MutableUpdateDidParams,
}

pub struct GetOwnerCall {
    pub func:    ScView,
    pub results: ImmutableGetOwnerResults,
}

pub struct ScFuncs {
}

impl ScFuncs {
    pub fn add_did(ctx: &impl ScFuncCallContext) -> AddDidCall {
        let mut f = AddDidCall {
            func:    ScFunc::new(ctx, HSC_NAME, HFUNC_ADD_DID),
            params:  MutableAddDidParams { proxy: Proxy::nil() },
        };
        ScFunc::link_params(&mut f.params.proxy, &f.func);
        f
    }

    pub fn delete_did(ctx: &impl ScFuncCallContext) -> DeleteDidCall {
        let mut f = DeleteDidCall {
            func:    ScFunc::new(ctx, HSC_NAME, HFUNC_DELETE_DID),
            params:  MutableDeleteDidParams { proxy: Proxy::nil() },
        };
        ScFunc::link_params(&mut f.params.proxy, &f.func);
        f
    }

    pub fn init(ctx: &impl ScFuncCallContext) -> InitCall {
        let mut f = InitCall {
            func:    ScInitFunc::new(ctx, HSC_NAME, HFUNC_INIT),
            params:  MutableInitParams { proxy: Proxy::nil() },
        };
        ScInitFunc::link_params(&mut f.params.proxy, &f.func);
        f
    }

    pub fn set_owner(ctx: &impl ScFuncCallContext) -> SetOwnerCall {
        let mut f = SetOwnerCall {
            func:    ScFunc::new(ctx, HSC_NAME, HFUNC_SET_OWNER),
            params:  MutableSetOwnerParams { proxy: Proxy::nil() },
        };
        ScFunc::link_params(&mut f.params.proxy, &f.func);
        f
    }

    pub fn update_did(ctx: &impl ScFuncCallContext) -> UpdateDidCall {
        let mut f = UpdateDidCall {
            func:    ScFunc::new(ctx, HSC_NAME, HFUNC_UPDATE_DID),
            params:  MutableUpdateDidParams { proxy: Proxy::nil() },
        };
        ScFunc::link_params(&mut f.params.proxy, &f.func);
        f
    }

    pub fn get_owner(ctx: &impl ScViewCallContext) -> GetOwnerCall {
        let mut f = GetOwnerCall {
            func:    ScView::new(ctx, HSC_NAME, HVIEW_GET_OWNER),
            results: ImmutableGetOwnerResults { proxy: Proxy::nil() },
        };
        ScView::link_results(&mut f.results.proxy, &f.func);
        f
    }
}
