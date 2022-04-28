pub use themeliobridgetest_mod::*;
#[allow(clippy::too_many_arguments)]
mod themeliobridgetest_mod {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "ThemelioBridgeTest was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static THEMELIOBRIDGETEST_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"height\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"HeaderAlreadyRelayed\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"signerSyms\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"epochSyms\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"InsufficientSignatures\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"signersLength\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"signaturesLength\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"InvalidSignatures\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"height\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"MissingHeader\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"start\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"end\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"dataLength\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"OutOfBoundsSlice\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"TxNotVerified\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Approval\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"height\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"HeaderRelayed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"epoch\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32[]\",\"name\":\"stakers\",\"type\":\"bytes32[]\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256[]\",\"name\":\"symsStaked\",\"type\":\"uint256[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"StakersRelayed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"themelioRecipient\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"TokensBurned\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"TokensMinted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Transfer\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"tx_hash\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"height\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"TxVerified\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"msg\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"WARNING_Deprecated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_address\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_bytes\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_bytes32\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"\",\"type\":\"int256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_int\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"val\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_address\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"val\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_bytes\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes32\",\"name\":\"val\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_bytes32\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256\",\"name\":\"val\",\"type\":\"int256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"decimals\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_decimal_int\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"val\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"decimals\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_decimal_uint\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256\",\"name\":\"val\",\"type\":\"int256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_int\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"val\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_string\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"val\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_uint\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_string\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_uint\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"logs\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"IS_TEST\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"allowance\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"approve\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"themelioRecipient\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"burn\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"header\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"blockHeight\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"txHash\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32[]\",\"name\":\"proof\",\"type\":\"bytes32[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"computeMerkleRootTestHelper\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"give\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deal\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"give\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"adjust\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deal\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"give\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deal\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"decimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"header\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"offset\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"decodeIntegerTestHelper\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"subtractedValue\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"decreaseAllowance\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"what\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"args\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deployCode\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"addr\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"what\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deployCode\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"addr\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"epochs\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"totalStakedSyms\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"header\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"extractBlockHeightTestHelper\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"transaction\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"extractValueAndRecipientTestHelper\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"failed\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"headers\",\"outputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"who\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"hoax\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"who\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"origin\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"hoax\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"who\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"origin\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"give\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"hoax\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"who\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"give\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"hoax\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"addedValue\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"increaseAllowance\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"name\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"header_\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes32[]\",\"name\":\"signers_\",\"type\":\"bytes32[]\",\"components\":[]},{\"internalType\":\"bytes32[]\",\"name\":\"signatures_\",\"type\":\"bytes32[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"relayHeader\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32[]\",\"name\":\"signers\",\"type\":\"bytes32[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"relayHeaderTestHelper\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"epoch_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32[]\",\"name\":\"stakers_\",\"type\":\"bytes32[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"stakerSyms_\",\"type\":\"uint256[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"relayStakers\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"epoch\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"staker\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"relayStakersHelper\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"time\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"rewind\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"time\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"skip\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"who\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"give\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"startHoax\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"who\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"origin\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"give\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"startHoax\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"who\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"startHoax\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"who\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"origin\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"startHoax\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"symbol\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"testBlake3Hasher\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"testBlake3KeyedHasher\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"testDecimals\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"testEd25519\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"testEncodedIntegerSize\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"testExtractMerkleRoot\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"testExtractTokenType\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"testHashDatablock\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"testHashNode\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"testSlice\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"give\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"tip\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transfer\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferFrom\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"transaction_\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"txIndex_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"blockHeight_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32[]\",\"name\":\"proof_\",\"type\":\"bytes32[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"verifyTx\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"header\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"blockHeight\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"verifyTxTestHelper\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"vm\",\"outputs\":[{\"internalType\":\"contract Vm\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static THEMELIOBRIDGETEST_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60806040526007805460ff191660011790553480156200001e57600080fd5b50604080518082018252600b81526a1ddc985c1c1959081b595b60aa1b6020808301918252835180850190945260048452631dd3515360e21b9084015281519192916200006e916003916200008d565b508051620000849060049060208401906200008d565b5050506200016f565b8280546200009b9062000133565b90600052602060002090601f016020900481019282620000bf57600085556200010a565b82601f10620000da57805160ff19168380011785556200010a565b828001600101855582156200010a579182015b828111156200010a578251825591602001919060010190620000ed565b50620001189291506200011c565b5090565b5b808211156200011857600081556001016200011d565b600181811c908216806200014857607f821691505b6020821081036200016957634e487b7160e01b600052602260045260246000fd5b50919050565b61a044806200017f6000396000f3fe608060405234801561001057600080fd5b50600436106103275760003560e01c806370a08231116101b8578063bcf64e0511610104578063dd62ed3e116100a2578063f34d5c4a1161007c578063f34d5c4a146106cf578063f7842f2d146106d7578063f8551b181461071e578063fa7626d41461072657600080fd5b8063dd62ed3e146106a1578063e27c5b17146106b4578063e9a79a7b146106bc57600080fd5b8063d06d8229116100de578063d06d822914610660578063d6004d5114610673578063d82555f11461067b578063d8626f361461068e57600080fd5b8063bcf64e051461061a578063c6b61e4c1461062d578063c88a5e6d1461064d57600080fd5b8063a457c2d711610171578063af9bbe5f1161014b578063af9bbe5f146105e4578063b2d7ee31146105f7578063b9c071b4146105ff578063ba414fa61461061257600080fd5b8063a457c2d7146105be578063a797dccb14610375578063a9059cbb146105d157600080fd5b806370a0823114610541578063852418ba1461056a57806395d89b411461057d57806397754ae9146105855780639a8325a0146105985780639f84cd1d146105ab57600080fd5b80632d6c17a311610277578063442a807a116102305780635d4696cf1161020a5780635d4696cf146104e3578063679c917c146104eb5780636bce989b1461051b5780636f5970751461052e57600080fd5b8063442a807a146104b557806351ee9269146104c857806356f90d79146104d057600080fd5b80632d6c17a314610445578063313ce5671461045857806339509351146104675780633a7684631461047a5780633bf82db11461048f57806342217b63146104a257600080fd5b806314229b63116102e457806323b872dd116102be57806323b872dd146103ec5780632993c221146103ff57806329a9e3001461040757806329ce9dde1461041a57600080fd5b806314229b63146103be57806318160ddd146103d1578063233240ee146103d957600080fd5b806306fdde031461032c578063073a47f31461034a57806307e075b41461036d578063095ea7b3146103775780630f9d0d471461038a578063108554f2146103ab575b600080fd5b610334610733565b6040516103419190619040565b60405180910390f35b61035d610358366004619097565b6107c5565b6040519015158152602001610341565b6103756108c5565b005b61035d610385366004619127565b61094c565b61039d610398366004619192565b610966565b604051908152602001610341565b6103756103b9366004619127565b61097b565b61039d6103cc3660046191d3565b610a4c565b60025461039d565b6103756103e736600461921e565b610a65565b61035d6103fa366004619239565b610b39565b610375610b5d565b610375610415366004619275565b610cd3565b61042d610428366004619373565b610d7e565b6040516001600160a01b039091168152602001610341565b6103756104533660046193ea565b610e2f565b60405160098152602001610341565b61035d610475366004619127565b610e6b565b61042d600080516020619fcf83398151915281565b61037561049d366004619239565b610e8d565b61035d6104b0366004619403565b610f67565b61035d6104c3366004619485565b611108565b61037561137b565b6103346104de3660046193ea565b611422565b6103756114bc565b6104fe6104f9366004619192565b6114cb565b604080519283526001600160a01b03909116602083015201610341565b610375610529366004619239565b6114ea565b61037561053c36600461921e565b6114fc565b61039d61054f36600461921e565b6001600160a01b031660009081526020819052604090205490565b6103756105783660046194ff565b61159f565b610334611651565b610375610593366004619542565b611660565b61042d6105a6366004619591565b611880565b6103756105b93660046191d3565b61190f565b61035d6105cc366004619127565b61192e565b61035d6105df366004619127565b6119ab565b6103756105f2366004619239565b6119b9565b610375611a60565b61037561060d3660046193ea565b611b32565b61035d611b50565b6103756106283660046195c5565b611c75565b61039d61063b3660046193ea565b60066020526000908152604090205481565b61037561065b366004619127565b611cbd565b61037561066e366004619275565b611cf0565b610375611d9b565b610375610689366004619239565b611f19565b61039d61069c3660046195e7565b611fc4565b61039d6106af366004619275565b611ff9565b610375612024565b6103756106ca366004619127565b6120c3565b610375612162565b6107096106e53660046195c5565b60009182526006602090815260408084208054938552600101909152909120549091565b60408051928352602083019190915201610341565b6103756121bc565b60075461035d9060ff1681565b60606003805461074290619673565b80601f016020809104026020016040519081016040528092919081815260200182805461076e90619673565b80156107bb5780601f10610790576101008083540402835291602001916107bb565b820191906000526020600020905b81548152906001019060200180831161079e57829003601f168201915b5050505050905090565b60008084815b81811015610865578585828181106107e5576107e56196ad565b90506020020135600660008b815260200190815260200160002060010160008a8a85818110610816576108166196ad565b90506020020135815260200190815260200160002081905550858582818110610841576108416196ad565b905060200201358361085391906196d9565b925061085e816196f1565b90506107cb565b50600088815260066020526040908190208390555188907f39480b3d2bac27737f11346dee1d48b8e82f5f7079dd9ce6a66725eff0fc9143906108af908a908a908a908a90619740565b60405180910390a2506001979650505050505050565b60006108cf61221b565b90506109076040518060400160405280600b81526020016a68656c6c6f68656c6c6f3f60a81b8152508261224090919063ffffffff16565b9050600061091482612352565b905061094861092282619772565b7f10e6acb2cfcc4bb07588ad5b8e85f6a13f19e24f3302826effd93ce1ebbece6e612381565b5050565b60003361095a818585612461565b60019150505b92915050565b6000806109738484612585565b949350505050565b60405163c88a5e6d60e01b8152600080516020619fcf8339815191529063c88a5e6d906109ae9085908590600401619796565b600060405180830381600087803b1580156109c857600080fd5b505af11580156109dc573d6000803e3d6000fd5b50506040516303223eab60e11b81526001600160a01b0385166004820152600080516020619fcf83398151915292506306447d5691506024015b600060405180830381600087803b158015610a3057600080fd5b505af1158015610a44573d6000803e3d6000fd5b505050505050565b600080610a5a858585612590565b9150505b9392505050565b60405163c88a5e6d60e01b8152600080516020619fcf8339815191529063c88a5e6d90610a9c908490600160801b90600401619796565b600060405180830381600087803b158015610ab657600080fd5b505af1158015610aca573d6000803e3d6000fd5b505060405163ca669fa760e01b81526001600160a01b0384166004820152600080516020619fcf833981519152925063ca669fa791506024015b600060405180830381600087803b158015610b1e57600080fd5b505af1158015610b32573d6000803e3d6000fd5b5050505050565b600033610b478582856127a6565b610b5285858561281a565b506001949350505050565b60408051607d60f91b60208201528151600181830301815260219091019091526000610b8982826129e8565b9050610b96816001612aa7565b604080516001630404ff0160d81b031960208201528151600581830301815260259091019091526000610bca8260016129e8565b9050610bd7816003612aa7565b6040805160016603fffffeff000160b81b031960208201528151600981830301815260299091019091526000610c0e8260026129e8565b9050610c1b816005612aa7565b604080517ffffffffd0000000001000000ffffff000000000000000000000000000000000060208201528151600f818303018152602f9091019091526000610c648260036129e8565b9050610c71816009612aa7565b604080517ffffffffffe00000000000000000100000000000000ffffffff0000000000000060208201528151601981830301815260399091019091526000610cba8260046129e8565b9050610cc7816011612aa7565b50505050505050505050565b60405163c88a5e6d60e01b8152600080516020619fcf8339815191529063c88a5e6d90610d0a908590600160801b90600401619796565b600060405180830381600087803b158015610d2457600080fd5b505af1158015610d38573d6000803e3d6000fd5b50506040516323f2866760e11b81526001600160a01b03808616600483015284166024820152600080516020619fcf83398151915292506347e50cce9150604401610a16565b604051638d1cc92560e01b81526000908190600080516020619fcf83398151915290638d1cc92590610db4908790600401619040565b6000604051808303816000875af1158015610dd3573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052610dfb91908101906197af565b83604051602001610e0d92919061981c565b60405160208183030381529060405290508051602082016000f0949350505050565b600080516020619fcf83398151915263e5d6bf02610e4d834261984b565b6040518263ffffffff1660e01b8152600401610b0491815260200190565b60003361095a818585610e7e8383611ff9565b610e8891906196d9565b612461565b60405163c88a5e6d60e01b8152600080516020619fcf8339815191529063c88a5e6d90610ec09086908590600401619796565b600060405180830381600087803b158015610eda57600080fd5b505af1158015610eee573d6000803e3d6000fd5b50506040516308b6ac0f60e31b81526001600160a01b03808716600483015285166024820152600080516020619fcf83398151915292506345b5607891506044015b600060405180830381600087803b158015610f4a57600080fd5b505af1158015610f5e573d6000803e3d6000fd5b50505050505050565b60008381526005602052604081208054829190610f8390619673565b80601f0160208091040260200160405190810160405280929190818152602001828054610faf90619673565b8015610ffc5780601f10610fd157610100808354040283529160200191610ffc565b820191906000526020600020905b815481529060010190602001808311610fdf57829003601f168201915b50505050509050805160000361102d57604051633709061d60e11b8152600481018690526024015b60405180910390fd5b600061103882612b74565b9050600061107b8a8a8080601f016020809104026020016040519081016040528093929190818152602001838380828437600092019190915250612bc092505050565b90508161108a828a8989612c40565b036110e55760008061109c8c8c612d25565b915091506110aa8183612f3d565b604051899084907ff0b34c36ce72539e8b1154718e3abefe1cedceb1fd30727d0dbc325bca1079a490600090a36001955050505050506110fe565b604051633102cbe560e11b815260040160405180910390fd5b9695505050505050565b6000611115846002619862565b821461113e57604051638d08a59d60e01b81526004810185905260248101839052604401611024565b600061114a8888612585565b600081815260056020526040902080549192509061116790619673565b15905061118a57604051632a3567ab60e01b815260048101829052602401611024565b600060068161119c62030d4085619897565b81526020810191909152604001600090812054915080805b888110156112e457600660006111cd62030d4088619897565b815260200190815260200160002060010160008b8b848181106111f2576111f26196ad565b9050602002013581526020019081526020016000205491506000821180156112c257506112c28a8a8381811061122a5761122a6196ad565b9050602002013589898460026112409190619862565b81811061124f5761124f6196ad565b905060200201358a8a8560026112659190619862565b6112709060016196d9565b81811061127f5761127f6196ad565b905060200201358f8f8080601f01602080910402602001604051908101604052809392919081815260200183838082843760009201919091525061301c92505050565b156112d4576112d182846196d9565b92505b6112dd816196f1565b90506111b4565b5060036112f2846002619862565b6112fc9190619897565b8211611325576040516378f4355360e11b81526004810183905260248101849052604401611024565b600084815260056020526040902061133e908c8c618d98565b5060405184907f6eec8ede03d7cffe8c856c9375d84dc4b6eba3a8286c092df68fc582422e19b890600090a25060019a9950505050505050505050565b60006113a96040518060400160405280600b81526020016a68656c6c6f68656c6c6f2160a81b815250614851565b90506113e16040518060400160405280600b81526020016a68656c6c6f68656c6c6f3f60a81b8152508261224090919063ffffffff16565b905060006113ee82612352565b90506109486113fc82619772565b7f0edd7e645d2bc1bba1f323f6339a3d0448ec6b675991e8dc76d2396eb0dffca2612381565b6005602052600090815260409020805461143b90619673565b80601f016020809104026020016040519081016040528092919081815260200182805461146790619673565b80156114b45780601f10611489576101008083540402835291602001916114b4565b820191906000526020600020905b81548152906001019060200180831161149757829003601f168201915b505050505081565b60096114c88180612aa7565b50565b6000806000806114db8686612d25565b909450925050505b9250929050565b6114f78383836000611660565b505050565b60405163c88a5e6d60e01b8152600080516020619fcf8339815191529063c88a5e6d90611533908490600160801b90600401619796565b600060405180830381600087803b15801561154d57600080fd5b505af1158015611561573d6000803e3d6000fd5b50506040516303223eab60e11b81526001600160a01b0384166004820152600080516020619fcf83398151915292506306447d569150602401610b04565b671d3cd4f726deca9060006115b762030d4083619897565b9050826000805b82811015611637576115d181601e6196d9565b6000858152600660205260408120600101908989858181106115f5576115f56196ad565b9050602002013581526020019081526020016000208190555080601e61161b91906196d9565b61162590836196d9565b9150611630816196f1565b90506115be565b506000928352600660205260409092209190915550505050565b60606004805461074290619673565b604080516001600160a01b0385811660248084019190915283518084039091018152604490920183526020820180516001600160e01b03166370a0823160e01b17905291516000928716916116b4916198ab565b6000604051808303816000865af19150503d80600081146116f1576040519150601f19603f3d011682016040523d82523d6000602084013e6116f6565b606091505b5091505060008180602001905181019061171091906198c7565b905061177c846117768761174d6370a0823160e01b61173060088d614877565b9060038201805463ffffffff191660e09290921c91909117905590565b90600282018054600181018255600091825260209091206001600160a01b039290921691015590565b9061489c565b8215610a445760408051600481526024810182526020810180516001600160e01b03166318160ddd60e01b17905290516000916001600160a01b038916916117c491906198ab565b6000604051808303816000865af19150503d8060008114611801576040519150601f19603f3d011682016040523d82523d6000602084013e611806565b606091505b5091505060008180602001905181019061182091906198c7565b90508286101561184557611834868461984b565b61183e908261984b565b905061185c565b61184f838761984b565b61185990826196d9565b90505b611876816117766318160ddd60e01b61173060088d614877565b5050505050505050565b604051638d1cc92560e01b81526000908190600080516020619fcf83398151915290638d1cc925906118b6908690600401619040565b6000604051808303816000875af11580156118d5573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526118fd91908101906197af565b90508051602082016000f09392505050565b6000818152600560205260409020611928908484618d98565b50505050565b6000338161193c8286611ff9565b90508381101561199c5760405162461bcd60e51b815260206004820152602560248201527f45524332303a2064656372656173656420616c6c6f77616e63652062656c6f77604482015264207a65726f60d81b6064820152608401611024565b610b528286868403612461565b565b60003361095a81858561281a565b60405163c88a5e6d60e01b8152600080516020619fcf8339815191529063c88a5e6d906119ec9086908590600401619796565b600060405180830381600087803b158015611a0657600080fd5b505af1158015611a1a573d6000803e3d6000fd5b50506040516323f2866760e11b81526001600160a01b03808716600483015285166024820152600080516020619fcf83398151915292506347e50cce9150604401610f30565b6000604051602001611aa4907f54686520666f756e646174696f6e206f6620612074727573746c65737320496e8152651d195c9b995d60d21b602082015260260190565b60408051601f1981840301815291905290507fd82042fffbb34d09630aa9c56a2c3f0f2be196f28aaea9cc7332b509c7fc69da7f8854ac521549d8d45d1743d187d8da9ea15d7ece91d0024cac14ad344a0206e27f0101137835043d999fe08b6e946cf5f120a5eaa10681dfa698c963d4ba65220c6000611b278484848861301c565b9050610b32816148a6565b600080516020619fcf83398151915263e5d6bf02610e4d83426196d9565b600754600090610100900460ff1615611b725750600754610100900460ff1690565b6000600080516020619fcf8339815191523b15611c7057604051600090600080516020619fcf833981519152907f667f9d70ca411d70ead50d8d5c22070dafc36ad75f3dcf5e7237b22ade9aecc490611bda9083906519985a5b195960d21b90602001619796565b60408051601f1981840301815290829052611bf892916020016198e0565b60408051601f1981840301815290829052611c12916198ab565b6000604051808303816000865af19150503d8060008114611c4f576040519150601f19603f3d011682016040523d82523d6000602084013e611c54565b606091505b5091505080806020019051810190611c6c9190619911565b9150505b919050565b33611c808184614908565b8183826001600160a01b03167f52916471973ae53f679d702015168c0a34628d9d95a48de6bd2093661e39a7c360405160405180910390a4505050565b60405163c88a5e6d60e01b8152600080516020619fcf8339815191529063c88a5e6d90610a169085908590600401619796565b60405163c88a5e6d60e01b8152600080516020619fcf8339815191529063c88a5e6d90611d27908590600160801b90600401619796565b600060405180830381600087803b158015611d4157600080fd5b505af1158015611d55573d6000803e3d6000fd5b50506040516308b6ac0f60e31b81526001600160a01b03808616600483015284166024820152600080516020619fcf83398151915292506345b560789150604401610a16565b604080517fff6b91090007737cd4cc72ac2067ab3441218f0977d00039c2363867bafd2e4460208201527ff4fda84c8c112efd7da407a7bbab3660ca201e02b3ac54ea0775839e2fb4b4f6818301527ff458ebef7d1bb11fff52cd0b0d522541a034493c8bce35d5c78616da0644b75860608201527f8980bc3fd95e678b2155cc31bac5a1ce87db5f32c719f5209984d6aea258298160808201527f0b153d97ddb22b004f9efec8ffe0630521d94ec973dea0a1369884fec037ff4760a08201527fba4c2d0ba0167d711026711ffe026c833667f9a7602473a7b5053d4d3798d76860c08201527f161cc8276a1dcfcf68a4b63b85f9960ef20792d8260e16eb93620066c905bba060e08201527f71d65be9bc30b11a68a0819886d2ce85b9414e00719a706a77d8bc0772000000610100820152815160fd81830301815261011d9091019091526000611eed82612b74565b9050610948817fcc31bac5a1ce87db5f32c719f5209984d6aea25829810b153d97ddb22b004f9e612381565b7f42b963ace015abb7c727e77dec115c5f418f4f96d9ec897f85c202793274661a604051611f9e906020808252603b908201527f54686520607469706020737464636865617420686173206265656e206465707260408201527f6563617465642e2055736520606465616c6020696e73746561642e0000000000606082015260800190565b60405180910390a16114f7816117768461174d6370a0823160e01b61173060088a614877565b6000858152600560205260408120611fdd908989618d98565b506000611fec86868686612c40565b9998505050505050505050565b6001600160a01b03918216600090815260016020908152604080832093909416825291909152205490565b60408051670123456789abcdef60c01b602082015281516008818303018152602890910190915260026005606061205c848484614a56565b6040516245678960e81b602082015290915061208c9082906023015b604051602081830303815290604052614c7b565b600792506000915061209f848484614a56565b60405166efcdab8967452360c81b6020820152909150611928908290602701612078565b60405163c88a5e6d60e01b8152600080516020619fcf8339815191529063c88a5e6d906120f69085908590600401619796565b600060405180830381600087803b15801561211057600080fd5b505af1158015612124573d6000803e3d6000fd5b505060405163ca669fa760e01b81526001600160a01b0385166004820152600080516020619fcf833981519152925063ca669fa79150602401610a16565b6119a961219660405160200161218290636e6f646560e01b815260040190565b604051602081830303815290604052614d50565b7f7b568d1038ae40d3683670f02841d47a11794b6a629c2c02fedd5856e868cc2b612381565b6119a96121f56040516020016121e1906864617461626c6f636b60b81b815260090190565b604051602081830303815290604052612bc0565b7f6ccea12fef78d2af66a4bca268cdbeccc47b3ee3ec9fbf83da1a67b526e9da2e612381565b612223618e1c565b600061222d614d8b565b905061223a816000614df1565b91505090565b612248618e1c565b60005b82518163ffffffff16101561234a5783516104009061226990614e46565b63ffffffff16036122c757600061228b6122868660000151614e6b565b614eea565b8551602001519091506000906122a290600161992e565b90506122af868383614f20565b6122c28660200151828860800151614f65565b865250505b60006122d68560000151614e46565b6122e290610400619950565b90506000612302828463ffffffff1687516122fd919061984b565b614fcc565b905060006123268663ffffffff861661231b8786619975565b63ffffffff16614ff0565b90506123368760000151826150b7565b6123408285619975565b935050505061224b565b509192915050565b60408051602080825281830190925260609160009190602082018180368337019050509050610960838261525b565b80821461094857600080516020619faf8339815191526040516123e39060208082526025908201527f4572726f723a2061203d3d2062206e6f7420736174697366696564205b627974604082015264657333325d60d81b606082015260800190565b60405180910390a17fafb795c9c61e4fe7468c386f925d7a5429ecad9c0495ddb8d38d690614d32f998160405161241a9190619994565b60405180910390a17fafb795c9c61e4fe7468c386f925d7a5429ecad9c0495ddb8d38d690614d32f998260405161245191906199cc565b60405180910390a16109486152dc565b6001600160a01b0383166124c35760405162461bcd60e51b8152602060048201526024808201527f45524332303a20617070726f76652066726f6d20746865207a65726f206164646044820152637265737360e01b6064820152608401611024565b6001600160a01b0382166125245760405162461bcd60e51b815260206004820152602260248201527f45524332303a20617070726f766520746f20746865207a65726f206164647265604482015261737360f01b6064820152608401611024565b6001600160a01b0383811660008181526001602090815260408083209487168084529482529182902085905590518481527f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925910160405180910390a3505050565b600080610973848460215b6000808483856125a18260016196d9565b926125ae939291906199f6565b6125b791619a20565b9050600060fb60f81b6001600160f81b0319831610156125dc575060f881901c610a5a565b6001600160f81b0319821660fb60f81b036126515761263e86868080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152506126389250889150600290506196d9565b86614a56565b61264790619a50565b60f01c9050610a5a565b6001600160f81b03198216603f60fa1b036126c0576126ad86868080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152506126389250889150600490506196d9565b6126b690619a87565b60e01c9050610a5a565b6001600160f81b0319821660fd60f81b0361272f5761271c86868080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152506126389250889150600890506196d9565b61272590619aba565b60c01c9050610a5a565b6001600160f81b03198216607f60f91b0361279e5761278b86868080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152506126389250889150601090506196d9565b61279490619aed565b60801c9050610a5a565b610a5a619b20565b60006127b28484611ff9565b90506000198114611928578181101561280d5760405162461bcd60e51b815260206004820152601d60248201527f45524332303a20696e73756666696369656e7420616c6c6f77616e63650000006044820152606401611024565b6119288484848403612461565b6001600160a01b03831661287e5760405162461bcd60e51b815260206004820152602560248201527f45524332303a207472616e736665722066726f6d20746865207a65726f206164604482015264647265737360d81b6064820152608401611024565b6001600160a01b0382166128e05760405162461bcd60e51b815260206004820152602360248201527f45524332303a207472616e7366657220746f20746865207a65726f206164647260448201526265737360e81b6064820152608401611024565b6001600160a01b038316600090815260208190526040902054818110156129585760405162461bcd60e51b815260206004820152602660248201527f45524332303a207472616e7366657220616d6f756e7420657863656564732062604482015265616c616e636560d01b6064820152608401611024565b6001600160a01b0380851660009081526020819052604080822085850390559185168152908120805484929061298f9084906196d9565b92505081905550826001600160a01b0316846001600160a01b03167fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef846040516129db91815260200190565b60405180910390a3611928565b600080612a0084846129fb8160016196d9565b614a56565b612a0990619b36565b9050600060fb60f81b6001600160f81b031983161015612a2b57506001610973565b6001600160f81b0319821660fb60f81b03612a4857506003610973565b6001600160f81b03198216603f60fa1b03612a6557506005610973565b6001600160f81b0319821660fd60f81b03612a8257506009610973565b6001600160f81b03198216607f60f91b03612a9f57506011610973565b610973619b20565b80821461094857600080516020619faf833981519152604051612b069060208082526022908201527f4572726f723a2061203d3d2062206e6f7420736174697366696564205b75696e604082015261745d60f01b606082015260800190565b60405180910390a17fb2de2fbe801a0df6c0cbddfd448ba3c41d48a040ca35c56c8196ef0fcae721a881604051612b3d9190619994565b60405180910390a17fb2de2fbe801a0df6c0cbddfd448ba3c41d48a040ca35c56c8196ef0fcae721a88260405161245191906199cc565b6000602181612b8384836129e8565b9050612b908160406196d9565b612b9a90836196d9565b91506000612bae85846129fb8160206196d9565b612bb790619772565b95945050505050565b60405160009081612c0f612bfb7fc811f2ef6eb6bd09fb973c747cbf349e682393ca4d8df88e5f0bcd564c10a84b6020850190815260200190565b604051602081830303815290604052614851565b9050612c1b8185612240565b90506000612c2882612352565b612c3190619772565b60409390935250909392505050565b6000846060825b84811015612d1957612c5a600288619b69565b600003612cab5782868683818110612c7457612c746196ad565b90506020020135604051602001612c95929190918252602082015260400190565b6040516020818303038152906040529150612cf1565b858582818110612cbd57612cbd6196ad565b9050602002013583604051602001612cdf929190918252602082015260400190565b60405160208183030381529060405291505b612cfc600288619897565b9650612d0782614d50565b9250612d12816196f1565b9050612c47565b50909695505050505050565b600080600181612d36868684612590565b9050612d7986868080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152508692506129e8915050565b612d8390836196d9565b915060005b81811015612df857612dd187878080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152508792506129e8915050565b612ddc9060206196d9565b612de690846196d9565b9250612df1816196f1565b9050612d88565b50612e3a86868080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152508692506129e8915050565b612e459060206196d9565b612e4f90836196d9565b91506000612e5e878785612590565b9050612ea187878080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152508792506129e8915050565b612eac9060026196d9565b612eb690846196d9565b9250612ef987878080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152508792506129e8915050565b612f0390846196d9565b92506000878488612f158260146196d9565b92612f22939291906199f6565b612f2b91619b7d565b9195505060601c925050509250929050565b6001600160a01b038216612f935760405162461bcd60e51b815260206004820152601f60248201527f45524332303a206d696e7420746f20746865207a65726f2061646472657373006044820152606401611024565b8060026000828254612fa591906196d9565b90915550506001600160a01b03821660009081526020819052604081208054839290612fd29084906196d9565b90915550506040518181526001600160a01b038316906000907fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef9060200160405180910390a35050565b8051600090819081906040016001600160401b0381111561303f5761303f6192a8565b6040519080825280601f01601f191660200182016040528015613069576020820181803683370190505b50905060005b60208110156130bf5786816020811061308a5761308a6196ad565b1a60f81b8282815181106130a0576130a06196ad565b60200101906001600160f81b031916908160001a90535060010161306f565b5060005b6020811015613116578781602081106130de576130de6196ad565b1a60f81b8282602001815181106130f7576130f76196ad565b60200101906001600160f81b031916908160001a9053506001016130c3565b5060005b845181101561317457848181518110613135576131356196ad565b602001015160f81c60f81b828260400181518110613155576131556196ad565b60200101906001600160f81b031916908160001a90535060010161311a565b506000613180826153d5565b9050600060c082600360200201516001600160401b0316901b608083600260200201516001600160401b0316901b604084600160200201516001600160401b0316901b84600060200201516001600160401b031617171790506008817fff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff0016901c6008827eff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff16901b1790506010817fffff0000ffff0000ffff0000ffff0000ffff0000ffff0000ffff0000ffff000016901c6010827dffff0000ffff0000ffff0000ffff0000ffff0000ffff0000ffff0000ffff16901b1790506020817bffffffff00000000ffffffff00000000ffffffff00000000ffffffff1916901c6020827bffffffff00000000ffffffff00000000ffffffff00000000ffffffff16901b179050600060c0836007600881106132d9576132d96196ad565b60200201516001600160401b0316901b608084600660200201516001600160401b0316901b604085600560200201516001600160401b0316901b85600460200201516001600160401b031617171790506008817fff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff0016901c6008827eff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff16901b1790506010817fffff0000ffff0000ffff0000ffff0000ffff0000ffff0000ffff0000ffff000016901c6010827dffff0000ffff0000ffff0000ffff0000ffff0000ffff0000ffff0000ffff16901b1790506020817bffffffff00000000ffffffff00000000ffffffff00000000ffffffff1916901c6020827bffffffff00000000ffffffff00000000ffffffff00000000ffffffff16901b1790506f14def9dea2f79cd65812631a5cf5d3ed600160fc1b018061343657613436619881565b6f14def9dea2f79cd65812631a5cf5d3ed600160fc1b017f0ffffffffffffffffffffffffffffffec6ef5bf4737dcf70d6ec31748d98951d8309830867ffffffff0000000063ffffffff60a01b017dff000000ff000000ff000000ff000000ff000000ff000000ff000000ff0060088d811b9182167cff000000ff000000ff000000ff000000ff000000ff000000ff000000ff9e90911c9d8e1617601090811b7fff000000ff000000ff000000ff000000ff000000ff000000ff000000ff0000009092167eff000000ff000000ff000000ff000000ff000000ff000000ff000000ff0000909e169d909d17909c1c9b909b17602081811b9c8d1663ffffffff63ffffffff60801b019290911c91821617604090811b600163ffffffff60601b01600160e01b0319909d1663ffffffff60401b63ffffffff60c01b0190921691909117901c9a909a17608081811b91901c17999450506001600160ff1b0389169250600091508190506013600160ff1b03838409905060006013600160ff1b036014600160ff1b038308905060006013600160ff1b037f52036cee2b6ffe738cc740797779e89800700a4d4141d8ab75eb4dca135978a38409600101905060006013600160ff1b03828409905061360b81615c8f565b5094506013600160ff1b0385860994506013600160ff1b0380826013600160ff1b0388890909840994506013600160ff1b03826013600160ff1b038788090990508281146136a157826013600160ff1b03038114613673576000975050505050505050610973565b6013600160ff1b037f2b8324804fc1df0b2b4d00993dfbd7a72f431806ad2fe478c4ee1b274a0ea0b0860994505b5050505060ff88901c60018216146136bd576013600160ff1b03035b67ffffffff0000000063ffffffff60a01b017dff000000ff000000ff000000ff000000ff000000ff000000ff000000ff00600888811b9182167cff000000ff000000ff000000ff000000ff000000ff000000ff000000ff9990911c98891617601090811b7fff000000ff000000ff000000ff000000ff000000ff000000ff000000ff0000009092167eff000000ff000000ff000000ff000000ff000000ff000000ff000000ff00009099169890981790971c96909617602081811b97881663ffffffff63ffffffff60801b019290911c91821617604090811b600163ffffffff60601b01600160e01b031990981663ffffffff60401b63ffffffff60c01b0190921691909117901c95909517608081811b91901c17946f14def9dea2f79cd65812631a5cf5d3ed600160fc1b0186106137fc5760009350505050610973565b60008060008061380a618e5d565b8587016013600160ff1b038789038101906000907f2406d9dc56dffce7198e80f2eef3d13000e0149a8283b156ebd69b9426b2f1596013600160ff1b038c8c09099050898960018060006013600160ff1b03828509905060006013600160ff1b03848709905060006013600160ff1b03848609905060006013600160ff1b03848509905060006013600160ff1b03848509905060006013600160ff1b03858709905060006013600160ff1b03858609905081820199508383036013600160ff1b030198508284019a506013600160ff1b03806138e8576138e8619881565b89602519038283010897505050505050505060006013600160ff1b038061391157613911619881565b828509905060006013600160ff1b03848709905060006013600160ff1b03848609905060006013600160ff1b03848509905060006013600160ff1b03848509905060006013600160ff1b03858709905060006013600160ff1b03858609905081820199508383036013600160ff1b030198508284019a506013600160ff1b038061399d5761399d619881565b89602519038283010897505050505050505060006013600160ff1b03806139c6576139c6619881565b828509905060006013600160ff1b03848709905060006013600160ff1b03848609905060006013600160ff1b03848509905060006013600160ff1b03848509905060006013600160ff1b03858709905060006013600160ff1b03858609905081820199508383036013600160ff1b030198508284019a506013600160ff1b0380613a5257613a52619881565b8960251903828301089750600196508e955060009450505050505b6000808080806013600160ff1b03898c09905060006013600160ff1b038b8e09905060006013600160ff1b038b8d0990506013600160ff1b038e8e0960208a0151518484019850939092036013600160ff1b030195509093508001915084908660088110613add57613add6196ad565b6020020152828660016020020151600160200201518660088110613b0357613b036196ad565b60200201526013600160ff1b037f2406d9dc56dffce7198e80f2eef3d13000e0149a8283b156ebd69b9426b2f15983096020870151604001518660088110613b4d57613b4d6196ad565b602002015285515181908660088110613b6857613b686196ad565b6020020152868660006020020151600160200201518660088110613b8e57613b8e6196ad565b60200201526013600160ff1b03818809965084600703613bb15750505050613c34565b60006013600160ff1b038f8609905060006013600160ff1b038f8609905060006013600160ff1b038f860990508183036013600160ff1b03019c506013600160ff1b0380613c0157613c01619881565b8185088383019e509b506013600160ff1b03816013600160ff1b030385089a505060019096019550613a6d945050505050565b506000613c4083615c8f565b90935090506013600160ff1b0383840992506013600160ff1b0383840992506013600160ff1b0383840992506013600160ff1b0383840992506013600160ff1b0383840992506013600160ff1b03818409925060075b8251602001516000906013600160ff1b03908360088110613cb957613cb96196ad565b602002015186096020850151519091506013600160ff1b039082908460088110613ce557613ce56196ad565b6020020151096020850151518360088110613d0257613d026196ad565b60200201526013600160ff1b03818560016020020151600160200201518460088110613d3057613d306196ad565b60200201510960208086015101518360088110613d4f57613d4f6196ad565b60200201526013600160ff1b03818560016020020151600260200201518460088110613d7d57613d7d6196ad565b6020020151096020850151604001518360088110613d9d57613d9d6196ad565b60200201526000829003613db15750613de1565b8351516013600160ff1b03908360088110613dce57613dce6196ad565b6020020151860994505060001901613c96565b5060405180606001604052806040518061010001604052807f43e7ce9d19ea5d329385a44c321ea16167c996e37dc6070c97de49e37ac61db981526020017f40cff34425d8ec30a3bb74ba58cd5854fa1e38186ad0d31ebc8ae251ceb2c97e81526020017f459bd27046e8dd45aea7008db87a5a8f7906779253d64523589518599fdfbf4b81526020017f69fdd1e28c23cc3894d0c8ff90e76f6d5b6e4c2e620136d04dd83c4a51581ab981526020017f54dceb3413ce5cfa11196dfc960b6edaf4b380c6d4d2378419cc0279ba49c5f381526020017f4e24184dd71a3d77eef3729f7f8cf7c17224cf40aa7b9548b9942f3c5084ceed81526020017f5a0e5aab20262674ae1175761cbf5e889b52a55fd7ac5027c228cebdc8d2360a81526020017f26239334073e9b38c62859556d451c3dcc8d30e84b361174f488eadde2cf17d981525081526020016040518061010001604052807f227e97c94c7c0933d2e0c21a3447c504fe9ccf82e8a05f59ce881c82eba0489f81526020017f226a3e0ecc4afec6fd0d288413014a9dbddecf06c1a2f0bb702ba77c613d820981526020017f34d7efc851d45c5e71efeb0f235b794691de6228877569b3a8d52bf058b8a4a081526020017f3c1f5fb3ca7166fce1471c9b752b6d28c56301ad7b65e8451b2c8c5526726e1281526020017f6102416cf02f02ff5be75275f55f28db89b2a9d2456b860ce22fc0e5031f7cc581526020017f40adf677f1bfdae057f0fd179c12617918ddaa2891a6530fb1a4294fa866549081526020017f61936f3c415609046187b8baa978cbc9b47893363ae5a3cc7d909f3635ae7f4881526020017f562a9662b6ec47f9e979d473c02b51e4423368238c58ddb52f0e5c6a180e641081525081526020016040518061010001604052807f3788bdb44f8632d42d0dbee5eea1acc6136cf411e655624f55e48902c3bd553481526020017f6190cf2c2a7b5ad769d594a82844f23b4167fa7c8ac30e51aa6cfbebdcd4b94581526020017f65f7787096be9204123a71f3ac88a87be1513217737d6a1e2f3a13a43d7e3a9a81526020017f023af32dbfa67975536479a7a7ce74a02142147fac0480187f1f13349cda1f2d81526020017f64fc44b7fc6841bddb0ced8b8b0fe6759137ef87ee96651215fc1dbcd25c64dc81526020017f1434aa3748b701d5b69df3d7d340c1fe3f6b9c1efc617484caadb47e382f447581526020017f457a6da8c962ef35f2b217423e5844e9d23534527e8ea4290d24e3ddf21720c681526020017f63b9540ceb60ccb51e4d989d956e053cf2511837efb79089d2ff40284202c53d815250815250826000600281106141bb576141bb6196ad565b602002018190525050505050505050505050600060038c60001c901b90506000897f80000000000000000000000000000000a6f7cef517bce6b2c09318d2e7ae9f6001905060008060019050600060019050600060019050600060fc90505b6008811b8781161561435c576000808080806013600160ff1b03888c09905060006013600160ff1b038b8b09905081810195508181036013600160ff1b030194506013600160ff1b038061427057614270619881565b898c0993506013600160ff1b038a8d09925050506000868d901c6007169050866007901b198d169c5060008e905060006013600160ff1b03806142b5576142b5619881565b82516020015184600881106142cc576142cc6196ad565b602002015187098251519091506000906013600160ff1b039085600881106142f6576142f66196ad565b602002015189098351604001519091506000906013600160ff1b03908660088110614323576143236196ad565b6020020151870990508282036013600160ff1b03019e508087019d508282019c508087036013600160ff1b03019b505050505050505050505b86811615614495576000808080806013600160ff1b03888c09905060006013600160ff1b038b8b09905081810195508181036013600160ff1b030194506013600160ff1b03806143ae576143ae619881565b898c0993506013600160ff1b038a8d096007808a1b198f169e91945090891c1691508e905060006013600160ff1b0382600160200201516000602002015184600881106143fd576143fd6196ad565b6020020151870960208084015101519091506000906013600160ff1b0390856008811061442c5761442c6196ad565b602002015189096020840151604001519091506000906013600160ff1b0390866008811061445c5761445c6196ad565b6020020151870990508282036013600160ff1b03019e508087036013600160ff1b03019d508282019c508087019b505050505050505050505b816000036145c7576000808080806013600160ff1b03888c09905060006013600160ff1b038b8b09905081810195508181036013600160ff1b030194506013600160ff1b03806144e7576144e7619881565b898c0993506013600160ff1b038a8d099250505060078b168d60006013600160ff1b03826001602002015160006020020151846008811061452a5761452a6196ad565b6020020151870960208084015101519091506000906013600160ff1b03908560088110614559576145596196ad565b602002015189096020840151604001519091506000906013600160ff1b03908660088110614589576145896196ad565b6020020151870990508282036013600160ff1b03019e508087036013600160ff1b03019d508282019c508087019b505050505050505050505061467d565b60006013600160ff1b03848809905060006013600160ff1b03878709905060006013600160ff1b03868909905060006013600160ff1b03848509905060006013600160ff1b03848509905060006013600160ff1b03858709905060006013600160ff1b0385860990508182019c508383036013600160ff1b03019b508284019a506013600160ff1b038061465d5761465d619881565b8c6025190382830108995050600019909701965061421a95505050505050565b5092995090975095509350600091508190506146a26013600160ff1b03858809615c8f565b915091506013600160ff1b03806146bb576146bb619881565b82830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0381830991506013600160ff1b0380858409880996506013600160ff1b0380878409860967ffffffff0000000063ffffffff60a01b017dff000000ff000000ff000000ff000000ff000000ff000000ff000000ff0060ff9990991b909117600881811b998a167cff000000ff000000ff000000ff000000ff000000ff000000ff000000ff9290911c91821617601090811b7fff000000ff000000ff000000ff000000ff000000ff000000ff000000ff000000909a167eff000000ff000000ff000000ff000000ff000000ff000000ff000000ff000090921691909117901c97909717602081811b98891663ffffffff63ffffffff60801b019290911c91821617604090811b600163ffffffff60601b01600160e01b031990991663ffffffff60401b63ffffffff60c01b0190921691909117901c96909617608081811b91901c178d149950505050505050505050949350505050565b614859618e1c565b614861618e8a565b8261486c81836169be565b610973826010614df1565b6005820180546001600160a01b0319166001600160a01b039290921691909117905590565b6109488282616a95565b806114c857600080516020619faf8339815191526040516148f89060208082526017908201527f4572726f723a20417373657274696f6e204661696c6564000000000000000000604082015260600190565b60405180910390a16114c86152dc565b6001600160a01b0382166149685760405162461bcd60e51b815260206004820152602160248201527f45524332303a206275726e2066726f6d20746865207a65726f206164647265736044820152607360f81b6064820152608401611024565b6001600160a01b038216600090815260208190526040902054818110156149dc5760405162461bcd60e51b815260206004820152602260248201527f45524332303a206275726e20616d6f756e7420657863656564732062616c616e604482015261636560f01b6064820152608401611024565b6001600160a01b0383166000908152602081905260408120838303905560028054849290614a0b90849061984b565b90915550506040518281526000906001600160a01b038516907fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef9060200160405180910390a3505050565b8251606090828411614b73578084108015614a6f575060015b8015614a7b5750808311155b614aa95760405163491f466960e11b8152600481018590526024810184905260448101829052606401611024565b6000614ab5858561984b565b90506000816001600160401b03811115614ad157614ad16192a8565b6040519080825280601f01601f191660200182016040528015614afb576020820181803683370190505b50905060005b82811015614b685787614b1482896196d9565b81518110614b2457614b246196ad565b602001015160f81c60f81b828281518110614b4157614b416196ad565b60200101906001600160f81b031916908160001a905350614b61816196f1565b9050614b01565b509250610a5e915050565b8084108015614b80575060015b8015614b8e57506000198312155b614bbc5760405163491f466960e11b8152600481018590526024810184905260448101829052606401611024565b6000614bc8848661984b565b90506000816001600160401b03811115614be457614be46192a8565b6040519080825280601f01601f191660200182016040528015614c0e576020820181803683370190505b50905060005b82811015614b685787614c27828961984b565b81518110614c3757614c376196ad565b602001015160f81c60f81b828281518110614c5457614c546196ad565b60200101906001600160f81b031916908160001a905350614c74816196f1565b9050614c14565b614c858282616dd9565b61094857600080516020619faf833981519152604051614ce29060208082526023908201527f4572726f723a2061203d3d2062206e6f7420736174697366696564205b62797460408201526265735d60e81b606082015260800190565b60405180910390a17fd26e16cad4548705e4c9e2d94f98ee91c289085ee425594fd5635fa2964ccf1882604051614d199190619bb0565b60405180910390a17fd26e16cad4548705e4c9e2d94f98ee91c289085ee425594fd5635fa2964ccf18816040516124519190619bec565b60405160009081612c0f612bfb7fd943cb6e931507cafe2357fbe5cce15af420a84c67251eddb0bf934b7bbbef916020850190815260200190565b614d93618e8a565b506040805161010081018252636a09e667815263bb67ae856020820152633c6ef3729181019190915263a54ff53a606082015263510e527f6080820152639b05688c60a0820152631f83d9ab60c0820152635be0cd1960e082015290565b614df9618e1c565b614e01618ea9565b6040518060a00160405280614e1886600087614f65565b8152602001858152602001828152602001600060ff1681526020018463ffffffff1681525091505092915050565b6000816060015182608001516040614e5e9190619c16565b60ff166109609190619975565b614e73618ed7565b614e7b618f12565b614e89836040015182616e65565b6040518060a001604052808460000151815260200182815260200184602001516001600160401b03168152602001846060015163ffffffff1681526020016002614ed286616f4c565b8660a00151171763ffffffff16815250915050919050565b614ef2618e8a565b6000614f1583600001518460200151856040015186606001518760800151616f6d565b9050610a5e81617275565b60018116600003614f5b57614f47614f37846172ec565b8385602001518660800151617338565b915060011c677fffffffffffffff16614f20565b6114f7838361734f565b614f6d618f31565b6040805181815260608101825260009160208201818036833750506040805160c0810182529788526001600160401b03969096166020880152509385019390935250600060608401819052608084015263ffffffff1660a08301525090565b60008163ffffffff168363ffffffff161015614fe9575081610960565b5080610960565b60606000614ffe848461984b565b6001600160401b03811115615015576150156192a8565b6040519080825280601f01601f19166020018201604052801561503f576020820181803683370190505b50905060005b838110156150ae578561505882876196d9565b81518110615068576150686196ad565b602001015160f81c60f81b828281518110615085576150856196ad565b60200101906001600160f81b031916908160001a905350806150a6816196f1565b915050615045565b50949350505050565b60005b81518163ffffffff1610156114f757604060ff16836060015163ffffffff1603615165576150e6618f12565b6150f4846040015182616e65565b835160208501516151209161511b91849060406151108a616f4c565b8a60a0015117616f6d565b617275565b845260808401805160019190615137908390619c3f565b60ff169052506040805181815260608101825290602082018180368337505050604085015250600060608401525b6060830151600090615178906040619950565b90506000615193828463ffffffff1686516122fd919061984b565b905060005b8163ffffffff168163ffffffff16101561522c57846151b78286619975565b63ffffffff16815181106151cd576151cd6196ad565b602001015160f81c60f81b86604001518760600151836151ed9190619975565b63ffffffff1681518110615203576152036196ad565b60200101906001600160f81b031916908160001a9053508061522481619c64565b915050615198565b50808560600181815161523f9190619975565b63ffffffff169052506152528184619975565b925050506150ba565b600061526a8360000151614e6b565b606084015190915060ff165b63ffffffff8116156152d25761528d600182619950565b90506152cb84604001518263ffffffff16603681106152ae576152ae6196ad565b60200201516152bc84614eea565b86602001518760800151617391565b9150615276565b6119288284617498565b600080516020619fcf8339815191523b156153c457604051600090600080516020619fcf833981519152907f70ca10bbd0dbfd9020a9f4b13402c16cb120705e0d1c0aeab10fa353ae586fc4906153459083906519985a5b195960d21b90600190602001619c87565b60408051601f198184030181529082905261536392916020016198e0565b60408051601f198184030181529082905261537d916198ab565b6000604051808303816000865af19150503d80600081146153ba576040519150601f19603f3d011682016040523d82523d6000602084013e6153bf565b606091505b505050505b6007805461ff001916610100179055565b6153dd618e8a565b6040805161010081018252676a09e667f3bcc908815267bb67ae8584caa73b6020820152673c6ef372fe94f82b9181019190915267a54ff53a5f1d36f1606082015267510e527fade682d16080820152679b05688c2b3e6c1f60a0820152671f83d9abfb41bd6b60c0820152675be0cd19137e217960e0820152600080615462618f6d565b6040805161010081018252600080825260208201819052918101829052606081018290526080810182905260a0810182905260c0810182905260e081019190915260408051610a008101825267428a2f98d728ae228152677137449123ef65cd602082015267b5c0fbcfec4d3b2f9181019190915267e9b5dba58189dbbc6060820152673956c25bf348b53860808201526759f111f1b605d01960a082015267923f82a4af194f9b60c082015267ab1c5ed5da6d811860e082015267d807aa98a30302426101008201526712835b0145706fbe61012082015267243185be4ee4b28c61014082015267550c7dc3d5ffb4e26101608201526772be5d74f27b896f6101808201526780deb1fe3b1696b16101a0820152679bdc06a725c712356101c082015267c19bf174cf6926946101e082015267e49b69c19ef14ad261020082015267efbe4786384f25e3610220820152670fc19dc68b8cd5b561024082015267240ca1cc77ac9c65610260820152672de92c6f592b0275610280820152674a7484aa6ea6e4836102a0820152675cb0a9dcbd41fbd46102c08201526776f988da831153b56102e082015267983e5152ee66dfab61030082015267a831c66d2db4321061032082015267b00327c898fb213f61034082015267bf597fc7beef0ee461036082015267c6e00bf33da88fc261038082015267d5a79147930aa7256103a08201526706ca6351e003826f6103c082015267142929670a0e6e706103e08201526727b70a8546d22ffc610400820152672e1b21385c26c926610420820152674d2c6dfc5ac42aed6104408201526753380d139d95b3df61046082015267650a73548baf63de61048082015267766a0abb3c77b2a86104a08201526781c2c92e47edaee66104c08201526792722c851482353b6104e082015267a2bfe8a14cf1036461050082015267a81a664bbc42300161052082015267c24b8b70d0f8979161054082015267c76c51a30654be3061056082015267d192e819d6ef521861058082015267d69906245565a9106105a082015267f40e35855771202a6105c082015267106aa07032bbd1b86105e08201526719a4c116b8d2d0c8610600820152671e376c085141ab53610620820152672748774cdf8eeb996106408201526734b0bcb5e19b48a861066082015267391c0cb3c5c95a63610680820152674ed8aa4ae3418acb6106a0820152675b9cca4f7763e3736106c082015267682e6ff3d6b2b8a36106e082015267748f82ee5defb2fc6107008201526778a5636f43172f606107208201526784c87814a1f0ab72610740820152678cc702081a6439ec6107608201526790befffa23631e2861078082015267a4506cebde82bde96107a082015267bef9a3f7b2c679156107c082015267c67178f2e372532b6107e082015267ca273eceea26619c61080082015267d186b8c721c0c20761082082015267eada7dd6cde0eb1e61084082015267f57d4f7fee6ed1786108608201526706f067aa72176fba610880820152670a637dc5a2c898a66108a082015267113f9804bef90dae6108c0820152671b710b35131c471b6108e08201526728db77f523047d846109008201526732caab7b40c72493610920820152673c9ebe0a15c9bebc61094082015267431d67c49c100d4c610960820152674cc5d4becb3e42b661098082015267597f299cfc657e2a6109a0820152675fcb6fab3ad6faec6109c0820152676c44198c4a4758176109e0820152600061596089617512565b905060005b608082518161597657615976619881565b04811015615c8157600061598a8383617726565b89516001600160401b0390811687526020808c01518216908801526040808c01518216908801526060808c01518216908801526080808c015182169088015260a0808c015182169088015260c0808c015182169088015260e0808c015190911690870152905060005b6050811015615be6576010811015615a4657818160108110615a1757615a176196ad565b6020020151878260508110615a2e57615a2e6196ad565b6001600160401b039092166020929092020152615ae7565b866010820360508110615a5b57615a5b6196ad565b6020020151615a8288600f840360508110615a7857615a786196ad565b60200201516177b6565b886007840360508110615a9757615a976196ad565b6020020151615abe8a6002860360508110615ab457615ab46196ad565b60200201516177e5565b010101878260508110615ad357615ad36196ad565b6001600160401b0390921660209290920201525b868160508110615af957615af96196ad565b6020020151858260508110615b1057615b106196ad565b6020020151615b3888608001518960a001518a60c001516001600160401b0383181691161890565b615b45896080015161780c565b8960e00151010101019850615b7186600001518760200151886040015180821690831691909216181890565b8651615b7c9061782f565b60c0880180516001600160401b0390811660e08b015260a08a018051821690925260808a018051821690925260608a0180518e01821690925260408a018051821690925260208a01805182169092528951811690915291018a8101909116875297506001016159f3565b5050835188516001600160401b03910181168952602080860151908a01805190910182169052604080860151908a01805190910182169052606080860151908a01805190910182169052608080860151908a0180519091018216905260a080860151908a0180519091018216905260c080860151908a0180519091018216905260e080860151908a0180519091019091169052600101615965565b509598975050505050505050565b6000806013600160ff1b0383840990506013600160ff1b0381820991506013600160ff1b03836013600160ff1b038485090991506013600160ff1b0381830990506013600160ff1b03826013600160ff1b0383840909915060006013600160ff1b0383840990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381840992506013600160ff1b0383840990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b03818409905060006013600160ff1b0382830990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382850993506013600160ff1b0384850991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382850991506013600160ff1b0382830990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381820990506013600160ff1b0381830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382830991506013600160ff1b0382850993505050915091565b602082511115616a2c5760405162461bcd60e51b815260206004820152603360248201527f4461746120627974657320697320746f6f206c6f6e6720746f20636f6e7665726044820152727420746f203820342d6279746520776f72647360681b6064820152608401611024565b60005b60048351616a3d9190619897565b8160ff1610156114f757616a5e83616a56836004619c16565b60ff16617852565b828260ff1660088110616a7357616a736196ad565b63ffffffff909216602092909202015280616a8d81619ca8565b915050616a2f565b600582015460038301546004840154600285018054604080516020808402820181019092528281526001600160a01b039096169560e09590951b9460009390929091830182828015616b0657602002820191906000526020600020905b815481526020019060010190808311616af2575b50505050509050600083616b198361792b565b604051602001616b2a9291906198e0565b60408051601f198184030181528282526001600160a01b038816600090815260018b0160209081528382206001600160e01b03198a168352815292812091945090929091616b7c918691889101619cc7565b60408051601f198184030181529181528151602092830120835290820192909252016000205460ff16616bb457616bb2876179ca565b505b6001600160a01b0385166000908152602088815260408083206001600160e01b0319881684528252808320905190918391616bf3918791899101619cc7565b6040516020818303038152906040528051906020012081526020019081526020016000205460001b9050600080876001600160a01b031684604051616c3891906198ab565b600060405180830381855afa9150503d8060008114616c73576040519150601f19603f3d011682016040523d82523d6000602084013e616c78565b606091505b509150616c91905081616c8c886020619862565b6185ee565b604051630667f9d760e41b815290925060009150600080516020619fcf8339815191529063667f9d7090616ccb908b908790600401619796565b6020604051808303816000875af1158015616cea573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190616d0e91906198c7565b9050808214616d2f5760405162461bcd60e51b815260040161102490619d01565b6040516370ca10bb60e01b8152600080516020619fcf833981519152906370ca10bb90616d64908b9087908e90600401619c87565b600060405180830381600087803b158015616d7e57600080fd5b505af1158015616d92573d6000803e3d6000fd5b50505060058b0180546001600160a01b03191690555060038a01805463ffffffff19169055616dc560028b016000618f8c565b896004016000905550505050505050505050565b805182516001919003616e5c5760005b8351811015616e5657828181518110616e0457616e046196ad565b602001015160f81c60f81b6001600160f81b031916848281518110616e2b57616e2b6196ad565b01602001516001600160f81b03191614616e4457600091505b80616e4e816196f1565b915050616de9565b50610960565b50600092915050565b6040825111158015616e82575060048251616e809190619b69565b155b616eeb5760405162461bcd60e51b815260206004820152603460248201527f4461746120627974657320697320746f6f206c6f6e6720746f20636f6e7665726044820152737420746f20313620342d6279746520776f72647360601b6064820152608401611024565b60005b60048351616efc9190619897565b8160ff1610156114f757616f1583616a56836004619c16565b828260ff1660108110616f2a57616f2a6196ad565b63ffffffff909216602092909202015280616f4481619ca8565b915050616eee565b6000816080015160ff16600003616f6557506001919050565b506000919050565b616f75618f12565b6000616f7f614d8b565b9050616f89618f12565b60005b60108160ff161015616fe957878160ff1660108110616fad57616fad6196ad565b6020020151828260ff1660108110616fc757616fc76196ad565b63ffffffff909216602092909202015280616fe181619ca8565b915050616f8c565b5060006040518061020001604052808a60006008811061700b5761700b6196ad565b6020908102919091015163ffffffff168252018a60016020908102919091015163ffffffff168252018a60026020908102919091015163ffffffff168252018a60036020908102919091015163ffffffff168252018a60046020908102919091015163ffffffff168252018a60056020908102919091015163ffffffff168252018a60066020908102919091015163ffffffff168252018a60076020908102919091015163ffffffff168252018460006020908102919091015163ffffffff168252018460016020908102919091015163ffffffff168252018460026020908102919091015163ffffffff168252018460036020908102919091015163ffffffff90811683528a811683830152908a901c8116604083015288811660608301528716608090910152905061713f818361866b565b6171488261874f565b617152818361866b565b61715b8261874f565b617165818361866b565b61716e8261874f565b617178818361866b565b6171818261874f565b61718b818361866b565b6171948261874f565b61719e818361866b565b6171a78261874f565b6171b1818361866b565b60005b60088160ff16101561726857816171cc826008619c3f565b60ff16601081106171df576171df6196ad565b6020020151828260ff16601081106171f9576171f96196ad565b6020020180519190911863ffffffff1690528960ff821660088110617220576172206196ad565b602002015182617231836008619c3f565b60ff1660108110617244576172446196ad565b6020020180519190911863ffffffff1690528061726081619ca8565b9150506171b4565b5098975050505050505050565b61727d618e8a565b617285618e8a565b60005b60088160ff1610156172e557838160ff16601081106172a9576172a96196ad565b6020020151828260ff16600881106172c3576172c36196ad565b63ffffffff9092166020929092020152806172dd81619ca8565b915050617288565b5092915050565b6172f4618e8a565b6001826060018181516173079190619d77565b60ff90811690915260408401516060850151909250166036811061732d5761732d6196ad565b602002015192915050565b617340618e8a565b612bb761228686868686617391565b808260400151836060015160ff166036811061736d5761736d6196ad565b602002015260608201805160019190617387908390619c3f565b60ff169052505050565b617399618ed7565b6173a1618f12565b60005b60088160ff16101561740157868160ff16600881106173c5576173c56196ad565b6020020151828260ff16601081106173df576173df6196ad565b63ffffffff9092166020929092020152806173f981619ca8565b9150506173a4565b5060085b60108160ff16101561746257858160ff1660088110617426576174266196ad565b6020020151828260ff1660108110617440576174406196ad565b63ffffffff90921660209290920201528061745a81619ca8565b915050617405565b506040805160a081018252948552602085019190915260008482015260608401525060041763ffffffff16608082015292915050565b60006174bb83600001518460200151600086606001516008886080015117616f6d565b905060005b60088163ffffffff16101561192857617500828263ffffffff16601081106174ea576174ea6196ad565b6020020151846174fb846004619d9a565b61883d565b8061750a81619c64565b9150506174c0565b60606000608083516175249190619b69565b61752f90608061984b565b90506070608084516175419190619b69565b1061756357608083516175549190619b69565b6175609061010061984b565b90505b600081845161757291906196d9565b6001600160401b03811115617589576175896192a8565b6040519080825280601f01601f1916602001820160405280156175b3576020820181803683370190505b50905060005b845181101561761a578481815181106175d4576175d46196ad565b602001015160f81c60f81b8282815181106175f1576175f16196ad565b60200101906001600160f81b031916908160001a90535080617612816196f1565b9150506175b9565b50608060f81b81855181518110617633576176336196ad565b60200101906001600160f81b031916908160001a90535060008451600861765a9190619862565b60408051608083901b6001600160801b031916602082015281516010818303018152603090910190915290915060005b815181101561771b578181600184516176a3919061984b565b6176ad919061984b565b815181106176bd576176bd6196ad565b602001015160f81c60f81b8482600187516176d8919061984b565b6176e2919061984b565b815181106176f2576176f26196ad565b60200101906001600160f81b031916908160001a90535080617713816196f1565b91505061768a565b509195945050505050565b61772e618f12565b617736618f12565b60005b60108160ff1610156177ae5761777185617754836008619c16565b60ff16617762876080619862565b61776c91906196d9565b6188c2565b60c01c828260ff1660108110617789576177896196ad565b6001600160401b039092166020929092020152806177a681619ca8565b915050617739565b509392505050565b60006701ffffffffffffff600783901c166177d283600861891f565b6177dd84600161891f565b181892915050565b60006703ffffffffffffff600683901c1661780183603d61891f565b6177dd84601361891f565b600061781982602961891f565b61782483601261891f565b6177dd84600e61891f565b600061783c82602761891f565b61784783602261891f565b6177dd84601c61891f565b600061785f8260046196d9565b835110156178af5760405162461bcd60e51b815260206004820152601f60248201527f6c655f62797465735f6765745f75696e7433325f6f75744f66426f756e6473006044820152606401611024565b6000805b60048160ff1610156177ae576178ca816008619c16565b60ff1685856178da846003619d77565b60ff166178e791906196d9565b815181106178f7576178f76196ad565b0160200151617917916001600160f81b0319909116901c60e01c83619975565b91508061792381619ca8565b9150506178b3565b606060008251602061793d9190619862565b6001600160401b03811115617954576179546192a8565b6040519080825280601f01601f19166020018201604052801561797e576020820181803683370190505b50905060005b83518110156172e55760008482815181106179a1576179a16196ad565b6020026020010151905080826020026020018401525080806179c2906196f1565b915050617984565b600581015460038201546004830154600284018054604080516020808402820181019092528281526000966001600160a01b03169560e01b949387939192909190830182828015617a3a57602002820191906000526020600020905b815481526020019060010190808311617a26575b5050506001600160a01b038716600090815260018a01602090815260408083206001600160e01b03198a1684528252808320905195965094919350617a8492508591879101619cc7565b60408051601f198184030181529181528151602092830120835290820192909252016000205460ff1615617b20576001600160a01b0384166000908152602087815260408083206001600160e01b03198716845282528083209051909291617af0918591879101619cc7565b60405160208183030381529060405280519060200120815260200190815260200160002054945050505050919050565b600083617b2c8361792b565b604051602001617b3d9291906198e0565b6040516020818303038152906040529050600080516020619fef83398151915260001c6001600160a01b031663266cf1096040518163ffffffff1660e01b8152600401600060405180830381600087803b158015617b9a57600080fd5b505af1158015617bae573d6000803e3d6000fd5b50505050600080866001600160a01b031683604051617bcd91906198ab565b600060405180830381855afa9150503d8060008114617c08576040519150601f19603f3d011682016040523d82523d6000602084013e617c0d565b606091505b509150617c21905081616c8c876020619862565b6040516365bc948160e01b81526001600160a01b038916600482015290925060009150600080516020619fcf833981519152906365bc9481906024016000604051808303816000875af1158015617c7c573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052617ca49190810190619e3a565b5090508051600103617f4b576000600080516020619fef83398151915260001c6001600160a01b031663667f9d708984600081518110617ce657617ce66196ad565b60200260200101516040518363ffffffff1660e01b8152600401617d0b929190619796565b6020604051808303816000875af1158015617d2a573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190617d4e91906198c7565b905080617dac577f080fc4a96620c4462e705b23f346413fe3796bb63c6f8d8591baec0e231577a58883600081518110617d8a57617d8a6196ad565b602002602001015160001c604051617da3929190619796565b60405180910390a15b808314617dcb5760405162461bcd60e51b815260040161102490619d01565b7f9c9555b1e3102e3cf48f427d79cb678f5d9bd1ed0ad574389461e255f95170ed88888789604051602001617e01929190619cc7565b6040516020818303038152906040528051906020012085600081518110617e2a57617e2a6196ad565b602002602001015160001c604051617e459493929190619e93565b60405180910390a181600081518110617e6057617e606196ad565b6020908102919091018101516001600160a01b038a1660009081528c835260408082206001600160e01b03198c1683528452808220905192939092617ea9918a918c9101619cc7565b60408051601f1981840301815291815281516020928301208352828201939093529082016000908120939093556001600160a01b038b16835260018d810182528284206001600160e01b03198c16855282528284209251909391617f11918a918c9101619cc7565b60408051808303601f19018152918152815160209283012083529082019290925201600020805460ff1916911515919091179055506184a6565b6001815111156184535760005b815181101561844d576000600080516020619fef83398151915260001c6001600160a01b031663667f9d708a858581518110617f9657617f966196ad565b60200260200101516040518363ffffffff1660e01b8152600401617fbb929190619796565b6020604051808303816000875af1158015617fda573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190617ffe91906198c7565b90508061805b577f080fc4a96620c4462e705b23f346413fe3796bb63c6f8d8591baec0e231577a589848481518110618039576180396196ad565b602002602001015160001c604051618052929190619796565b60405180910390a15b600080516020619fef83398151915260001c6001600160a01b03166370ca10bb8a85858151811061808e5761808e6196ad565b602002602001015161133760f01b6040518463ffffffff1660e01b81526004016180ba93929190619c87565b600060405180830381600087803b1580156180d457600080fd5b505af11580156180e8573d6000803e3d6000fd5b50505050600060608a6001600160a01b03168760405161810891906198ab565b600060405180830381855afa9150503d8060008114618143576040519150601f19603f3d011682016040523d82523d6000602084013e618148565b606091505b50909250905061815d81616c8c8b6020619862565b9550818015618170575061133760f01b86145b156183ab577f9c9555b1e3102e3cf48f427d79cb678f5d9bd1ed0ad574389461e255f95170ed8b8b8a8c6040516020016181ab929190619cc7565b604051602081830303815290604052805190602001208888815181106181d3576181d36196ad565b602002602001015160001c6040516181ee9493929190619e93565b60405180910390a1848481518110618208576182086196ad565b6020908102919091018101516001600160a01b038d1660009081528f835260408082206001600160e01b03198f1683528452808220905192939092618251918d918f9101619cc7565b6040516020818303038152906040528051906020012081526020019081526020016000208190555060018d60010160008d6001600160a01b03166001600160a01b0316815260200190815260200160002060008c6001600160e01b0319166001600160e01b031916815260200190815260200160002060008a8c6040516020016182dc929190619cc7565b60405160208183030381529060405280519060200120815260200190815260200160002060006101000a81548160ff021916908315150217905550600080516020619fef83398151915260001c6001600160a01b03166370ca10bb8c87878151811061834a5761834a6196ad565b6020026020010151866040518463ffffffff1660e01b815260040161837193929190619c87565b600060405180830381600087803b15801561838b57600080fd5b505af115801561839f573d6000803e3d6000fd5b5050505050505061844d565b600080516020619fef83398151915260001c6001600160a01b03166370ca10bb8c8787815181106183de576183de6196ad565b6020026020010151866040518463ffffffff1660e01b815260040161840593929190619c87565b600060405180830381600087803b15801561841f57600080fd5b505af1158015618433573d6000803e3d6000fd5b505050505050508080618445906196f1565b915050617f58565b506184a6565b60405162461bcd60e51b815260206004820152602260248201527f4e6f2073746f726167652075736520646574656374656420666f722074617267604482015261195d60f21b6064820152608401611024565b6001600160a01b038716600090815260018a01602090815260408083206001600160e01b03198a168452825280832090519092916184e89188918a9101619cc7565b60408051601f198184030181529181528151602092830120835290820192909252016000205460ff166185485760405162461bcd60e51b8152602060048201526008602482015267139bdd119bdd5b9960c21b6044820152606401611024565b6005890180546001600160a01b031916905560038901805463ffffffff1916905561857760028a016000618f8c565b600060048a018190556001600160a01b038816815260208a815260408083206001600160e01b03198a168452825280832090519092916185bb9188918a9101619cc7565b60405160208183030381529060405280519060200120815260200190815260200160002054975050505050505050919050565b60008060006020855111618603578451618606565b60205b905060005b818110156186615761861e816008619862565b8661862983886196d9565b81518110618639576186396196ad565b01602001516001600160f81b031916901c929092179180618659816196f1565b91505061860b565b5090949350505050565b61868c82600060046008600c868460200201518760015b6020020151618957565b6186a882600160056009600d8660026020020151876003618682565b6186c48260026006600a600e8660046020020151876005618682565b6186e08260036007600b600f8660066020020151876007618682565b6186fc8260006005600a600f8660086020020151876009618682565b6187188260016006600b600c86600a602002015187600b618682565b61873482600260076008600d86600c602002015187600d618682565b61094882600360046009600e8681602002015187600f618682565b6000618759618cd5565b9050618763618f12565b60005b60108160ff1610156187dc5783838260ff1660108110618788576187886196ad565b602002015160ff16601081106187a0576187a06196ad565b6020020151828260ff16601081106187ba576187ba6196ad565b63ffffffff9092166020929092020152806187d481619ca8565b915050618766565b5060005b60108160ff16101561192857818160ff1660108110618801576188016196ad565b6020020151848260ff166010811061881b5761881b6196ad565b63ffffffff90921660209290920201528061883581619ca8565b9150506187e0565b60005b60048160ff16101561192857618857816008619c16565b618862906002619f9f565b6188729063ffffffff8616619897565b60f81b8361888360ff841685619975565b63ffffffff1681518110618899576188996196ad565b60200101906001600160f81b031916908160001a905350806188ba81619ca8565b915050618840565b60008060005b60088110156177ae576188dc816008619862565b856188e783876196d9565b815181106188f7576188f76196ad565b01602001516001600160f81b031916901c919091179080618917816196f1565b9150506188c8565b60006001600160401b038316821c61893883604061984b565b6001600160401b0316846001600160401b0316901b610a5e919061992e565b81878663ffffffff1660108110618970576189706196ad565b6020020151888863ffffffff166010811061898d5761898d6196ad565b60200201510101878763ffffffff16601081106189ac576189ac6196ad565b602002019063ffffffff16908163ffffffff1681525050618a08878763ffffffff16601081106189de576189de6196ad565b6020020151888563ffffffff16601081106189fb576189fb6196ad565b6020020151186010618d63565b878463ffffffff1660108110618a2057618a206196ad565b602002019063ffffffff16908163ffffffff1681525050868363ffffffff1660108110618a4f57618a4f6196ad565b6020020151878563ffffffff1660108110618a6c57618a6c6196ad565b602002015101878563ffffffff1660108110618a8a57618a8a6196ad565b602002019063ffffffff16908163ffffffff1681525050618ae6878563ffffffff1660108110618abc57618abc6196ad565b6020020151888763ffffffff1660108110618ad957618ad96196ad565b602002015118600c618d63565b878663ffffffff1660108110618afe57618afe6196ad565b602002019063ffffffff16908163ffffffff168152505080878663ffffffff1660108110618b2e57618b2e6196ad565b6020020151888863ffffffff1660108110618b4b57618b4b6196ad565b60200201510101878763ffffffff1660108110618b6a57618b6a6196ad565b602002019063ffffffff16908163ffffffff1681525050618bc6878763ffffffff1660108110618b9c57618b9c6196ad565b6020020151888563ffffffff1660108110618bb957618bb96196ad565b6020020151186008618d63565b878463ffffffff1660108110618bde57618bde6196ad565b602002019063ffffffff16908163ffffffff1681525050868363ffffffff1660108110618c0d57618c0d6196ad565b6020020151878563ffffffff1660108110618c2a57618c2a6196ad565b602002015101878563ffffffff1660108110618c4857618c486196ad565b602002019063ffffffff16908163ffffffff1681525050618ca4878563ffffffff1660108110618c7a57618c7a6196ad565b6020020151888763ffffffff1660108110618c9757618c976196ad565b6020020151186007618d63565b878663ffffffff1660108110618cbc57618cbc6196ad565b63ffffffff909216602092909202015250505050505050565b618cdd618f12565b5060408051610200810182526002815260066020820152600391810191909152600a606082015260076080820152600060a0820152600460c0820152600d60e08201526001610100820152600b610120820152600c61014082015260056101608201526009610180820152600e6101a0820152600f6101c082015260086101e082015290565b600060e083901b618d75836020619d77565b6001600160e01b031990911660ff84811682901c92161b1760e01c905092915050565b828054618da490619673565b90600052602060002090601f016020900481019282618dc65760008555618e0c565b82601f10618ddf5782800160ff19823516178555618e0c565b82800160010185558215618e0c579182015b82811115618e0c578235825591602001919060010190618df1565b50618e18929150618fa6565b5090565b6040518060a00160405280618e2f618f31565b8152602001618e3c618e8a565b8152602001618e49618ea9565b815260006020820181905260409091015290565b60405180604001604052806002905b618e74618fbb565b815260200190600190039081618e6c5790505090565b6040518061010001604052806008906020820280368337509192915050565b604051806106c001604052806036905b618ec1618e8a565b815260200190600190039081618eb95790505090565b6040518060a00160405280618eea618e8a565b8152602001618ef7618f12565b81526000602082018190526040820181905260609091015290565b6040518061020001604052806010906020820280368337509192915050565b6040518060c00160405280618f44618e8a565b815260006020820181905260606040830181905282018190526080820181905260a09091015290565b60405180610a0001604052806050906020820280368337509192915050565b50805460008255906000526020600020908101906114c891905b5b80821115618e185760008155600101618fa7565b60405180606001604052806003905b618fd2618e8a565b815260200190600190039081618fca5790505090565b60005b83811015619003578181015183820152602001618feb565b838111156119285750506000910152565b6000815180845261902c816020860160208601618fe8565b601f01601f19169290920160200192915050565b602081526000610a5e6020830184619014565b60008083601f84011261906557600080fd5b5081356001600160401b0381111561907c57600080fd5b6020830191508360208260051b85010111156114e357600080fd5b6000806000806000606086880312156190af57600080fd5b8535945060208601356001600160401b03808211156190cd57600080fd5b6190d989838a01619053565b909650945060408801359150808211156190f257600080fd5b506190ff88828901619053565b969995985093965092949392505050565b80356001600160a01b0381168114611c7057600080fd5b6000806040838503121561913a57600080fd5b61914383619110565b946020939093013593505050565b60008083601f84011261916357600080fd5b5081356001600160401b0381111561917a57600080fd5b6020830191508360208285010111156114e357600080fd5b600080602083850312156191a557600080fd5b82356001600160401b038111156191bb57600080fd5b6191c785828601619151565b90969095509350505050565b6000806000604084860312156191e857600080fd5b83356001600160401b038111156191fe57600080fd5b61920a86828701619151565b909790965060209590950135949350505050565b60006020828403121561923057600080fd5b610a5e82619110565b60008060006060848603121561924e57600080fd5b61925784619110565b925061926560208501619110565b9150604084013590509250925092565b6000806040838503121561928857600080fd5b61929183619110565b915061929f60208401619110565b90509250929050565b634e487b7160e01b600052604160045260246000fd5b604051601f8201601f191681016001600160401b03811182821017156192e6576192e66192a8565b604052919050565b60006001600160401b03821115619307576193076192a8565b50601f01601f191660200190565b6000619328619323846192ee565b6192be565b905082815283838301111561933c57600080fd5b828260208301376000602084830101529392505050565b600082601f83011261936457600080fd5b610a5e83833560208501619315565b6000806040838503121561938657600080fd5b82356001600160401b038082111561939d57600080fd5b6193a986838701619353565b935060208501359150808211156193bf57600080fd5b508301601f810185136193d157600080fd5b6193e085823560208401619315565b9150509250929050565b6000602082840312156193fc57600080fd5b5035919050565b6000806000806000806080878903121561941c57600080fd5b86356001600160401b038082111561943357600080fd5b61943f8a838b01619151565b90985096506020890135955060408901359450606089013591508082111561946657600080fd5b5061947389828a01619053565b979a9699509497509295939492505050565b6000806000806000806060878903121561949e57600080fd5b86356001600160401b03808211156194b557600080fd5b6194c18a838b01619151565b909850965060208901359150808211156194da57600080fd5b6194e68a838b01619053565b9096509450604089013591508082111561946657600080fd5b6000806020838503121561951257600080fd5b82356001600160401b0381111561952857600080fd5b6191c785828601619053565b80151581146114c857600080fd5b6000806000806080858703121561955857600080fd5b61956185619110565b935061956f60208601619110565b925060408501359150606085013561958681619534565b939692955090935050565b6000602082840312156195a357600080fd5b81356001600160401b038111156195b957600080fd5b61097384828501619353565b600080604083850312156195d857600080fd5b50508035926020909101359150565b600080600080600080600060a0888a03121561960257600080fd5b87356001600160401b038082111561961957600080fd5b6196258b838c01619151565b909950975060208a0135965060408a0135955060608a0135945060808a013591508082111561965357600080fd5b506196608a828b01619053565b989b979a50959850939692959293505050565b600181811c9082168061968757607f821691505b6020821081036196a757634e487b7160e01b600052602260045260246000fd5b50919050565b634e487b7160e01b600052603260045260246000fd5b634e487b7160e01b600052601160045260246000fd5b600082198211156196ec576196ec6196c3565b500190565b600060018201619703576197036196c3565b5060010190565b81835260006001600160fb1b0383111561972357600080fd5b8260051b8083602087013760009401602001938452509192915050565b60408152600061975460408301868861970a565b828103602084015261976781858761970a565b979650505050505050565b805160208083015191908110156196a75760001960209190910360031b1b16919050565b6001600160a01b03929092168252602082015260400190565b6000602082840312156197c157600080fd5b81516001600160401b038111156197d757600080fd5b8201601f810184136197e857600080fd5b80516197f6619323826192ee565b81815285602083850101111561980b57600080fd5b612bb7826020830160208601618fe8565b6000835161982e818460208801618fe8565b835190830190619842818360208801618fe8565b01949350505050565b60008282101561985d5761985d6196c3565b500390565b600081600019048311821515161561987c5761987c6196c3565b500290565b634e487b7160e01b600052601260045260246000fd5b6000826198a6576198a6619881565b500490565b600082516198bd818460208701618fe8565b9190910192915050565b6000602082840312156198d957600080fd5b5051919050565b6001600160e01b0319831681528151600090619903816004850160208701618fe8565b919091016004019392505050565b60006020828403121561992357600080fd5b8151610a5e81619534565b60006001600160401b03808316818516808303821115619842576198426196c3565b600063ffffffff8381169083168181101561996d5761996d6196c3565b039392505050565b600063ffffffff808316818516808303821115619842576198426196c3565b6040815260006199be60408301600a8152690808115e1c1958dd195960b21b602082015260400190565b905082602083015292915050565b6040815260006199be60408301600a815269080808081058dd1d585b60b21b602082015260400190565b60008085851115619a0657600080fd5b83861115619a1357600080fd5b5050820193919092039150565b6001600160f81b03198135818116916001851015619a485780818660010360031b1b83161692505b505092915050565b805160208201516001600160f01b03198082169291906002831015619a7f5780818460020360031b1b83161693505b505050919050565b805160208201516001600160e01b03198082169291906004831015619a7f5760049290920360031b82901b161692915050565b805160208201516001600160c01b03198082169291906008831015619a7f5760089290920360031b82901b161692915050565b805160208201516001600160801b03198082169291906010831015619a7f5760109290920360031b82901b161692915050565b634e487b7160e01b600052600160045260246000fd5b805160208201516001600160f81b03198082169291906001831015619a7f5760019290920360031b82901b161692915050565b600082619b7857619b78619881565b500690565b6bffffffffffffffffffffffff198135818116916014851015619a485760149490940360031b84901b1690921692915050565b604081526000619bda60408301600a8152690808115e1c1958dd195960b21b602082015260400190565b82810360208401526109738185619014565b604081526000619bda60408301600a815269080808081058dd1d585b60b21b602082015260400190565b600060ff821660ff84168160ff0481118215151615619c3757619c376196c3565b029392505050565b600060ff821660ff84168060ff03821115619c5c57619c5c6196c3565b019392505050565b600063ffffffff808316818103619c7d57619c7d6196c3565b6001019392505050565b6001600160a01b039390931683526020830191909152604082015260600190565b600060ff821660ff8103619cbe57619cbe6196c3565b60010192915050565b825160009082906020808701845b83811015619cf157815185529382019390820190600101619cd5565b5050948252509092019392505050565b60208082526050908201527f5061636b656420736c6f742e205468697320776f756c6420636175736520646160408201527f6e6765726f7573206f76657277726974696e6720616e642063757272656e746c60608201526f1e481a5cdb9d081cdd5c1c1bdc9d195960821b608082015260a00190565b600060ff821660ff841680821015619d9157619d916196c3565b90039392505050565b600063ffffffff80831681851681830481118215151615619dbd57619dbd6196c3565b02949350505050565b600082601f830112619dd757600080fd5b815160206001600160401b03821115619df257619df26192a8565b8160051b619e018282016192be565b9283528481018201928281019087851115619e1b57600080fd5b83870192505b8483101561976757825182529183019190830190619e21565b60008060408385031215619e4d57600080fd5b82516001600160401b0380821115619e6457600080fd5b619e7086838701619dc6565b93506020850151915080821115619e8657600080fd5b506193e085828601619dc6565b6001600160a01b039490941684526001600160e01b03199290921660208401526040830152606082015260800190565b600181815b80851115619efe578160001904821115619ee457619ee46196c3565b80851615619ef157918102915b93841c9390800290619ec8565b509250929050565b600082619f1557506001610960565b81619f2257506000610960565b8160018114619f385760028114619f4257619f5e565b6001915050610960565b60ff841115619f5357619f536196c3565b50506001821b610960565b5060208310610133831016604e8410600b8410161715619f81575081810a610960565b619f8b8383619ec3565b8060001904821115619c3757619c376196c3565b6000610a5e60ff841683619f0656fe41304facd9323d75b11bcdd609cb38effffdb05710f7caf0e9b16c6d9d709f500000000000000000000000007109709ecfa91a80626ff3989d68f67f5b1dd12d885cb69240a935d632d79c317109709ecfa91a80626ff3989d68f67f5b1dd12da2646970667358221220defcd26bddb7026bbdd55660672d398f494fe09541edcfa105f6861d0ba5fb2c64736f6c634300080d0033" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct ThemelioBridgeTest<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for ThemelioBridgeTest<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ThemelioBridgeTest<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ThemelioBridgeTest))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> ThemelioBridgeTest<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), THEMELIOBRIDGETEST_ABI.clone(), client)
                .into()
        }
        #[doc = r" Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it."]
        #[doc = r" Returns a new instance of a deployer that returns an instance of this contract after sending the transaction"]
        #[doc = r""]
        #[doc = r" Notes:"]
        #[doc = r" 1. If there are no constructor arguments, you should pass `()` as the argument."]
        #[doc = r" 1. The default poll duration is 7 seconds."]
        #[doc = r" 1. The default number of confirmations is 1 block."]
        #[doc = r""]
        #[doc = r""]
        #[doc = r" # Example"]
        #[doc = r""]
        #[doc = r" Generate contract bindings with `abigen!` and deploy a new contract instance."]
        #[doc = r""]
        #[doc = r" *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact."]
        #[doc = r""]
        #[doc = r" ```ignore"]
        #[doc = r" # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {"]
        #[doc = r#"     abigen!(Greeter,"../greeter.json");"#]
        #[doc = r""]
        #[doc = r#"    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();"#]
        #[doc = r"    let msg = greeter_contract.greet().call().await.unwrap();"]
        #[doc = r" # }"]
        #[doc = r" ```"]
        pub fn deploy<T: ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> Result<
            ethers::contract::builders::ContractDeployer<M, Self>,
            ethers::contract::ContractError<M>,
        > {
            let factory = ethers::contract::ContractFactory::new(
                THEMELIOBRIDGETEST_ABI.clone(),
                THEMELIOBRIDGETEST_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `IS_TEST` (0xfa7626d4) function"]
        pub fn is_test(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([250, 118, 38, 212], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `allowance` (0xdd62ed3e) function"]
        pub fn allowance(
            &self,
            owner: ethers::core::types::Address,
            spender: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([221, 98, 237, 62], (owner, spender))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `approve` (0x095ea7b3) function"]
        pub fn approve(
            &self,
            spender: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([9, 94, 167, 179], (spender, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `balanceOf` (0x70a08231) function"]
        pub fn balance_of(
            &self,
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `burn` (0xbcf64e05) function"]
        pub fn burn(
            &self,
            value: ethers::core::types::U256,
            themelio_recipient: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([188, 246, 78, 5], (value, themelio_recipient))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `computeMerkleRootTestHelper` (0xd8626f36) function"]
        pub fn compute_merkle_root_test_helper(
            &self,
            header: ethers::core::types::Bytes,
            block_height: ethers::core::types::U256,
            tx_hash: [u8; 32],
            index: ethers::core::types::U256,
            proof: ::std::vec::Vec<[u8; 32]>,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash(
                    [216, 98, 111, 54],
                    (header, block_height, tx_hash, index, proof),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `deal` (0x6bce989b) function"]
        pub fn deal_with_token(
            &self,
            token: ethers::core::types::Address,
            to: ethers::core::types::Address,
            give: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([107, 206, 152, 155], (token, to, give))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `deal` (0x97754ae9) function"]
        pub fn deal_with_token_and_adjust(
            &self,
            token: ethers::core::types::Address,
            to: ethers::core::types::Address,
            give: ethers::core::types::U256,
            adjust: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([151, 117, 74, 233], (token, to, give, adjust))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `deal` (0xc88a5e6d) function"]
        pub fn deal(
            &self,
            to: ethers::core::types::Address,
            give: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([200, 138, 94, 109], (to, give))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decimals` (0x313ce567) function"]
        pub fn decimals(&self) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decodeIntegerTestHelper` (0x14229b63) function"]
        pub fn decode_integer_test_helper(
            &self,
            header: ethers::core::types::Bytes,
            offset: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([20, 34, 155, 99], (header, offset))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decreaseAllowance` (0xa457c2d7) function"]
        pub fn decrease_allowance(
            &self,
            spender: ethers::core::types::Address,
            subtracted_value: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([164, 87, 194, 215], (spender, subtracted_value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `deployCode` (0x29ce9dde) function"]
        pub fn deploy_code_with_args(
            &self,
            what: String,
            args: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([41, 206, 157, 222], (what, args))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `deployCode` (0x9a8325a0) function"]
        pub fn deploy_code(
            &self,
            what: String,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([154, 131, 37, 160], what)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `epochs` (0xc6b61e4c) function"]
        pub fn epochs(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([198, 182, 30, 76], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `extractBlockHeightTestHelper` (0x0f9d0d47) function"]
        pub fn extract_block_height_test_helper(
            &self,
            header: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([15, 157, 13, 71], header)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `extractValueAndRecipientTestHelper` (0x679c917c) function"]
        pub fn extract_value_and_recipient_test_helper(
            &self,
            transaction: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, ethers::core::types::Address),
        > {
            self.0
                .method_hash([103, 156, 145, 124], transaction)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `failed` (0xba414fa6) function"]
        pub fn failed(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([186, 65, 79, 166], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `headers` (0x56f90d79) function"]
        pub fn headers(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Bytes> {
            self.0
                .method_hash([86, 249, 13, 121], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `hoax` (0x233240ee) function"]
        pub fn hoax_0(
            &self,
            who: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([35, 50, 64, 238], who)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `hoax` (0x29a9e300) function"]
        pub fn hoax_2(
            &self,
            who: ethers::core::types::Address,
            origin: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([41, 169, 227, 0], (who, origin))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `hoax` (0xaf9bbe5f) function"]
        pub fn hoax_3(
            &self,
            who: ethers::core::types::Address,
            origin: ethers::core::types::Address,
            give: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([175, 155, 190, 95], (who, origin, give))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `hoax` (0xe9a79a7b) function"]
        pub fn hoax_4(
            &self,
            who: ethers::core::types::Address,
            give: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([233, 167, 154, 123], (who, give))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `increaseAllowance` (0x39509351) function"]
        pub fn increase_allowance(
            &self,
            spender: ethers::core::types::Address,
            added_value: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([57, 80, 147, 81], (spender, added_value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `name` (0x06fdde03) function"]
        pub fn name(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `relayHeader` (0x442a807a) function"]
        pub fn relay_header(
            &self,
            header: ethers::core::types::Bytes,
            signers: ::std::vec::Vec<[u8; 32]>,
            signatures: ::std::vec::Vec<[u8; 32]>,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([68, 42, 128, 122], (header, signers, signatures))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `relayHeaderTestHelper` (0x852418ba) function"]
        pub fn relay_header_test_helper(
            &self,
            signers: ::std::vec::Vec<[u8; 32]>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([133, 36, 24, 186], signers)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `relayStakers` (0x073a47f3) function"]
        pub fn relay_stakers(
            &self,
            epoch: ethers::core::types::U256,
            stakers: ::std::vec::Vec<[u8; 32]>,
            staker_syms: ::std::vec::Vec<ethers::core::types::U256>,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([7, 58, 71, 243], (epoch, stakers, staker_syms))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `relayStakersHelper` (0xf7842f2d) function"]
        pub fn relay_stakers_helper(
            &self,
            epoch: ethers::core::types::U256,
            staker: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, ethers::core::types::U256),
        > {
            self.0
                .method_hash([247, 132, 47, 45], (epoch, staker))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `rewind` (0x2d6c17a3) function"]
        pub fn rewind(
            &self,
            time: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([45, 108, 23, 163], time)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `skip` (0xb9c071b4) function"]
        pub fn skip(
            &self,
            time: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([185, 192, 113, 180], time)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `startHoax` (0x108554f2) function"]
        pub fn start_hoax_1(
            &self,
            who: ethers::core::types::Address,
            give: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([16, 133, 84, 242], (who, give))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `startHoax` (0x3bf82db1) function"]
        pub fn start_hoax_2(
            &self,
            who: ethers::core::types::Address,
            origin: ethers::core::types::Address,
            give: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([59, 248, 45, 177], (who, origin, give))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `startHoax` (0x6f597075) function"]
        pub fn start_hoax_0(
            &self,
            who: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([111, 89, 112, 117], who)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `startHoax` (0xd06d8229) function"]
        pub fn start_hoax_4(
            &self,
            who: ethers::core::types::Address,
            origin: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([208, 109, 130, 41], (who, origin))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `symbol` (0x95d89b41) function"]
        pub fn symbol(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `testBlake3Hasher` (0x07e075b4) function"]
        pub fn test_blake_3_hasher(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([7, 224, 117, 180], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `testBlake3KeyedHasher` (0x51ee9269) function"]
        pub fn test_blake_3_keyed_hasher(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([81, 238, 146, 105], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `testDecimals` (0x5d4696cf) function"]
        pub fn test_decimals(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([93, 70, 150, 207], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `testEd25519` (0xb2d7ee31) function"]
        pub fn test_ed_25519(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([178, 215, 238, 49], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `testEncodedIntegerSize` (0x2993c221) function"]
        pub fn test_encoded_integer_size(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([41, 147, 194, 33], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `testExtractMerkleRoot` (0xd6004d51) function"]
        pub fn test_extract_merkle_root(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([214, 0, 77, 81], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `testExtractTokenType` (0xa797dccb) function"]
        pub fn test_extract_token_type(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([167, 151, 220, 203], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `testHashDatablock` (0xf8551b18) function"]
        pub fn test_hash_datablock(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([248, 85, 27, 24], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `testHashNode` (0xf34d5c4a) function"]
        pub fn test_hash_node(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([243, 77, 92, 74], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `testSlice` (0xe27c5b17) function"]
        pub fn test_slice(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([226, 124, 91, 23], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `tip` (0xd82555f1) function"]
        pub fn tip(
            &self,
            token: ethers::core::types::Address,
            to: ethers::core::types::Address,
            give: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([216, 37, 85, 241], (token, to, give))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `totalSupply` (0x18160ddd) function"]
        pub fn total_supply(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transfer` (0xa9059cbb) function"]
        pub fn transfer(
            &self,
            to: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (to, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferFrom` (0x23b872dd) function"]
        pub fn transfer_from(
            &self,
            from: ethers::core::types::Address,
            to: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (from, to, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `verifyTx` (0x42217b63) function"]
        pub fn verify_tx(
            &self,
            transaction: ethers::core::types::Bytes,
            tx_index: ethers::core::types::U256,
            block_height: ethers::core::types::U256,
            proof: ::std::vec::Vec<[u8; 32]>,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [66, 33, 123, 99],
                    (transaction, tx_index, block_height, proof),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `verifyTxTestHelper` (0x9f84cd1d) function"]
        pub fn verify_tx_test_helper(
            &self,
            header: ethers::core::types::Bytes,
            block_height: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([159, 132, 205, 29], (header, block_height))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `vm` (0x3a768463) function"]
        pub fn vm(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([58, 118, 132, 99], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Approval` event"]
        pub fn approval_filter(&self) -> ethers::contract::builders::Event<M, ApprovalFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `HeaderRelayed` event"]
        pub fn header_relayed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, HeaderRelayedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `StakersRelayed` event"]
        pub fn stakers_relayed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, StakersRelayedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TokensBurned` event"]
        pub fn tokens_burned_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TokensBurnedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TokensMinted` event"]
        pub fn tokens_minted_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TokensMintedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Transfer` event"]
        pub fn transfer_filter(&self) -> ethers::contract::builders::Event<M, TransferFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TxVerified` event"]
        pub fn tx_verified_filter(&self) -> ethers::contract::builders::Event<M, TxVerifiedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `WARNING_Deprecated` event"]
        pub fn warning_deprecated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, WarningDeprecatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log` event"]
        pub fn log_filter(&self) -> ethers::contract::builders::Event<M, LogFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_address` event"]
        pub fn log_address_filter(&self) -> ethers::contract::builders::Event<M, LogAddressFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_bytes` event"]
        pub fn log_bytes_filter(&self) -> ethers::contract::builders::Event<M, LogBytesFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_bytes32` event"]
        pub fn log_bytes_32_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogBytes32Filter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_int` event"]
        pub fn log_int_filter(&self) -> ethers::contract::builders::Event<M, LogIntFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_named_address` event"]
        pub fn log_named_address_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogNamedAddressFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_named_bytes` event"]
        pub fn log_named_bytes_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogNamedBytesFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_named_bytes32` event"]
        pub fn log_named_bytes_32_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogNamedBytes32Filter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_named_decimal_int` event"]
        pub fn log_named_decimal_int_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogNamedDecimalIntFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_named_decimal_uint` event"]
        pub fn log_named_decimal_uint_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogNamedDecimalUintFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_named_int` event"]
        pub fn log_named_int_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogNamedIntFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_named_string` event"]
        pub fn log_named_string_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogNamedStringFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_named_uint` event"]
        pub fn log_named_uint_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogNamedUintFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_string` event"]
        pub fn log_string_filter(&self) -> ethers::contract::builders::Event<M, LogStringFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_uint` event"]
        pub fn log_uint_filter(&self) -> ethers::contract::builders::Event<M, LogUintFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `logs` event"]
        pub fn logs_filter(&self) -> ethers::contract::builders::Event<M, LogsFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, ThemelioBridgeTestEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for ThemelioBridgeTest<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub spender: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "HeaderRelayed", abi = "HeaderRelayed(uint256)")]
    pub struct HeaderRelayedFilter {
        #[ethevent(indexed)]
        pub height: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(
        name = "StakersRelayed",
        abi = "StakersRelayed(uint256,bytes32[],uint256[])"
    )]
    pub struct StakersRelayedFilter {
        #[ethevent(indexed)]
        pub epoch: ethers::core::types::U256,
        pub stakers: Vec<[u8; 32]>,
        pub syms_staked: Vec<ethers::core::types::U256>,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "TokensBurned", abi = "TokensBurned(address,uint256,bytes32)")]
    pub struct TokensBurnedFilter {
        #[ethevent(indexed)]
        pub recipient: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub value: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub themelio_recipient: [u8; 32],
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "TokensMinted", abi = "TokensMinted(address,uint256)")]
    pub struct TokensMintedFilter {
        #[ethevent(indexed)]
        pub recipient: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub value: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "TxVerified", abi = "TxVerified(bytes32,uint256)")]
    pub struct TxVerifiedFilter {
        #[ethevent(indexed)]
        pub tx_hash: [u8; 32],
        #[ethevent(indexed)]
        pub height: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "WARNING_Deprecated", abi = "WARNING_Deprecated(string)")]
    pub struct WarningDeprecatedFilter {
        pub msg: String,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "log", abi = "log(string)")]
    pub struct LogFilter(pub String);
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "log_address", abi = "log_address(address)")]
    pub struct LogAddressFilter(pub ethers::core::types::Address);
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "log_bytes", abi = "log_bytes(bytes)")]
    pub struct LogBytesFilter(pub ethers::core::types::Bytes);
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "log_bytes32", abi = "log_bytes32(bytes32)")]
    pub struct LogBytes32Filter(pub [u8; 32]);
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "log_int", abi = "log_int(int256)")]
    pub struct LogIntFilter(pub I256);
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "log_named_address", abi = "log_named_address(string,address)")]
    pub struct LogNamedAddressFilter {
        pub key: String,
        pub val: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "log_named_bytes", abi = "log_named_bytes(string,bytes)")]
    pub struct LogNamedBytesFilter {
        pub key: String,
        pub val: ethers::core::types::Bytes,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "log_named_bytes32", abi = "log_named_bytes32(string,bytes32)")]
    pub struct LogNamedBytes32Filter {
        pub key: String,
        pub val: [u8; 32],
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(
        name = "log_named_decimal_int",
        abi = "log_named_decimal_int(string,int256,uint256)"
    )]
    pub struct LogNamedDecimalIntFilter {
        pub key: String,
        pub val: I256,
        pub decimals: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(
        name = "log_named_decimal_uint",
        abi = "log_named_decimal_uint(string,uint256,uint256)"
    )]
    pub struct LogNamedDecimalUintFilter {
        pub key: String,
        pub val: ethers::core::types::U256,
        pub decimals: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "log_named_int", abi = "log_named_int(string,int256)")]
    pub struct LogNamedIntFilter {
        pub key: String,
        pub val: I256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "log_named_string", abi = "log_named_string(string,string)")]
    pub struct LogNamedStringFilter {
        pub key: String,
        pub val: String,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "log_named_uint", abi = "log_named_uint(string,uint256)")]
    pub struct LogNamedUintFilter {
        pub key: String,
        pub val: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "log_string", abi = "log_string(string)")]
    pub struct LogStringFilter(pub String);
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "log_uint", abi = "log_uint(uint256)")]
    pub struct LogUintFilter(pub ethers::core::types::U256);
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "logs", abi = "logs(bytes)")]
    pub struct LogsFilter(pub ethers::core::types::Bytes);
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ThemelioBridgeTestEvents {
        ApprovalFilter(ApprovalFilter),
        HeaderRelayedFilter(HeaderRelayedFilter),
        StakersRelayedFilter(StakersRelayedFilter),
        TokensBurnedFilter(TokensBurnedFilter),
        TokensMintedFilter(TokensMintedFilter),
        TransferFilter(TransferFilter),
        TxVerifiedFilter(TxVerifiedFilter),
        WarningDeprecatedFilter(WarningDeprecatedFilter),
        LogFilter(LogFilter),
        LogAddressFilter(LogAddressFilter),
        LogBytesFilter(LogBytesFilter),
        LogBytes32Filter(LogBytes32Filter),
        LogIntFilter(LogIntFilter),
        LogNamedAddressFilter(LogNamedAddressFilter),
        LogNamedBytesFilter(LogNamedBytesFilter),
        LogNamedBytes32Filter(LogNamedBytes32Filter),
        LogNamedDecimalIntFilter(LogNamedDecimalIntFilter),
        LogNamedDecimalUintFilter(LogNamedDecimalUintFilter),
        LogNamedIntFilter(LogNamedIntFilter),
        LogNamedStringFilter(LogNamedStringFilter),
        LogNamedUintFilter(LogNamedUintFilter),
        LogStringFilter(LogStringFilter),
        LogUintFilter(LogUintFilter),
        LogsFilter(LogsFilter),
    }
    impl ethers::contract::EthLogDecode for ThemelioBridgeTestEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(ThemelioBridgeTestEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = HeaderRelayedFilter::decode_log(log) {
                return Ok(ThemelioBridgeTestEvents::HeaderRelayedFilter(decoded));
            }
            if let Ok(decoded) = StakersRelayedFilter::decode_log(log) {
                return Ok(ThemelioBridgeTestEvents::StakersRelayedFilter(decoded));
            }
            if let Ok(decoded) = TokensBurnedFilter::decode_log(log) {
                return Ok(ThemelioBridgeTestEvents::TokensBurnedFilter(decoded));
            }
            if let Ok(decoded) = TokensMintedFilter::decode_log(log) {
                return Ok(ThemelioBridgeTestEvents::TokensMintedFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(ThemelioBridgeTestEvents::TransferFilter(decoded));
            }
            if let Ok(decoded) = TxVerifiedFilter::decode_log(log) {
                return Ok(ThemelioBridgeTestEvents::TxVerifiedFilter(decoded));
            }
            if let Ok(decoded) = WarningDeprecatedFilter::decode_log(log) {
                return Ok(ThemelioBridgeTestEvents::WarningDeprecatedFilter(decoded));
            }
            if let Ok(decoded) = LogFilter::decode_log(log) {
                return Ok(ThemelioBridgeTestEvents::LogFilter(decoded));
            }
            if let Ok(decoded) = LogAddressFilter::decode_log(log) {
                return Ok(ThemelioBridgeTestEvents::LogAddressFilter(decoded));
            }
            if let Ok(decoded) = LogBytesFilter::decode_log(log) {
                return Ok(ThemelioBridgeTestEvents::LogBytesFilter(decoded));
            }
            if let Ok(decoded) = LogBytes32Filter::decode_log(log) {
                return Ok(ThemelioBridgeTestEvents::LogBytes32Filter(decoded));
            }
            if let Ok(decoded) = LogIntFilter::decode_log(log) {
                return Ok(ThemelioBridgeTestEvents::LogIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedAddressFilter::decode_log(log) {
                return Ok(ThemelioBridgeTestEvents::LogNamedAddressFilter(decoded));
            }
            if let Ok(decoded) = LogNamedBytesFilter::decode_log(log) {
                return Ok(ThemelioBridgeTestEvents::LogNamedBytesFilter(decoded));
            }
            if let Ok(decoded) = LogNamedBytes32Filter::decode_log(log) {
                return Ok(ThemelioBridgeTestEvents::LogNamedBytes32Filter(decoded));
            }
            if let Ok(decoded) = LogNamedDecimalIntFilter::decode_log(log) {
                return Ok(ThemelioBridgeTestEvents::LogNamedDecimalIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedDecimalUintFilter::decode_log(log) {
                return Ok(ThemelioBridgeTestEvents::LogNamedDecimalUintFilter(decoded));
            }
            if let Ok(decoded) = LogNamedIntFilter::decode_log(log) {
                return Ok(ThemelioBridgeTestEvents::LogNamedIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedStringFilter::decode_log(log) {
                return Ok(ThemelioBridgeTestEvents::LogNamedStringFilter(decoded));
            }
            if let Ok(decoded) = LogNamedUintFilter::decode_log(log) {
                return Ok(ThemelioBridgeTestEvents::LogNamedUintFilter(decoded));
            }
            if let Ok(decoded) = LogStringFilter::decode_log(log) {
                return Ok(ThemelioBridgeTestEvents::LogStringFilter(decoded));
            }
            if let Ok(decoded) = LogUintFilter::decode_log(log) {
                return Ok(ThemelioBridgeTestEvents::LogUintFilter(decoded));
            }
            if let Ok(decoded) = LogsFilter::decode_log(log) {
                return Ok(ThemelioBridgeTestEvents::LogsFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for ThemelioBridgeTestEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ThemelioBridgeTestEvents::ApprovalFilter(element) => element.fmt(f),
                ThemelioBridgeTestEvents::HeaderRelayedFilter(element) => element.fmt(f),
                ThemelioBridgeTestEvents::StakersRelayedFilter(element) => element.fmt(f),
                ThemelioBridgeTestEvents::TokensBurnedFilter(element) => element.fmt(f),
                ThemelioBridgeTestEvents::TokensMintedFilter(element) => element.fmt(f),
                ThemelioBridgeTestEvents::TransferFilter(element) => element.fmt(f),
                ThemelioBridgeTestEvents::TxVerifiedFilter(element) => element.fmt(f),
                ThemelioBridgeTestEvents::WarningDeprecatedFilter(element) => element.fmt(f),
                ThemelioBridgeTestEvents::LogFilter(element) => element.fmt(f),
                ThemelioBridgeTestEvents::LogAddressFilter(element) => element.fmt(f),
                ThemelioBridgeTestEvents::LogBytesFilter(element) => element.fmt(f),
                ThemelioBridgeTestEvents::LogBytes32Filter(element) => element.fmt(f),
                ThemelioBridgeTestEvents::LogIntFilter(element) => element.fmt(f),
                ThemelioBridgeTestEvents::LogNamedAddressFilter(element) => element.fmt(f),
                ThemelioBridgeTestEvents::LogNamedBytesFilter(element) => element.fmt(f),
                ThemelioBridgeTestEvents::LogNamedBytes32Filter(element) => element.fmt(f),
                ThemelioBridgeTestEvents::LogNamedDecimalIntFilter(element) => element.fmt(f),
                ThemelioBridgeTestEvents::LogNamedDecimalUintFilter(element) => element.fmt(f),
                ThemelioBridgeTestEvents::LogNamedIntFilter(element) => element.fmt(f),
                ThemelioBridgeTestEvents::LogNamedStringFilter(element) => element.fmt(f),
                ThemelioBridgeTestEvents::LogNamedUintFilter(element) => element.fmt(f),
                ThemelioBridgeTestEvents::LogStringFilter(element) => element.fmt(f),
                ThemelioBridgeTestEvents::LogUintFilter(element) => element.fmt(f),
                ThemelioBridgeTestEvents::LogsFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `IS_TEST`function with signature `IS_TEST()` and selector `[250, 118, 38, 212]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "IS_TEST", abi = "IS_TEST()")]
    pub struct IsTestCall;
    #[doc = "Container type for all input parameters for the `allowance`function with signature `allowance(address,address)` and selector `[221, 98, 237, 62]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "allowance", abi = "allowance(address,address)")]
    pub struct AllowanceCall {
        pub owner: ethers::core::types::Address,
        pub spender: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `approve`function with signature `approve(address,uint256)` and selector `[9, 94, 167, 179]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall {
        pub spender: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `balanceOf`function with signature `balanceOf(address)` and selector `[112, 160, 130, 49]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall {
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `burn`function with signature `burn(uint256,bytes32)` and selector `[188, 246, 78, 5]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "burn", abi = "burn(uint256,bytes32)")]
    pub struct BurnCall {
        pub value: ethers::core::types::U256,
        pub themelio_recipient: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `computeMerkleRootTestHelper`function with signature `computeMerkleRootTestHelper(bytes,uint256,bytes32,uint256,bytes32[])` and selector `[216, 98, 111, 54]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "computeMerkleRootTestHelper",
        abi = "computeMerkleRootTestHelper(bytes,uint256,bytes32,uint256,bytes32[])"
    )]
    pub struct ComputeMerkleRootTestHelperCall {
        pub header: ethers::core::types::Bytes,
        pub block_height: ethers::core::types::U256,
        pub tx_hash: [u8; 32],
        pub index: ethers::core::types::U256,
        pub proof: ::std::vec::Vec<[u8; 32]>,
    }
    #[doc = "Container type for all input parameters for the `deal`function with signature `deal(address,address,uint256)` and selector `[107, 206, 152, 155]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "deal", abi = "deal(address,address,uint256)")]
    pub struct DealWithTokenCall {
        pub token: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
        pub give: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `deal`function with signature `deal(address,address,uint256,bool)` and selector `[151, 117, 74, 233]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "deal", abi = "deal(address,address,uint256,bool)")]
    pub struct DealWithTokenAndAdjustCall {
        pub token: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
        pub give: ethers::core::types::U256,
        pub adjust: bool,
    }
    #[doc = "Container type for all input parameters for the `deal`function with signature `deal(address,uint256)` and selector `[200, 138, 94, 109]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "deal", abi = "deal(address,uint256)")]
    pub struct DealCall {
        pub to: ethers::core::types::Address,
        pub give: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `decimals`function with signature `decimals()` and selector `[49, 60, 229, 103]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "decimals", abi = "decimals()")]
    pub struct DecimalsCall;
    #[doc = "Container type for all input parameters for the `decodeIntegerTestHelper`function with signature `decodeIntegerTestHelper(bytes,uint256)` and selector `[20, 34, 155, 99]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "decodeIntegerTestHelper",
        abi = "decodeIntegerTestHelper(bytes,uint256)"
    )]
    pub struct DecodeIntegerTestHelperCall {
        pub header: ethers::core::types::Bytes,
        pub offset: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `decreaseAllowance`function with signature `decreaseAllowance(address,uint256)` and selector `[164, 87, 194, 215]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "decreaseAllowance", abi = "decreaseAllowance(address,uint256)")]
    pub struct DecreaseAllowanceCall {
        pub spender: ethers::core::types::Address,
        pub subtracted_value: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `deployCode`function with signature `deployCode(string,bytes)` and selector `[41, 206, 157, 222]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "deployCode", abi = "deployCode(string,bytes)")]
    pub struct DeployCodeWithArgsCall {
        pub what: String,
        pub args: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `deployCode`function with signature `deployCode(string)` and selector `[154, 131, 37, 160]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "deployCode", abi = "deployCode(string)")]
    pub struct DeployCodeCall {
        pub what: String,
    }
    #[doc = "Container type for all input parameters for the `epochs`function with signature `epochs(uint256)` and selector `[198, 182, 30, 76]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "epochs", abi = "epochs(uint256)")]
    pub struct EpochsCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `extractBlockHeightTestHelper`function with signature `extractBlockHeightTestHelper(bytes)` and selector `[15, 157, 13, 71]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "extractBlockHeightTestHelper",
        abi = "extractBlockHeightTestHelper(bytes)"
    )]
    pub struct ExtractBlockHeightTestHelperCall {
        pub header: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `extractValueAndRecipientTestHelper`function with signature `extractValueAndRecipientTestHelper(bytes)` and selector `[103, 156, 145, 124]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "extractValueAndRecipientTestHelper",
        abi = "extractValueAndRecipientTestHelper(bytes)"
    )]
    pub struct ExtractValueAndRecipientTestHelperCall {
        pub transaction: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `failed`function with signature `failed()` and selector `[186, 65, 79, 166]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "failed", abi = "failed()")]
    pub struct FailedCall;
    #[doc = "Container type for all input parameters for the `headers`function with signature `headers(uint256)` and selector `[86, 249, 13, 121]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "headers", abi = "headers(uint256)")]
    pub struct HeadersCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `hoax`function with signature `hoax(address)` and selector `[35, 50, 64, 238]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "hoax", abi = "hoax(address)")]
    pub struct Hoax0Call {
        pub who: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `hoax`function with signature `hoax(address,address)` and selector `[41, 169, 227, 0]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "hoax", abi = "hoax(address,address)")]
    pub struct Hoax2Call {
        pub who: ethers::core::types::Address,
        pub origin: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `hoax`function with signature `hoax(address,address,uint256)` and selector `[175, 155, 190, 95]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "hoax", abi = "hoax(address,address,uint256)")]
    pub struct Hoax3Call {
        pub who: ethers::core::types::Address,
        pub origin: ethers::core::types::Address,
        pub give: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `hoax`function with signature `hoax(address,uint256)` and selector `[233, 167, 154, 123]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "hoax", abi = "hoax(address,uint256)")]
    pub struct Hoax4Call {
        pub who: ethers::core::types::Address,
        pub give: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `increaseAllowance`function with signature `increaseAllowance(address,uint256)` and selector `[57, 80, 147, 81]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "increaseAllowance", abi = "increaseAllowance(address,uint256)")]
    pub struct IncreaseAllowanceCall {
        pub spender: ethers::core::types::Address,
        pub added_value: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `name`function with signature `name()` and selector `[6, 253, 222, 3]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    #[doc = "Container type for all input parameters for the `relayHeader`function with signature `relayHeader(bytes,bytes32[],bytes32[])` and selector `[68, 42, 128, 122]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "relayHeader", abi = "relayHeader(bytes,bytes32[],bytes32[])")]
    pub struct RelayHeaderCall {
        pub header: ethers::core::types::Bytes,
        pub signers: ::std::vec::Vec<[u8; 32]>,
        pub signatures: ::std::vec::Vec<[u8; 32]>,
    }
    #[doc = "Container type for all input parameters for the `relayHeaderTestHelper`function with signature `relayHeaderTestHelper(bytes32[])` and selector `[133, 36, 24, 186]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "relayHeaderTestHelper",
        abi = "relayHeaderTestHelper(bytes32[])"
    )]
    pub struct RelayHeaderTestHelperCall {
        pub signers: ::std::vec::Vec<[u8; 32]>,
    }
    #[doc = "Container type for all input parameters for the `relayStakers`function with signature `relayStakers(uint256,bytes32[],uint256[])` and selector `[7, 58, 71, 243]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "relayStakers",
        abi = "relayStakers(uint256,bytes32[],uint256[])"
    )]
    pub struct RelayStakersCall {
        pub epoch: ethers::core::types::U256,
        pub stakers: ::std::vec::Vec<[u8; 32]>,
        pub staker_syms: ::std::vec::Vec<ethers::core::types::U256>,
    }
    #[doc = "Container type for all input parameters for the `relayStakersHelper`function with signature `relayStakersHelper(uint256,bytes32)` and selector `[247, 132, 47, 45]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "relayStakersHelper",
        abi = "relayStakersHelper(uint256,bytes32)"
    )]
    pub struct RelayStakersHelperCall {
        pub epoch: ethers::core::types::U256,
        pub staker: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `rewind`function with signature `rewind(uint256)` and selector `[45, 108, 23, 163]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "rewind", abi = "rewind(uint256)")]
    pub struct RewindCall {
        pub time: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `skip`function with signature `skip(uint256)` and selector `[185, 192, 113, 180]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "skip", abi = "skip(uint256)")]
    pub struct SkipCall {
        pub time: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `startHoax`function with signature `startHoax(address,uint256)` and selector `[16, 133, 84, 242]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "startHoax", abi = "startHoax(address,uint256)")]
    pub struct StartHoax1Call {
        pub who: ethers::core::types::Address,
        pub give: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `startHoax`function with signature `startHoax(address,address,uint256)` and selector `[59, 248, 45, 177]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "startHoax", abi = "startHoax(address,address,uint256)")]
    pub struct StartHoax2Call {
        pub who: ethers::core::types::Address,
        pub origin: ethers::core::types::Address,
        pub give: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `startHoax`function with signature `startHoax(address)` and selector `[111, 89, 112, 117]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "startHoax", abi = "startHoax(address)")]
    pub struct StartHoax0Call {
        pub who: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `startHoax`function with signature `startHoax(address,address)` and selector `[208, 109, 130, 41]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "startHoax", abi = "startHoax(address,address)")]
    pub struct StartHoax4Call {
        pub who: ethers::core::types::Address,
        pub origin: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `symbol`function with signature `symbol()` and selector `[149, 216, 155, 65]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
    #[doc = "Container type for all input parameters for the `testBlake3Hasher`function with signature `testBlake3Hasher()` and selector `[7, 224, 117, 180]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "testBlake3Hasher", abi = "testBlake3Hasher()")]
    pub struct TestBlake3HasherCall;
    #[doc = "Container type for all input parameters for the `testBlake3KeyedHasher`function with signature `testBlake3KeyedHasher()` and selector `[81, 238, 146, 105]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "testBlake3KeyedHasher", abi = "testBlake3KeyedHasher()")]
    pub struct TestBlake3KeyedHasherCall;
    #[doc = "Container type for all input parameters for the `testDecimals`function with signature `testDecimals()` and selector `[93, 70, 150, 207]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "testDecimals", abi = "testDecimals()")]
    pub struct TestDecimalsCall;
    #[doc = "Container type for all input parameters for the `testEd25519`function with signature `testEd25519()` and selector `[178, 215, 238, 49]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "testEd25519", abi = "testEd25519()")]
    pub struct TestEd25519Call;
    #[doc = "Container type for all input parameters for the `testEncodedIntegerSize`function with signature `testEncodedIntegerSize()` and selector `[41, 147, 194, 33]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "testEncodedIntegerSize", abi = "testEncodedIntegerSize()")]
    pub struct TestEncodedIntegerSizeCall;
    #[doc = "Container type for all input parameters for the `testExtractMerkleRoot`function with signature `testExtractMerkleRoot()` and selector `[214, 0, 77, 81]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "testExtractMerkleRoot", abi = "testExtractMerkleRoot()")]
    pub struct TestExtractMerkleRootCall;
    #[doc = "Container type for all input parameters for the `testExtractTokenType`function with signature `testExtractTokenType()` and selector `[167, 151, 220, 203]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "testExtractTokenType", abi = "testExtractTokenType()")]
    pub struct TestExtractTokenTypeCall;
    #[doc = "Container type for all input parameters for the `testHashDatablock`function with signature `testHashDatablock()` and selector `[248, 85, 27, 24]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "testHashDatablock", abi = "testHashDatablock()")]
    pub struct TestHashDatablockCall;
    #[doc = "Container type for all input parameters for the `testHashNode`function with signature `testHashNode()` and selector `[243, 77, 92, 74]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "testHashNode", abi = "testHashNode()")]
    pub struct TestHashNodeCall;
    #[doc = "Container type for all input parameters for the `testSlice`function with signature `testSlice()` and selector `[226, 124, 91, 23]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "testSlice", abi = "testSlice()")]
    pub struct TestSliceCall;
    #[doc = "Container type for all input parameters for the `tip`function with signature `tip(address,address,uint256)` and selector `[216, 37, 85, 241]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "tip", abi = "tip(address,address,uint256)")]
    pub struct TipCall {
        pub token: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
        pub give: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `totalSupply`function with signature `totalSupply()` and selector `[24, 22, 13, 221]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
    #[doc = "Container type for all input parameters for the `transfer`function with signature `transfer(address,uint256)` and selector `[169, 5, 156, 187]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "transfer", abi = "transfer(address,uint256)")]
    pub struct TransferCall {
        pub to: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `transferFrom`function with signature `transferFrom(address,address,uint256)` and selector `[35, 184, 114, 221]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "transferFrom", abi = "transferFrom(address,address,uint256)")]
    pub struct TransferFromCall {
        pub from: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `verifyTx`function with signature `verifyTx(bytes,uint256,uint256,bytes32[])` and selector `[66, 33, 123, 99]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "verifyTx", abi = "verifyTx(bytes,uint256,uint256,bytes32[])")]
    pub struct VerifyTxCall {
        pub transaction: ethers::core::types::Bytes,
        pub tx_index: ethers::core::types::U256,
        pub block_height: ethers::core::types::U256,
        pub proof: ::std::vec::Vec<[u8; 32]>,
    }
    #[doc = "Container type for all input parameters for the `verifyTxTestHelper`function with signature `verifyTxTestHelper(bytes,uint256)` and selector `[159, 132, 205, 29]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "verifyTxTestHelper", abi = "verifyTxTestHelper(bytes,uint256)")]
    pub struct VerifyTxTestHelperCall {
        pub header: ethers::core::types::Bytes,
        pub block_height: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `vm`function with signature `vm()` and selector `[58, 118, 132, 99]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "vm", abi = "vm()")]
    pub struct VmCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ThemelioBridgeTestCalls {
        IsTest(IsTestCall),
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        Burn(BurnCall),
        ComputeMerkleRootTestHelper(ComputeMerkleRootTestHelperCall),
        DealWithToken(DealWithTokenCall),
        DealWithTokenAndAdjust(DealWithTokenAndAdjustCall),
        Deal(DealCall),
        Decimals(DecimalsCall),
        DecodeIntegerTestHelper(DecodeIntegerTestHelperCall),
        DecreaseAllowance(DecreaseAllowanceCall),
        DeployCodeWithArgs(DeployCodeWithArgsCall),
        DeployCode(DeployCodeCall),
        Epochs(EpochsCall),
        ExtractBlockHeightTestHelper(ExtractBlockHeightTestHelperCall),
        ExtractValueAndRecipientTestHelper(ExtractValueAndRecipientTestHelperCall),
        Failed(FailedCall),
        Headers(HeadersCall),
        Hoax0(Hoax0Call),
        Hoax2(Hoax2Call),
        Hoax3(Hoax3Call),
        Hoax4(Hoax4Call),
        IncreaseAllowance(IncreaseAllowanceCall),
        Name(NameCall),
        RelayHeader(RelayHeaderCall),
        RelayHeaderTestHelper(RelayHeaderTestHelperCall),
        RelayStakers(RelayStakersCall),
        RelayStakersHelper(RelayStakersHelperCall),
        Rewind(RewindCall),
        Skip(SkipCall),
        StartHoax1(StartHoax1Call),
        StartHoax2(StartHoax2Call),
        StartHoax0(StartHoax0Call),
        StartHoax4(StartHoax4Call),
        Symbol(SymbolCall),
        TestBlake3Hasher(TestBlake3HasherCall),
        TestBlake3KeyedHasher(TestBlake3KeyedHasherCall),
        TestDecimals(TestDecimalsCall),
        TestEd25519(TestEd25519Call),
        TestEncodedIntegerSize(TestEncodedIntegerSizeCall),
        TestExtractMerkleRoot(TestExtractMerkleRootCall),
        TestExtractTokenType(TestExtractTokenTypeCall),
        TestHashDatablock(TestHashDatablockCall),
        TestHashNode(TestHashNodeCall),
        TestSlice(TestSliceCall),
        Tip(TipCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
        VerifyTx(VerifyTxCall),
        VerifyTxTestHelper(VerifyTxTestHelperCall),
        Vm(VmCall),
    }
    impl ethers::core::abi::AbiDecode for ThemelioBridgeTestCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <IsTestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestCalls::IsTest(decoded));
            }
            if let Ok(decoded) =
                <AllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestCalls::Allowance(decoded));
            }
            if let Ok(decoded) =
                <ApproveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestCalls::Approve(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestCalls::BalanceOf(decoded));
            }
            if let Ok(decoded) = <BurnCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(ThemelioBridgeTestCalls::Burn(decoded));
            }
            if let Ok(decoded) =
                <ComputeMerkleRootTestHelperCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ThemelioBridgeTestCalls::ComputeMerkleRootTestHelper(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <DealWithTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestCalls::DealWithToken(decoded));
            }
            if let Ok(decoded) =
                <DealWithTokenAndAdjustCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestCalls::DealWithTokenAndAdjust(decoded));
            }
            if let Ok(decoded) = <DealCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(ThemelioBridgeTestCalls::Deal(decoded));
            }
            if let Ok(decoded) =
                <DecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestCalls::Decimals(decoded));
            }
            if let Ok(decoded) =
                <DecodeIntegerTestHelperCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestCalls::DecodeIntegerTestHelper(decoded));
            }
            if let Ok(decoded) =
                <DecreaseAllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestCalls::DecreaseAllowance(decoded));
            }
            if let Ok(decoded) =
                <DeployCodeWithArgsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestCalls::DeployCodeWithArgs(decoded));
            }
            if let Ok(decoded) =
                <DeployCodeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestCalls::DeployCode(decoded));
            }
            if let Ok(decoded) = <EpochsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestCalls::Epochs(decoded));
            }
            if let Ok(decoded) =
                <ExtractBlockHeightTestHelperCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ThemelioBridgeTestCalls::ExtractBlockHeightTestHelper(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <ExtractValueAndRecipientTestHelperCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ThemelioBridgeTestCalls::ExtractValueAndRecipientTestHelper(
                    decoded,
                ));
            }
            if let Ok(decoded) = <FailedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestCalls::Failed(decoded));
            }
            if let Ok(decoded) =
                <HeadersCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestCalls::Headers(decoded));
            }
            if let Ok(decoded) = <Hoax0Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestCalls::Hoax0(decoded));
            }
            if let Ok(decoded) = <Hoax2Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestCalls::Hoax2(decoded));
            }
            if let Ok(decoded) = <Hoax3Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestCalls::Hoax3(decoded));
            }
            if let Ok(decoded) = <Hoax4Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestCalls::Hoax4(decoded));
            }
            if let Ok(decoded) =
                <IncreaseAllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestCalls::IncreaseAllowance(decoded));
            }
            if let Ok(decoded) = <NameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(ThemelioBridgeTestCalls::Name(decoded));
            }
            if let Ok(decoded) =
                <RelayHeaderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestCalls::RelayHeader(decoded));
            }
            if let Ok(decoded) =
                <RelayHeaderTestHelperCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestCalls::RelayHeaderTestHelper(decoded));
            }
            if let Ok(decoded) =
                <RelayStakersCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestCalls::RelayStakers(decoded));
            }
            if let Ok(decoded) =
                <RelayStakersHelperCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestCalls::RelayStakersHelper(decoded));
            }
            if let Ok(decoded) = <RewindCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestCalls::Rewind(decoded));
            }
            if let Ok(decoded) = <SkipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(ThemelioBridgeTestCalls::Skip(decoded));
            }
            if let Ok(decoded) =
                <StartHoax1Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestCalls::StartHoax1(decoded));
            }
            if let Ok(decoded) =
                <StartHoax2Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestCalls::StartHoax2(decoded));
            }
            if let Ok(decoded) =
                <StartHoax0Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestCalls::StartHoax0(decoded));
            }
            if let Ok(decoded) =
                <StartHoax4Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestCalls::StartHoax4(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestCalls::Symbol(decoded));
            }
            if let Ok(decoded) =
                <TestBlake3HasherCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestCalls::TestBlake3Hasher(decoded));
            }
            if let Ok(decoded) =
                <TestBlake3KeyedHasherCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestCalls::TestBlake3KeyedHasher(decoded));
            }
            if let Ok(decoded) =
                <TestDecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestCalls::TestDecimals(decoded));
            }
            if let Ok(decoded) =
                <TestEd25519Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestCalls::TestEd25519(decoded));
            }
            if let Ok(decoded) =
                <TestEncodedIntegerSizeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestCalls::TestEncodedIntegerSize(decoded));
            }
            if let Ok(decoded) =
                <TestExtractMerkleRootCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestCalls::TestExtractMerkleRoot(decoded));
            }
            if let Ok(decoded) =
                <TestExtractTokenTypeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestCalls::TestExtractTokenType(decoded));
            }
            if let Ok(decoded) =
                <TestHashDatablockCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestCalls::TestHashDatablock(decoded));
            }
            if let Ok(decoded) =
                <TestHashNodeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestCalls::TestHashNode(decoded));
            }
            if let Ok(decoded) =
                <TestSliceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestCalls::TestSlice(decoded));
            }
            if let Ok(decoded) = <TipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(ThemelioBridgeTestCalls::Tip(decoded));
            }
            if let Ok(decoded) =
                <TotalSupplyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestCalls::TotalSupply(decoded));
            }
            if let Ok(decoded) =
                <TransferCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestCalls::Transfer(decoded));
            }
            if let Ok(decoded) =
                <TransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestCalls::TransferFrom(decoded));
            }
            if let Ok(decoded) =
                <VerifyTxCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestCalls::VerifyTx(decoded));
            }
            if let Ok(decoded) =
                <VerifyTxTestHelperCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestCalls::VerifyTxTestHelper(decoded));
            }
            if let Ok(decoded) = <VmCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(ThemelioBridgeTestCalls::Vm(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ThemelioBridgeTestCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ThemelioBridgeTestCalls::IsTest(element) => element.encode(),
                ThemelioBridgeTestCalls::Allowance(element) => element.encode(),
                ThemelioBridgeTestCalls::Approve(element) => element.encode(),
                ThemelioBridgeTestCalls::BalanceOf(element) => element.encode(),
                ThemelioBridgeTestCalls::Burn(element) => element.encode(),
                ThemelioBridgeTestCalls::ComputeMerkleRootTestHelper(element) => element.encode(),
                ThemelioBridgeTestCalls::DealWithToken(element) => element.encode(),
                ThemelioBridgeTestCalls::DealWithTokenAndAdjust(element) => element.encode(),
                ThemelioBridgeTestCalls::Deal(element) => element.encode(),
                ThemelioBridgeTestCalls::Decimals(element) => element.encode(),
                ThemelioBridgeTestCalls::DecodeIntegerTestHelper(element) => element.encode(),
                ThemelioBridgeTestCalls::DecreaseAllowance(element) => element.encode(),
                ThemelioBridgeTestCalls::DeployCodeWithArgs(element) => element.encode(),
                ThemelioBridgeTestCalls::DeployCode(element) => element.encode(),
                ThemelioBridgeTestCalls::Epochs(element) => element.encode(),
                ThemelioBridgeTestCalls::ExtractBlockHeightTestHelper(element) => element.encode(),
                ThemelioBridgeTestCalls::ExtractValueAndRecipientTestHelper(element) => {
                    element.encode()
                }
                ThemelioBridgeTestCalls::Failed(element) => element.encode(),
                ThemelioBridgeTestCalls::Headers(element) => element.encode(),
                ThemelioBridgeTestCalls::Hoax0(element) => element.encode(),
                ThemelioBridgeTestCalls::Hoax2(element) => element.encode(),
                ThemelioBridgeTestCalls::Hoax3(element) => element.encode(),
                ThemelioBridgeTestCalls::Hoax4(element) => element.encode(),
                ThemelioBridgeTestCalls::IncreaseAllowance(element) => element.encode(),
                ThemelioBridgeTestCalls::Name(element) => element.encode(),
                ThemelioBridgeTestCalls::RelayHeader(element) => element.encode(),
                ThemelioBridgeTestCalls::RelayHeaderTestHelper(element) => element.encode(),
                ThemelioBridgeTestCalls::RelayStakers(element) => element.encode(),
                ThemelioBridgeTestCalls::RelayStakersHelper(element) => element.encode(),
                ThemelioBridgeTestCalls::Rewind(element) => element.encode(),
                ThemelioBridgeTestCalls::Skip(element) => element.encode(),
                ThemelioBridgeTestCalls::StartHoax1(element) => element.encode(),
                ThemelioBridgeTestCalls::StartHoax2(element) => element.encode(),
                ThemelioBridgeTestCalls::StartHoax0(element) => element.encode(),
                ThemelioBridgeTestCalls::StartHoax4(element) => element.encode(),
                ThemelioBridgeTestCalls::Symbol(element) => element.encode(),
                ThemelioBridgeTestCalls::TestBlake3Hasher(element) => element.encode(),
                ThemelioBridgeTestCalls::TestBlake3KeyedHasher(element) => element.encode(),
                ThemelioBridgeTestCalls::TestDecimals(element) => element.encode(),
                ThemelioBridgeTestCalls::TestEd25519(element) => element.encode(),
                ThemelioBridgeTestCalls::TestEncodedIntegerSize(element) => element.encode(),
                ThemelioBridgeTestCalls::TestExtractMerkleRoot(element) => element.encode(),
                ThemelioBridgeTestCalls::TestExtractTokenType(element) => element.encode(),
                ThemelioBridgeTestCalls::TestHashDatablock(element) => element.encode(),
                ThemelioBridgeTestCalls::TestHashNode(element) => element.encode(),
                ThemelioBridgeTestCalls::TestSlice(element) => element.encode(),
                ThemelioBridgeTestCalls::Tip(element) => element.encode(),
                ThemelioBridgeTestCalls::TotalSupply(element) => element.encode(),
                ThemelioBridgeTestCalls::Transfer(element) => element.encode(),
                ThemelioBridgeTestCalls::TransferFrom(element) => element.encode(),
                ThemelioBridgeTestCalls::VerifyTx(element) => element.encode(),
                ThemelioBridgeTestCalls::VerifyTxTestHelper(element) => element.encode(),
                ThemelioBridgeTestCalls::Vm(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ThemelioBridgeTestCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ThemelioBridgeTestCalls::IsTest(element) => element.fmt(f),
                ThemelioBridgeTestCalls::Allowance(element) => element.fmt(f),
                ThemelioBridgeTestCalls::Approve(element) => element.fmt(f),
                ThemelioBridgeTestCalls::BalanceOf(element) => element.fmt(f),
                ThemelioBridgeTestCalls::Burn(element) => element.fmt(f),
                ThemelioBridgeTestCalls::ComputeMerkleRootTestHelper(element) => element.fmt(f),
                ThemelioBridgeTestCalls::DealWithToken(element) => element.fmt(f),
                ThemelioBridgeTestCalls::DealWithTokenAndAdjust(element) => element.fmt(f),
                ThemelioBridgeTestCalls::Deal(element) => element.fmt(f),
                ThemelioBridgeTestCalls::Decimals(element) => element.fmt(f),
                ThemelioBridgeTestCalls::DecodeIntegerTestHelper(element) => element.fmt(f),
                ThemelioBridgeTestCalls::DecreaseAllowance(element) => element.fmt(f),
                ThemelioBridgeTestCalls::DeployCodeWithArgs(element) => element.fmt(f),
                ThemelioBridgeTestCalls::DeployCode(element) => element.fmt(f),
                ThemelioBridgeTestCalls::Epochs(element) => element.fmt(f),
                ThemelioBridgeTestCalls::ExtractBlockHeightTestHelper(element) => element.fmt(f),
                ThemelioBridgeTestCalls::ExtractValueAndRecipientTestHelper(element) => {
                    element.fmt(f)
                }
                ThemelioBridgeTestCalls::Failed(element) => element.fmt(f),
                ThemelioBridgeTestCalls::Headers(element) => element.fmt(f),
                ThemelioBridgeTestCalls::Hoax0(element) => element.fmt(f),
                ThemelioBridgeTestCalls::Hoax2(element) => element.fmt(f),
                ThemelioBridgeTestCalls::Hoax3(element) => element.fmt(f),
                ThemelioBridgeTestCalls::Hoax4(element) => element.fmt(f),
                ThemelioBridgeTestCalls::IncreaseAllowance(element) => element.fmt(f),
                ThemelioBridgeTestCalls::Name(element) => element.fmt(f),
                ThemelioBridgeTestCalls::RelayHeader(element) => element.fmt(f),
                ThemelioBridgeTestCalls::RelayHeaderTestHelper(element) => element.fmt(f),
                ThemelioBridgeTestCalls::RelayStakers(element) => element.fmt(f),
                ThemelioBridgeTestCalls::RelayStakersHelper(element) => element.fmt(f),
                ThemelioBridgeTestCalls::Rewind(element) => element.fmt(f),
                ThemelioBridgeTestCalls::Skip(element) => element.fmt(f),
                ThemelioBridgeTestCalls::StartHoax1(element) => element.fmt(f),
                ThemelioBridgeTestCalls::StartHoax2(element) => element.fmt(f),
                ThemelioBridgeTestCalls::StartHoax0(element) => element.fmt(f),
                ThemelioBridgeTestCalls::StartHoax4(element) => element.fmt(f),
                ThemelioBridgeTestCalls::Symbol(element) => element.fmt(f),
                ThemelioBridgeTestCalls::TestBlake3Hasher(element) => element.fmt(f),
                ThemelioBridgeTestCalls::TestBlake3KeyedHasher(element) => element.fmt(f),
                ThemelioBridgeTestCalls::TestDecimals(element) => element.fmt(f),
                ThemelioBridgeTestCalls::TestEd25519(element) => element.fmt(f),
                ThemelioBridgeTestCalls::TestEncodedIntegerSize(element) => element.fmt(f),
                ThemelioBridgeTestCalls::TestExtractMerkleRoot(element) => element.fmt(f),
                ThemelioBridgeTestCalls::TestExtractTokenType(element) => element.fmt(f),
                ThemelioBridgeTestCalls::TestHashDatablock(element) => element.fmt(f),
                ThemelioBridgeTestCalls::TestHashNode(element) => element.fmt(f),
                ThemelioBridgeTestCalls::TestSlice(element) => element.fmt(f),
                ThemelioBridgeTestCalls::Tip(element) => element.fmt(f),
                ThemelioBridgeTestCalls::TotalSupply(element) => element.fmt(f),
                ThemelioBridgeTestCalls::Transfer(element) => element.fmt(f),
                ThemelioBridgeTestCalls::TransferFrom(element) => element.fmt(f),
                ThemelioBridgeTestCalls::VerifyTx(element) => element.fmt(f),
                ThemelioBridgeTestCalls::VerifyTxTestHelper(element) => element.fmt(f),
                ThemelioBridgeTestCalls::Vm(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<IsTestCall> for ThemelioBridgeTestCalls {
        fn from(var: IsTestCall) -> Self {
            ThemelioBridgeTestCalls::IsTest(var)
        }
    }
    impl ::std::convert::From<AllowanceCall> for ThemelioBridgeTestCalls {
        fn from(var: AllowanceCall) -> Self {
            ThemelioBridgeTestCalls::Allowance(var)
        }
    }
    impl ::std::convert::From<ApproveCall> for ThemelioBridgeTestCalls {
        fn from(var: ApproveCall) -> Self {
            ThemelioBridgeTestCalls::Approve(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for ThemelioBridgeTestCalls {
        fn from(var: BalanceOfCall) -> Self {
            ThemelioBridgeTestCalls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<BurnCall> for ThemelioBridgeTestCalls {
        fn from(var: BurnCall) -> Self {
            ThemelioBridgeTestCalls::Burn(var)
        }
    }
    impl ::std::convert::From<ComputeMerkleRootTestHelperCall> for ThemelioBridgeTestCalls {
        fn from(var: ComputeMerkleRootTestHelperCall) -> Self {
            ThemelioBridgeTestCalls::ComputeMerkleRootTestHelper(var)
        }
    }
    impl ::std::convert::From<DealWithTokenCall> for ThemelioBridgeTestCalls {
        fn from(var: DealWithTokenCall) -> Self {
            ThemelioBridgeTestCalls::DealWithToken(var)
        }
    }
    impl ::std::convert::From<DealWithTokenAndAdjustCall> for ThemelioBridgeTestCalls {
        fn from(var: DealWithTokenAndAdjustCall) -> Self {
            ThemelioBridgeTestCalls::DealWithTokenAndAdjust(var)
        }
    }
    impl ::std::convert::From<DealCall> for ThemelioBridgeTestCalls {
        fn from(var: DealCall) -> Self {
            ThemelioBridgeTestCalls::Deal(var)
        }
    }
    impl ::std::convert::From<DecimalsCall> for ThemelioBridgeTestCalls {
        fn from(var: DecimalsCall) -> Self {
            ThemelioBridgeTestCalls::Decimals(var)
        }
    }
    impl ::std::convert::From<DecodeIntegerTestHelperCall> for ThemelioBridgeTestCalls {
        fn from(var: DecodeIntegerTestHelperCall) -> Self {
            ThemelioBridgeTestCalls::DecodeIntegerTestHelper(var)
        }
    }
    impl ::std::convert::From<DecreaseAllowanceCall> for ThemelioBridgeTestCalls {
        fn from(var: DecreaseAllowanceCall) -> Self {
            ThemelioBridgeTestCalls::DecreaseAllowance(var)
        }
    }
    impl ::std::convert::From<DeployCodeWithArgsCall> for ThemelioBridgeTestCalls {
        fn from(var: DeployCodeWithArgsCall) -> Self {
            ThemelioBridgeTestCalls::DeployCodeWithArgs(var)
        }
    }
    impl ::std::convert::From<DeployCodeCall> for ThemelioBridgeTestCalls {
        fn from(var: DeployCodeCall) -> Self {
            ThemelioBridgeTestCalls::DeployCode(var)
        }
    }
    impl ::std::convert::From<EpochsCall> for ThemelioBridgeTestCalls {
        fn from(var: EpochsCall) -> Self {
            ThemelioBridgeTestCalls::Epochs(var)
        }
    }
    impl ::std::convert::From<ExtractBlockHeightTestHelperCall> for ThemelioBridgeTestCalls {
        fn from(var: ExtractBlockHeightTestHelperCall) -> Self {
            ThemelioBridgeTestCalls::ExtractBlockHeightTestHelper(var)
        }
    }
    impl ::std::convert::From<ExtractValueAndRecipientTestHelperCall> for ThemelioBridgeTestCalls {
        fn from(var: ExtractValueAndRecipientTestHelperCall) -> Self {
            ThemelioBridgeTestCalls::ExtractValueAndRecipientTestHelper(var)
        }
    }
    impl ::std::convert::From<FailedCall> for ThemelioBridgeTestCalls {
        fn from(var: FailedCall) -> Self {
            ThemelioBridgeTestCalls::Failed(var)
        }
    }
    impl ::std::convert::From<HeadersCall> for ThemelioBridgeTestCalls {
        fn from(var: HeadersCall) -> Self {
            ThemelioBridgeTestCalls::Headers(var)
        }
    }
    impl ::std::convert::From<Hoax0Call> for ThemelioBridgeTestCalls {
        fn from(var: Hoax0Call) -> Self {
            ThemelioBridgeTestCalls::Hoax0(var)
        }
    }
    impl ::std::convert::From<Hoax2Call> for ThemelioBridgeTestCalls {
        fn from(var: Hoax2Call) -> Self {
            ThemelioBridgeTestCalls::Hoax2(var)
        }
    }
    impl ::std::convert::From<Hoax3Call> for ThemelioBridgeTestCalls {
        fn from(var: Hoax3Call) -> Self {
            ThemelioBridgeTestCalls::Hoax3(var)
        }
    }
    impl ::std::convert::From<Hoax4Call> for ThemelioBridgeTestCalls {
        fn from(var: Hoax4Call) -> Self {
            ThemelioBridgeTestCalls::Hoax4(var)
        }
    }
    impl ::std::convert::From<IncreaseAllowanceCall> for ThemelioBridgeTestCalls {
        fn from(var: IncreaseAllowanceCall) -> Self {
            ThemelioBridgeTestCalls::IncreaseAllowance(var)
        }
    }
    impl ::std::convert::From<NameCall> for ThemelioBridgeTestCalls {
        fn from(var: NameCall) -> Self {
            ThemelioBridgeTestCalls::Name(var)
        }
    }
    impl ::std::convert::From<RelayHeaderCall> for ThemelioBridgeTestCalls {
        fn from(var: RelayHeaderCall) -> Self {
            ThemelioBridgeTestCalls::RelayHeader(var)
        }
    }
    impl ::std::convert::From<RelayHeaderTestHelperCall> for ThemelioBridgeTestCalls {
        fn from(var: RelayHeaderTestHelperCall) -> Self {
            ThemelioBridgeTestCalls::RelayHeaderTestHelper(var)
        }
    }
    impl ::std::convert::From<RelayStakersCall> for ThemelioBridgeTestCalls {
        fn from(var: RelayStakersCall) -> Self {
            ThemelioBridgeTestCalls::RelayStakers(var)
        }
    }
    impl ::std::convert::From<RelayStakersHelperCall> for ThemelioBridgeTestCalls {
        fn from(var: RelayStakersHelperCall) -> Self {
            ThemelioBridgeTestCalls::RelayStakersHelper(var)
        }
    }
    impl ::std::convert::From<RewindCall> for ThemelioBridgeTestCalls {
        fn from(var: RewindCall) -> Self {
            ThemelioBridgeTestCalls::Rewind(var)
        }
    }
    impl ::std::convert::From<SkipCall> for ThemelioBridgeTestCalls {
        fn from(var: SkipCall) -> Self {
            ThemelioBridgeTestCalls::Skip(var)
        }
    }
    impl ::std::convert::From<StartHoax1Call> for ThemelioBridgeTestCalls {
        fn from(var: StartHoax1Call) -> Self {
            ThemelioBridgeTestCalls::StartHoax1(var)
        }
    }
    impl ::std::convert::From<StartHoax2Call> for ThemelioBridgeTestCalls {
        fn from(var: StartHoax2Call) -> Self {
            ThemelioBridgeTestCalls::StartHoax2(var)
        }
    }
    impl ::std::convert::From<StartHoax0Call> for ThemelioBridgeTestCalls {
        fn from(var: StartHoax0Call) -> Self {
            ThemelioBridgeTestCalls::StartHoax0(var)
        }
    }
    impl ::std::convert::From<StartHoax4Call> for ThemelioBridgeTestCalls {
        fn from(var: StartHoax4Call) -> Self {
            ThemelioBridgeTestCalls::StartHoax4(var)
        }
    }
    impl ::std::convert::From<SymbolCall> for ThemelioBridgeTestCalls {
        fn from(var: SymbolCall) -> Self {
            ThemelioBridgeTestCalls::Symbol(var)
        }
    }
    impl ::std::convert::From<TestBlake3HasherCall> for ThemelioBridgeTestCalls {
        fn from(var: TestBlake3HasherCall) -> Self {
            ThemelioBridgeTestCalls::TestBlake3Hasher(var)
        }
    }
    impl ::std::convert::From<TestBlake3KeyedHasherCall> for ThemelioBridgeTestCalls {
        fn from(var: TestBlake3KeyedHasherCall) -> Self {
            ThemelioBridgeTestCalls::TestBlake3KeyedHasher(var)
        }
    }
    impl ::std::convert::From<TestDecimalsCall> for ThemelioBridgeTestCalls {
        fn from(var: TestDecimalsCall) -> Self {
            ThemelioBridgeTestCalls::TestDecimals(var)
        }
    }
    impl ::std::convert::From<TestEd25519Call> for ThemelioBridgeTestCalls {
        fn from(var: TestEd25519Call) -> Self {
            ThemelioBridgeTestCalls::TestEd25519(var)
        }
    }
    impl ::std::convert::From<TestEncodedIntegerSizeCall> for ThemelioBridgeTestCalls {
        fn from(var: TestEncodedIntegerSizeCall) -> Self {
            ThemelioBridgeTestCalls::TestEncodedIntegerSize(var)
        }
    }
    impl ::std::convert::From<TestExtractMerkleRootCall> for ThemelioBridgeTestCalls {
        fn from(var: TestExtractMerkleRootCall) -> Self {
            ThemelioBridgeTestCalls::TestExtractMerkleRoot(var)
        }
    }
    impl ::std::convert::From<TestExtractTokenTypeCall> for ThemelioBridgeTestCalls {
        fn from(var: TestExtractTokenTypeCall) -> Self {
            ThemelioBridgeTestCalls::TestExtractTokenType(var)
        }
    }
    impl ::std::convert::From<TestHashDatablockCall> for ThemelioBridgeTestCalls {
        fn from(var: TestHashDatablockCall) -> Self {
            ThemelioBridgeTestCalls::TestHashDatablock(var)
        }
    }
    impl ::std::convert::From<TestHashNodeCall> for ThemelioBridgeTestCalls {
        fn from(var: TestHashNodeCall) -> Self {
            ThemelioBridgeTestCalls::TestHashNode(var)
        }
    }
    impl ::std::convert::From<TestSliceCall> for ThemelioBridgeTestCalls {
        fn from(var: TestSliceCall) -> Self {
            ThemelioBridgeTestCalls::TestSlice(var)
        }
    }
    impl ::std::convert::From<TipCall> for ThemelioBridgeTestCalls {
        fn from(var: TipCall) -> Self {
            ThemelioBridgeTestCalls::Tip(var)
        }
    }
    impl ::std::convert::From<TotalSupplyCall> for ThemelioBridgeTestCalls {
        fn from(var: TotalSupplyCall) -> Self {
            ThemelioBridgeTestCalls::TotalSupply(var)
        }
    }
    impl ::std::convert::From<TransferCall> for ThemelioBridgeTestCalls {
        fn from(var: TransferCall) -> Self {
            ThemelioBridgeTestCalls::Transfer(var)
        }
    }
    impl ::std::convert::From<TransferFromCall> for ThemelioBridgeTestCalls {
        fn from(var: TransferFromCall) -> Self {
            ThemelioBridgeTestCalls::TransferFrom(var)
        }
    }
    impl ::std::convert::From<VerifyTxCall> for ThemelioBridgeTestCalls {
        fn from(var: VerifyTxCall) -> Self {
            ThemelioBridgeTestCalls::VerifyTx(var)
        }
    }
    impl ::std::convert::From<VerifyTxTestHelperCall> for ThemelioBridgeTestCalls {
        fn from(var: VerifyTxTestHelperCall) -> Self {
            ThemelioBridgeTestCalls::VerifyTxTestHelper(var)
        }
    }
    impl ::std::convert::From<VmCall> for ThemelioBridgeTestCalls {
        fn from(var: VmCall) -> Self {
            ThemelioBridgeTestCalls::Vm(var)
        }
    }
}
