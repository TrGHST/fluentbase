// use hex_literal::hex;

use hex_literal::hex;

// // SPDX-License-Identifier: MIT
// pragma solidity 0.8.24;
//
// contract HelloWorld {
//     function sayHelloWorld() public pure returns (string memory) {
//         return "Hello World";
//     }
//     function getBalanceAsStr(address addr) public view  returns (string memory) {
//         uint256 balance = addr.balance;
//         return toString(balance);
//     }
//     function getSelfBalanceAsStr() public view  returns (string memory) {
//         uint256 balance = address(this).balance;
//         return toString(balance);
//     }
//     function toString(uint256 value) internal pure returns (string memory) {
//         if (value == 0) {
//             return "0";
//         }
//
//         uint256 temp = value;
//         uint256 digits;
//
//         while (temp != 0) {
//             digits++;
//             temp /= 10;
//         }
//
//         bytes memory buffer = new bytes(digits);
//
//         while (value != 0) {
//             digits--;
//             buffer[digits] = bytes1(uint8(48 + (value % 10)));
//             value /= 10;
//         }
//
//         return string(buffer);
//     }
// }
// methods:
// {
//     "3b2e9748": "getBalanceAsStr(address)",
//     "48b8bcc3": "getSelfBalanceAsStr()",
//     "45773e4e": "sayHelloWorld()"
// }
pub(crate) static CONTRACT_BYTECODE1: &[u8] = hex!("608060405234801561000f575f80fd5b506105ba8061001d5f395ff3fe608060405260043610610033575f3560e01c80633b2e97481461003757806345773e4e1461007357806348b8bcc31461009d575b5f80fd5b348015610042575f80fd5b5061005d600480360381019061005891906102f1565b6100bb565b60405161006a91906103a6565b60405180910390f35b34801561007e575f80fd5b506100876100e9565b60405161009491906103a6565b60405180910390f35b6100a5610126565b6040516100b291906103a6565b60405180910390f35b60605f8273ffffffffffffffffffffffffffffffffffffffff163190506100e18161013b565b915050919050565b60606040518060400160405280600b81526020017f48656c6c6f20576f726c64000000000000000000000000000000000000000000815250905090565b60605f4790506101358161013b565b91505090565b60605f8203610181576040518060400160405280600181526020017f3000000000000000000000000000000000000000000000000000000000000000815250905061028e565b5f8290505f5b5f82146101b0578080610199906103fc565b915050600a826101a99190610470565b9150610187565b5f8167ffffffffffffffff8111156101cb576101ca6104a0565b5b6040519080825280601f01601f1916602001820160405280156101fd5781602001600182028036833780820191505090505b5090505b5f8514610287578180610213906104cd565b925050600a8561022391906104f4565b603061022f9190610524565b60f81b81838151811061024557610244610557565b5b60200101907effffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff191690815f1a905350600a856102809190610470565b9450610201565b8093505050505b919050565b5f80fd5b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f6102c082610297565b9050919050565b6102d0816102b6565b81146102da575f80fd5b50565b5f813590506102eb816102c7565b92915050565b5f6020828403121561030657610305610293565b5b5f610313848285016102dd565b91505092915050565b5f81519050919050565b5f82825260208201905092915050565b5f5b83811015610353578082015181840152602081019050610338565b5f8484015250505050565b5f601f19601f8301169050919050565b5f6103788261031c565b6103828185610326565b9350610392818560208601610336565b61039b8161035e565b840191505092915050565b5f6020820190508181035f8301526103be818461036e565b905092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f819050919050565b5f610406826103f3565b91507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8203610438576104376103c6565b5b600182019050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f61047a826103f3565b9150610485836103f3565b92508261049557610494610443565b5b828204905092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b5f6104d7826103f3565b91505f82036104e9576104e86103c6565b5b600182039050919050565b5f6104fe826103f3565b9150610509836103f3565b92508261051957610518610443565b5b828206905092915050565b5f61052e826103f3565b9150610539836103f3565b9250828201905080821115610551576105506103c6565b5b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffdfea26469706673582212207ec3d35dc961bb0849482ddfc6287c3ebf1f4f3984b4bf9e55e8492f041fb2f164736f6c63430008180033").as_slice();
pub(crate) static CONTRACT_BYTECODE1_METHOD_GET_BALANCE_STR_ID: [u8; 4] = hex!("3b2e9748");
pub(crate) static CONTRACT_BYTECODE1_METHOD_GET_SELF_BALANCE_STR_ID: [u8; 4] = hex!("48b8bcc3");
pub(crate) static CONTRACT_BYTECODE1_METHOD_SAY_HELLO_WORLD_STR_ID: [u8; 4] = hex!("45773e4e");
