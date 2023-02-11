// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the schema definition file instead

#![allow(dead_code)]

use wasmlib::*;

pub const SC_NAME        : &str = "did_append_contract";
pub const SC_DESCRIPTION : &str = "did_append_contract description";
pub const HSC_NAME       : ScHname = ScHname(0xd8cbbfdb);

pub const PARAM_NEW_DID : &str = "newDID";
pub const PARAM_OWNER   : &str = "owner";

pub const RESULT_OWNER : &str = "owner";

pub const STATE_D_ID  : &str = "DID";
pub const STATE_OWNER : &str = "owner";

pub const FUNC_ADD_DID   : &str = "addDid";
pub const FUNC_INIT      : &str = "init";
pub const FUNC_SET_OWNER : &str = "setOwner";
pub const VIEW_GET_OWNER : &str = "getOwner";

pub const HFUNC_ADD_DID   : ScHname = ScHname(0x555df086);
pub const HFUNC_INIT      : ScHname = ScHname(0x1f44d644);
pub const HFUNC_SET_OWNER : ScHname = ScHname(0x2a15fe7b);
pub const HVIEW_GET_OWNER : ScHname = ScHname(0x137107a6);
