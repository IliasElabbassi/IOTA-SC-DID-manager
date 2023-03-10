// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the schema definition file instead

#![allow(dead_code)]
#![allow(unused_imports)]

use wasmlib::*;

use crate::*;

#[derive(Clone)]
pub struct ImmutableAddDidParams {
    pub(crate) proxy: Proxy,
}

impl ImmutableAddDidParams {
    pub fn new() -> ImmutableAddDidParams {
        ImmutableAddDidParams {
            proxy: params_proxy(),
        }
    }

    pub fn new_did(&self) -> ScImmutableString {
        ScImmutableString::new(self.proxy.root(PARAM_NEW_DID))
    }
}

#[derive(Clone)]
pub struct MutableAddDidParams {
    pub(crate) proxy: Proxy,
}

impl MutableAddDidParams {
    pub fn new_did(&self) -> ScMutableString {
        ScMutableString::new(self.proxy.root(PARAM_NEW_DID))
    }
}

#[derive(Clone)]
pub struct ImmutableDeleteDidParams {
    pub(crate) proxy: Proxy,
}

impl ImmutableDeleteDidParams {
    pub fn new() -> ImmutableDeleteDidParams {
        ImmutableDeleteDidParams {
            proxy: params_proxy(),
        }
    }

    pub fn to_delete_did(&self) -> ScImmutableString {
        ScImmutableString::new(self.proxy.root(PARAM_TO_DELETE_DID))
    }
}

#[derive(Clone)]
pub struct MutableDeleteDidParams {
    pub(crate) proxy: Proxy,
}

impl MutableDeleteDidParams {
    pub fn to_delete_did(&self) -> ScMutableString {
        ScMutableString::new(self.proxy.root(PARAM_TO_DELETE_DID))
    }
}

#[derive(Clone)]
pub struct ImmutableInitParams {
    pub(crate) proxy: Proxy,
}

impl ImmutableInitParams {
    pub fn new() -> ImmutableInitParams {
        ImmutableInitParams {
            proxy: params_proxy(),
        }
    }

    pub fn owner(&self) -> ScImmutableAgentID {
        ScImmutableAgentID::new(self.proxy.root(PARAM_OWNER))
    }
}

#[derive(Clone)]
pub struct MutableInitParams {
    pub(crate) proxy: Proxy,
}

impl MutableInitParams {
    pub fn owner(&self) -> ScMutableAgentID {
        ScMutableAgentID::new(self.proxy.root(PARAM_OWNER))
    }
}

#[derive(Clone)]
pub struct ImmutableSetOwnerParams {
    pub(crate) proxy: Proxy,
}

impl ImmutableSetOwnerParams {
    pub fn new() -> ImmutableSetOwnerParams {
        ImmutableSetOwnerParams {
            proxy: params_proxy(),
        }
    }

    pub fn owner(&self) -> ScImmutableAgentID {
        ScImmutableAgentID::new(self.proxy.root(PARAM_OWNER))
    }
}

#[derive(Clone)]
pub struct MutableSetOwnerParams {
    pub(crate) proxy: Proxy,
}

impl MutableSetOwnerParams {
    pub fn owner(&self) -> ScMutableAgentID {
        ScMutableAgentID::new(self.proxy.root(PARAM_OWNER))
    }
}

#[derive(Clone)]
pub struct ImmutableUpdateDidParams {
    pub(crate) proxy: Proxy,
}

impl ImmutableUpdateDidParams {
    pub fn new() -> ImmutableUpdateDidParams {
        ImmutableUpdateDidParams {
            proxy: params_proxy(),
        }
    }

    pub fn to_update_did(&self) -> ScImmutableString {
        ScImmutableString::new(self.proxy.root(PARAM_TO_UPDATE_DID))
    }

    pub fn value(&self) -> ScImmutableString {
        ScImmutableString::new(self.proxy.root(PARAM_VALUE))
    }
}

#[derive(Clone)]
pub struct MutableUpdateDidParams {
    pub(crate) proxy: Proxy,
}

impl MutableUpdateDidParams {
    pub fn to_update_did(&self) -> ScMutableString {
        ScMutableString::new(self.proxy.root(PARAM_TO_UPDATE_DID))
    }

    pub fn value(&self) -> ScMutableString {
        ScMutableString::new(self.proxy.root(PARAM_VALUE))
    }
}

#[derive(Clone)]
pub struct ImmutableGetDIDParams {
    pub(crate) proxy: Proxy,
}

impl ImmutableGetDIDParams {
    pub fn new() -> ImmutableGetDIDParams {
        ImmutableGetDIDParams {
            proxy: params_proxy(),
        }
    }

    pub fn index(&self) -> ScImmutableUint8 {
        ScImmutableUint8::new(self.proxy.root(PARAM_INDEX))
    }
}

#[derive(Clone)]
pub struct MutableGetDIDParams {
    pub(crate) proxy: Proxy,
}

impl MutableGetDIDParams {
    pub fn index(&self) -> ScMutableUint8 {
        ScMutableUint8::new(self.proxy.root(PARAM_INDEX))
    }
}
