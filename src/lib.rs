extern crate proc_macro;
extern crate syn;

#[proc_macro]
/// Calculates hash of the input string and generates the output: "ScHname(<calculated hash>);"
/// Usage: pub const HNAME_<PROPERTY NAME> : ScHname = generate_schname!("name");
pub fn generate_schname(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input_string = get_input_string(input);
    let calculated_hash = calculate_hash_from_input(&input_string);
    generate_output_schname_tokenstream(calculated_hash)
}

/// Calculates hash of the input string and generates the output: "0x123ABC"
/// Usage1: pub const hash_name : u32 = calculate_hash!("name")
/// Usage2: enum MyEnum { Hash_Name = calculate_hash!("fairroulette"); }
#[proc_macro]
pub fn calculate_hash(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input_string = get_input_string(input);
    let calculated_hash = calculate_hash_from_input(&input_string);
    generate_output_u32_tokenstream(calculated_hash)
}

fn get_input_string(input: proc_macro::TokenStream) -> String {
    let args: Vec<proc_macro::TokenTree> = input.into_iter().collect();
    let string_arg : String = match &args.get(0) {
        Some(proc_macro::TokenTree::Literal(literal)) => literal.to_string(),
        _ => panic!("No value found")
    };

    String::from(string_arg.trim_matches('\"'))
}

fn calculate_hash_from_input(input: &str) -> u32{
    let input_string = input.to_string();
    let calculated_hash = calculate_blake2b_hash(&input_string);
    calculated_hash
}

fn generate_output_schname_tokenstream(calculated_hash : u32) -> proc_macro::TokenStream {
    let expanded = quote::quote! {
        ScHname(#calculated_hash)
    };

    let output = proc_macro::TokenStream::from(expanded);
    output
}

fn generate_output_u32_tokenstream(calculated_hash : u32) -> proc_macro::TokenStream {
    let expanded = quote::quote! {
        #calculated_hash
    };

    let output = proc_macro::TokenStream::from(expanded);
    output
}

use std::convert::TryInto;
use blake2::digest::{Update, VariableOutput};
use blake2::VarBlake2b;

fn calculate_blake2b_hash(data_string : &str) -> u32{
    let data = data_string.as_bytes();
    let mut result = 0;

    let mut hasher = VarBlake2b::new(32).unwrap();
    hasher.update(data);
    hasher.finalize_variable(|res| {
        let mut hname_bytes : &[u8] = &res[..4];
        let are_first_4_bytes_zero = hname_bytes[..4].iter().all(|x| *x == 0_u8);
        if are_first_4_bytes_zero {
            hname_bytes = &hname_bytes[4..8]
        }
        result = u32::from_le_bytes(hname_bytes.try_into().expect("could not convert bytes to little endian u32"));
    });
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fairroulette() {
        let result = calculate_blake2b_hash("fairroulette");
        assert_eq!(0xdf79d138, result);
    }

    #[test]
    fn lock_bets() {
        let result = calculate_blake2b_hash("lockBets");
        assert_eq!(0xe163b43c, result);
    }

    #[test]
    fn pay_winners() {
        let result = calculate_blake2b_hash("payWinners");
        assert_eq!(0xfb2b0144, result);
    }

    #[test]
    fn donate_with_feedback() {
        let result = calculate_blake2b_hash("donatewithfeedback");
        assert_eq!(0x696d7f66, result);
    }

    #[test]
    fn tip_100() {
        let expected = 0xeae53bfb_u32;
        let result = calculate_blake2b_hash("implements(ScHname,ScHname)->bool");
        println!("{:#x}", expected);
        println!("{:#x}", result);
        assert_eq!(expected, result);
    }
}
