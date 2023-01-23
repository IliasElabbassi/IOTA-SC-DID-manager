use wasmlib::*;                                 // smart contract lib
use chrono::{DateTime,  Utc, NaiveDateTime};    // date and time lib
use serde_with::serde_as;                       // needs for serialization
use serde::{Serialize, Deserialize};
use std::collections::HashMap;                  // hashmap

#[no_mangle]
fn on_load() {
    let exports = ScExports::new();
    exports.add_func("fun1", fun1);²
    exports.add_func("init_fun", init_fun);
    exports.add_func("fun3", fun2);
}

// FUNCTION RUN ONCE ON DEPLOY
fn init_fun(context: &ScFuncContext) {
    // only contract owner should be able to do this
    let creator = context.contract_creator();
    let caller = context.caller();
    context.require(creator == caller, "You are not authorised to execute init_fun");

    // DO STUFF HERE THAT ONLY THE CREATOR CAN EXECUTE
}

// STRUCTURE IMPLEMENTATION NEEDS TO BE SERIALIZABLE
#[serde_as]
#[derive(Deserialize, Serialize)]
struct MyStruct {
    id: i32,
    name: String,
}

fn fun1(context: &ScFuncContext) {

}

fn fun2(context: &ScFuncContext) {
    // only contract owner should be able to do this
    let creator = context.contract_creator();
    let caller = context.caller();
    context.require(creator == caller, "You are not authorised to execute fun2");
}
