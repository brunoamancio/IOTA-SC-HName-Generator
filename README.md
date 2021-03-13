[![Actions Status](https://github.com/brunoamancio/IOTA-SC-HName-Generator/workflows/Build%20and%20test/badge.svg)](https://github.com/brunoamancio/IOTA-SC-HName-Generator/actions)

# Hash generator for IOTA Smart contracts

IOTA Smart contract require developers to generate hashes for smart contract functions and views, as well as for parameter and variable names. 

They are used to be passed as argument to calls to other function in the same smart contract and other smart contract functions and views.

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

// Here is the HNAME generation
pub const HNAME_MY_SC_FUNCTION : ScHname  = generate_hname!("my_sc_function");
```

No need to manually generate hashes and hardcode them.
