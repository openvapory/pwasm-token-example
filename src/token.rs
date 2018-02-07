// Contract doesn't use standard library
#![no_std]

extern crate pwasm_std;
extern crate pwasm_abi;
extern crate pwasm_token_contract;
extern crate pwasm_ethereum as ext;

use pwasm_abi::eth::EndpointInterface;

/// The main function receives a pointer for the call descriptor.
#[no_mangle]
pub fn call() {
	// pwasm_std::parse_args parses the call descriptor into arguments and result pointers
	let args = ext::input();
	let mut endpoint = pwasm_token_contract::Endpoint::new(pwasm_token_contract::TokenContractInstance{});
	// Args is an Solidity-compatible abi call: first 4 bytes are the Method ID of keccak hash of function signature
	// followed by sequence of arguments packed into chunks of 32 bytes.
	// Read http://solidity.readthedocs.io/en/develop/abi-spec.html#formal-specification-of-the-encoding for details
	ext::ret(&endpoint.dispatch(&args));
}

#[no_mangle]
pub fn deploy() {
	let args = ext::input();
	let mut endpoint = pwasm_token_contract::Endpoint::new(pwasm_token_contract::TokenContractInstance{});
	endpoint.dispatch_ctor(&args);
}
