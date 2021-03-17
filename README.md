[![Actions Status](https://github.com/brunoamancio/IOTA-SC-HName-Generator/workflows/Build%20and%20test/badge.svg)](https://github.com/brunoamancio/IOTA-SC-HName-Generator/actions)

# Hash generator for IOTA Smart contracts

IOTA Smart contract require developers to generate hashes for smart contract functions and views, as well as for parameter and variable names. 

They are used to be passed as argument to calls to other functions in the same smart contract and other smart contract functions and views.

With this generator uses procedural macros so you can pre-generate your hashes (in compile-time) to ensure no calculations is necessary in runtime.

The usage is very simple. For example, if your smart contract has the function below:

`samplecontract.rs`
```
fn my_sc_function(ctx: &ScFuncContext) {
    ctx.log("Hello world!");
}
```

You can set your constants and generate their hashes with:

`contants.rs`
```
pub const MY_SC_FUNCTION : &str = "my_sc_function";

// Here is the HNAME generation. The output type is ScHName.
pub const HNAME_MY_SC_FUNCTION_1 : ScHname  = iota_sc_hname_generator::generate_schname!("my_sc_function");

// Here is the HASH generation. The output type is u32.
pub const HNAME_MY_SC_FUNCTION_2 : ScHname = ScHname(iota_sc_hname_generator::generate_hash!("aa"));
```

No need to manually generate hashes and hardcode them.
