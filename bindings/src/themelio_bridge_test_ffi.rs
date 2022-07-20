pub use themeliobridgetestffi_mod::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod themeliobridgetestffi_mod {
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
    #[doc = "ThemelioBridgeTestFFI was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static THEMELIOBRIDGETESTFFI_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[],\"type\":\"error\",\"name\":\"ERC1155NotOwnerOrApproved\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"height\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"HeaderAlreadyVerified\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"HeaderNotVerified\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"covhash\",\"type\":\"bytes32\",\"components\":[]}],\"type\":\"error\",\"name\":\"InvalidCovhash\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidStakes\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"verifierHeight\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"headerHeight\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"InvalidVerifier\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"MalformedData\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"height\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"MissingHeader\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"height\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"MissingVerifier\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"NotOwner\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"start\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"end\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"dataLength\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"OutOfBoundsSlice\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"txHash\",\"type\":\"bytes32\",\"components\":[]}],\"type\":\"error\",\"name\":\"TxAlreadyVerified\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"TxNotVerified\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"newAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"AdminChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"approved\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ApprovalForAll\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"beacon\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"BeaconUpgraded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"height\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"HeaderVerified\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"version\",\"type\":\"uint8\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Initialized\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"keccakStakesHash\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes32\",\"name\":\"blake3StakesHash\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"StakesVerified\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"themelioRecipient\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"TokensBurned\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"TokensMinted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256[]\",\"name\":\"ids\",\"type\":\"uint256[]\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256[]\",\"name\":\"values\",\"type\":\"uint256[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TransferBatch\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TransferSingle\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"tx_hash\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"height\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"TxVerified\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"value\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"URI\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"implementation\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"Upgraded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_address\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256[]\",\"name\":\"val\",\"type\":\"uint256[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_array\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"int256[]\",\"name\":\"val\",\"type\":\"int256[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_array\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"val\",\"type\":\"address[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_array\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_bytes\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_bytes32\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"\",\"type\":\"int256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_int\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"val\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_address\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256[]\",\"name\":\"val\",\"type\":\"uint256[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_array\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256[]\",\"name\":\"val\",\"type\":\"int256[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_array\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"address[]\",\"name\":\"val\",\"type\":\"address[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_array\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"val\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_bytes\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes32\",\"name\":\"val\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_bytes32\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256\",\"name\":\"val\",\"type\":\"int256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"decimals\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_decimal_int\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"val\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"decimals\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_decimal_uint\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256\",\"name\":\"val\",\"type\":\"int256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_int\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"val\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_string\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"val\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_uint\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_string\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_uint\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"logs\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"IS_SCRIPT\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"IS_TEST\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"accounts\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"ids\",\"type\":\"uint256[]\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOfBatch\",\"outputs\":[{\"internalType\":\"uint256[]\",\"name\":\"\",\"type\":\"uint256[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"id_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"themelioRecipient_\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"burn\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"ids_\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"values_\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"themelioRecipient_\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"burnBatch\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"txHash\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32[]\",\"name\":\"proof\",\"type\":\"bytes32[]\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"computeMerkleRootHelper\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"give\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deal\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"give\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"adjust\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deal\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"give\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deal\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"header_\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"decodeHeaderHelper\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"encodedStakeDoc_\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"decodeStakeDocHelper\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"decodeTransactionHelper\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"denom\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"denomToStringHelper\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"what\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"args\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deployCode\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"addr\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"what\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deployCode\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"addr\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"failed\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"hashDatablockHelper\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"headerLimbo\",\"outputs\":[{\"internalType\":\"uint128\",\"name\":\"votes\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"bytesVerified\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"stakeDocIndex\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"headers\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"transactionsHash\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"stakesHash\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"who\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"hoax\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"who\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"origin\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"hoax\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"who\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"origin\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"give\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"hoax\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"who\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"give\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"hoax\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"blockHeight_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"transactionsHash_\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"stakesHash_\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isApprovedForAll\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mintHelper\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"proxiableUUID\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"time\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"rewind\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"ids\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"amounts\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"safeBatchTransferFrom\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"safeTransferFrom\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"approved\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setApprovalForAll\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"time\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"skip\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"spends\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"who\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"give\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"startHoax\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"who\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"origin\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"give\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"startHoax\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"who\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"startHoax\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"who\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"origin\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"startHoax\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes4\",\"name\":\"interfaceId\",\"type\":\"bytes4\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"supportsInterface\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"testBlake3DifferentialFFI\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint128\",\"name\":\"integer\",\"type\":\"uint128\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"testDecodeIntegerDifferentialFFI\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"message\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"testEd25519DifferentialFFI\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"testKeccakBigHashFFI\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"start\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"end\",\"type\":\"uint8\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"testSliceDifferentialFFI\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"give\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"tip\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newImplementation\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"upgradeTo\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newImplementation\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"upgradeToAndCall\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"uri\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"verifierHeight_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"header_\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"stakes_\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes32[]\",\"name\":\"signatures_\",\"type\":\"bytes32[]\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"firstTime_\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"verifyHeader\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"verifierStakesHash\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"verifierHeight\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"verifyHeaderHelper\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"stakes_\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"verifyStakes\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"transaction_\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"txIndex_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"blockHeight_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32[]\",\"name\":\"proof_\",\"type\":\"bytes32[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"verifyTx\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"blockHeight\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"transactionsHash\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"stakesHash\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"verifyTxHelper\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"vm\",\"outputs\":[{\"internalType\":\"contract Vm\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    pub struct ThemelioBridgeTestFFI<M: Clone>(ethers::contract::Contract<M>);
    impl<M: Clone> Clone for ThemelioBridgeTestFFI<M> {
        fn clone(&self) -> Self {
            ThemelioBridgeTestFFI(self.0.clone())
        }
    }
    impl<M: Clone> std::ops::Deref for ThemelioBridgeTestFFI<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware + Clone> std::fmt::Debug for ThemelioBridgeTestFFI<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ThemelioBridgeTestFFI))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware + Clone> ThemelioBridgeTestFFI<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                THEMELIOBRIDGETESTFFI_ABI.clone(),
                client,
            )
            .into()
        }
        #[doc = "Calls the contract's `IS_SCRIPT` (0xf8ccbf47) function"]
        pub fn is_script(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([248, 204, 191, 71], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `IS_TEST` (0xfa7626d4) function"]
        pub fn is_test(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([250, 118, 38, 212], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `balanceOf` (0x00fdd58e) function"]
        pub fn balance_of(
            &self,
            account: ethers::core::types::Address,
            id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([0, 253, 213, 142], (account, id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `balanceOfBatch` (0x4e1273f4) function"]
        pub fn balance_of_batch(
            &self,
            accounts: ::std::vec::Vec<ethers::core::types::Address>,
            ids: ::std::vec::Vec<ethers::core::types::U256>,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<ethers::core::types::U256>>
        {
            self.0
                .method_hash([78, 18, 115, 244], (accounts, ids))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `burn` (0x66d8caa0) function"]
        pub fn burn(
            &self,
            account: ethers::core::types::Address,
            id: ethers::core::types::U256,
            value: ethers::core::types::U256,
            themelio_recipient: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [102, 216, 202, 160],
                    (account, id, value, themelio_recipient),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `burnBatch` (0xc02b0cb5) function"]
        pub fn burn_batch(
            &self,
            account: ethers::core::types::Address,
            ids: ::std::vec::Vec<ethers::core::types::U256>,
            values: ::std::vec::Vec<ethers::core::types::U256>,
            themelio_recipient: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [192, 43, 12, 181],
                    (account, ids, values, themelio_recipient),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `computeMerkleRootHelper` (0x9b7d42a1) function"]
        pub fn compute_merkle_root_helper(
            &self,
            tx_hash: [u8; 32],
            index: ethers::core::types::U256,
            proof: ::std::vec::Vec<[u8; 32]>,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([155, 125, 66, 161], (tx_hash, index, proof))
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
        #[doc = "Calls the contract's `decodeHeaderHelper` (0x863e2c31) function"]
        pub fn decode_header_helper(
            &self,
            header: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, [u8; 32], [u8; 32]),
        > {
            self.0
                .method_hash([134, 62, 44, 49], header)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decodeStakeDocHelper` (0x22c37d1b) function"]
        pub fn decode_stake_doc_helper(
            &self,
            encoded_stake_doc: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                [u8; 32],
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([34, 195, 125, 27], encoded_stake_doc)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decodeTransactionHelper` (0x31748095) function"]
        pub fn decode_transaction_helper(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([49, 116, 128, 149], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `denomToStringHelper` (0x03e47d44) function"]
        pub fn denom_to_string_helper(
            &self,
            denom: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([3, 228, 125, 68], denom)
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
        #[doc = "Calls the contract's `failed` (0xba414fa6) function"]
        pub fn failed(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([186, 65, 79, 166], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `hashDatablockHelper` (0xa984e452) function"]
        pub fn hash_datablock_helper(
            &self,
            data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([169, 132, 228, 82], data)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `headerLimbo` (0xae567a4b) function"]
        pub fn header_limbo(
            &self,
            p0: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, (u128, u64, u64)> {
            self.0
                .method_hash([174, 86, 122, 75], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `headers` (0x56f90d79) function"]
        pub fn headers(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ([u8; 32], [u8; 32])> {
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
        pub fn hoax_1(
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
        pub fn hoax_2(
            &self,
            who: ethers::core::types::Address,
            give: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([233, 167, 154, 123], (who, give))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initialize` (0x09f10a3c) function"]
        pub fn initialize(
            &self,
            block_height: ethers::core::types::U256,
            transactions_hash: [u8; 32],
            stakes_hash: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [9, 241, 10, 60],
                    (block_height, transactions_hash, stakes_hash),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isApprovedForAll` (0xe985e9c5) function"]
        pub fn is_approved_for_all(
            &self,
            account: ethers::core::types::Address,
            operator: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([233, 133, 233, 197], (account, operator))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mintHelper` (0xea8cedb1) function"]
        pub fn mint_helper(
            &self,
            account: ethers::core::types::Address,
            id: ethers::core::types::U256,
            value: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([234, 140, 237, 177], (account, id, value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `proxiableUUID` (0x52d1902d) function"]
        pub fn proxiable_uuid(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([82, 209, 144, 45], ())
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
        #[doc = "Calls the contract's `safeBatchTransferFrom` (0x2eb2c2d6) function"]
        pub fn safe_batch_transfer_from(
            &self,
            from: ethers::core::types::Address,
            to: ethers::core::types::Address,
            ids: ::std::vec::Vec<ethers::core::types::U256>,
            amounts: ::std::vec::Vec<ethers::core::types::U256>,
            data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([46, 178, 194, 214], (from, to, ids, amounts, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `safeTransferFrom` (0xf242432a) function"]
        pub fn safe_transfer_from(
            &self,
            from: ethers::core::types::Address,
            to: ethers::core::types::Address,
            id: ethers::core::types::U256,
            amount: ethers::core::types::U256,
            data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 66, 67, 42], (from, to, id, amount, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setApprovalForAll` (0xa22cb465) function"]
        pub fn set_approval_for_all(
            &self,
            operator: ethers::core::types::Address,
            approved: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([162, 44, 180, 101], (operator, approved))
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
        #[doc = "Calls the contract's `spends` (0xfd367df0) function"]
        pub fn spends(&self, p0: [u8; 32]) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([253, 54, 125, 240], p0)
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
        pub fn start_hoax_3(
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
        pub fn start_hoax_2(
            &self,
            who: ethers::core::types::Address,
            origin: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([208, 109, 130, 41], (who, origin))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `supportsInterface` (0x01ffc9a7) function"]
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `testBlake3DifferentialFFI` (0x3a6dcf1d) function"]
        pub fn test_blake_3_differential_ffi(
            &self,
            data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([58, 109, 207, 29], data)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `testDecodeIntegerDifferentialFFI` (0x2ed5db4f) function"]
        pub fn test_decode_integer_differential_ffi(
            &self,
            integer: u128,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([46, 213, 219, 79], integer)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `testEd25519DifferentialFFI` (0x0dbd3927) function"]
        pub fn test_ed_25519_differential_ffi(
            &self,
            message: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([13, 189, 57, 39], message)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `testKeccakBigHashFFI` (0x7f52c271) function"]
        pub fn test_keccak_big_hash_ffi(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([127, 82, 194, 113], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `testSliceDifferentialFFI` (0x2e69613f) function"]
        pub fn test_slice_differential_ffi(
            &self,
            data: ethers::core::types::Bytes,
            start: u8,
            end: u8,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([46, 105, 97, 63], (data, start, end))
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
        #[doc = "Calls the contract's `upgradeTo` (0x3659cfe6) function"]
        pub fn upgrade_to(
            &self,
            new_implementation: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([54, 89, 207, 230], new_implementation)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `upgradeToAndCall` (0x4f1ef286) function"]
        pub fn upgrade_to_and_call(
            &self,
            new_implementation: ethers::core::types::Address,
            data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([79, 30, 242, 134], (new_implementation, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `uri` (0x0e89341c) function"]
        pub fn uri(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([14, 137, 52, 28], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `verifyHeader` (0x703cfbf0) function"]
        pub fn verify_header(
            &self,
            verifier_height: ethers::core::types::U256,
            header: ethers::core::types::Bytes,
            stakes: ethers::core::types::Bytes,
            signatures: ::std::vec::Vec<[u8; 32]>,
            first_time: bool,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [112, 60, 251, 240],
                    (verifier_height, header, stakes, signatures, first_time),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `verifyHeaderHelper` (0x6a2ff71f) function"]
        pub fn verify_header_helper(
            &self,
            verifier_stakes_hash: [u8; 32],
            verifier_height: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([106, 47, 247, 31], (verifier_stakes_hash, verifier_height))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `verifyStakes` (0x6c0fd94c) function"]
        pub fn verify_stakes(
            &self,
            stakes: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([108, 15, 217, 76], stakes)
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
        #[doc = "Calls the contract's `verifyTxHelper` (0xaa9a46d6) function"]
        pub fn verify_tx_helper(
            &self,
            block_height: ethers::core::types::U256,
            transactions_hash: [u8; 32],
            stakes_hash: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [170, 154, 70, 214],
                    (block_height, transactions_hash, stakes_hash),
                )
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
        #[doc = "Gets the contract's `AdminChanged` event"]
        pub fn admin_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, AdminChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ApprovalForAll` event"]
        pub fn approval_for_all_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ApprovalForAllFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `BeaconUpgraded` event"]
        pub fn beacon_upgraded_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, BeaconUpgradedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `HeaderVerified` event"]
        pub fn header_verified_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, HeaderVerifiedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Initialized` event"]
        pub fn initialized_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, InitializedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `StakesVerified` event"]
        pub fn stakes_verified_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, StakesVerifiedFilter> {
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
        #[doc = "Gets the contract's `TransferBatch` event"]
        pub fn transfer_batch_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TransferBatchFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TransferSingle` event"]
        pub fn transfer_single_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TransferSingleFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TxVerified` event"]
        pub fn tx_verified_filter(&self) -> ethers::contract::builders::Event<M, TxVerifiedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `URI` event"]
        pub fn uri_filter(&self) -> ethers::contract::builders::Event<M, UriFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Upgraded` event"]
        pub fn upgraded_filter(&self) -> ethers::contract::builders::Event<M, UpgradedFilter> {
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
        #[doc = "Gets the contract's `log_array` event"]
        pub fn log_array_1_filter(&self) -> ethers::contract::builders::Event<M, LogArray1Filter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_array` event"]
        pub fn log_array_2_filter(&self) -> ethers::contract::builders::Event<M, LogArray2Filter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_array` event"]
        pub fn log_array_3_filter(&self) -> ethers::contract::builders::Event<M, LogArray3Filter> {
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
        #[doc = "Gets the contract's `log_named_array` event"]
        pub fn log_named_array_1_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogNamedArray1Filter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_named_array` event"]
        pub fn log_named_array_2_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogNamedArray2Filter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_named_array` event"]
        pub fn log_named_array_3_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogNamedArray3Filter> {
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
        pub fn events(&self) -> ethers::contract::builders::Event<M, ThemelioBridgeTestFFIEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware + Clone> From<ethers::contract::Contract<M>>
        for ThemelioBridgeTestFFI<M>
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
    #[ethevent(name = "AdminChanged", abi = "AdminChanged(address,address)")]
    pub struct AdminChangedFilter {
        pub previous_admin: ethers::core::types::Address,
        pub new_admin: ethers::core::types::Address,
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
    #[ethevent(name = "ApprovalForAll", abi = "ApprovalForAll(address,address,bool)")]
    pub struct ApprovalForAllFilter {
        #[ethevent(indexed)]
        pub account: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub operator: ethers::core::types::Address,
        pub approved: bool,
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
    #[ethevent(name = "BeaconUpgraded", abi = "BeaconUpgraded(address)")]
    pub struct BeaconUpgradedFilter {
        #[ethevent(indexed)]
        pub beacon: ethers::core::types::Address,
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
    #[ethevent(name = "HeaderVerified", abi = "HeaderVerified(uint256)")]
    pub struct HeaderVerifiedFilter {
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
    #[ethevent(name = "Initialized", abi = "Initialized(uint8)")]
    pub struct InitializedFilter {
        pub version: u8,
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
    #[ethevent(name = "StakesVerified", abi = "StakesVerified(bytes32,bytes32)")]
    pub struct StakesVerifiedFilter {
        pub keccak_stakes_hash: [u8; 32],
        pub blake_3_stakes_hash: [u8; 32],
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
    #[ethevent(name = "TokensBurned", abi = "TokensBurned(bytes32)")]
    pub struct TokensBurnedFilter {
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
        pub account: ethers::core::types::Address,
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
    #[ethevent(
        name = "TransferBatch",
        abi = "TransferBatch(address,address,address,uint256[],uint256[])"
    )]
    pub struct TransferBatchFilter {
        #[ethevent(indexed)]
        pub operator: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub from: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
        pub ids: Vec<ethers::core::types::U256>,
        pub values: Vec<ethers::core::types::U256>,
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
        name = "TransferSingle",
        abi = "TransferSingle(address,address,address,uint256,uint256)"
    )]
    pub struct TransferSingleFilter {
        #[ethevent(indexed)]
        pub operator: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub from: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
        pub id: ethers::core::types::U256,
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
    #[ethevent(name = "URI", abi = "URI(string,uint256)")]
    pub struct UriFilter {
        pub value: String,
        #[ethevent(indexed)]
        pub id: ethers::core::types::U256,
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
    #[ethevent(name = "Upgraded", abi = "Upgraded(address)")]
    pub struct UpgradedFilter {
        #[ethevent(indexed)]
        pub implementation: ethers::core::types::Address,
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
    #[ethevent(name = "log_array", abi = "log_array(uint256[])")]
    pub struct LogArray1Filter {
        pub val: Vec<ethers::core::types::U256>,
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
    #[ethevent(name = "log_array", abi = "log_array(int256[])")]
    pub struct LogArray2Filter {
        pub val: Vec<I256>,
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
    #[ethevent(name = "log_array", abi = "log_array(address[])")]
    pub struct LogArray3Filter {
        pub val: Vec<ethers::core::types::Address>,
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
    #[ethevent(name = "log_named_array", abi = "log_named_array(string,uint256[])")]
    pub struct LogNamedArray1Filter {
        pub key: String,
        pub val: Vec<ethers::core::types::U256>,
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
    #[ethevent(name = "log_named_array", abi = "log_named_array(string,int256[])")]
    pub struct LogNamedArray2Filter {
        pub key: String,
        pub val: Vec<I256>,
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
    #[ethevent(name = "log_named_array", abi = "log_named_array(string,address[])")]
    pub struct LogNamedArray3Filter {
        pub key: String,
        pub val: Vec<ethers::core::types::Address>,
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
    pub enum ThemelioBridgeTestFFIEvents {
        AdminChangedFilter(AdminChangedFilter),
        ApprovalForAllFilter(ApprovalForAllFilter),
        BeaconUpgradedFilter(BeaconUpgradedFilter),
        HeaderVerifiedFilter(HeaderVerifiedFilter),
        InitializedFilter(InitializedFilter),
        StakesVerifiedFilter(StakesVerifiedFilter),
        TokensBurnedFilter(TokensBurnedFilter),
        TokensMintedFilter(TokensMintedFilter),
        TransferBatchFilter(TransferBatchFilter),
        TransferSingleFilter(TransferSingleFilter),
        TxVerifiedFilter(TxVerifiedFilter),
        UriFilter(UriFilter),
        UpgradedFilter(UpgradedFilter),
        LogFilter(LogFilter),
        LogAddressFilter(LogAddressFilter),
        LogArray1Filter(LogArray1Filter),
        LogArray2Filter(LogArray2Filter),
        LogArray3Filter(LogArray3Filter),
        LogBytesFilter(LogBytesFilter),
        LogBytes32Filter(LogBytes32Filter),
        LogIntFilter(LogIntFilter),
        LogNamedAddressFilter(LogNamedAddressFilter),
        LogNamedArray1Filter(LogNamedArray1Filter),
        LogNamedArray2Filter(LogNamedArray2Filter),
        LogNamedArray3Filter(LogNamedArray3Filter),
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
    impl ethers::contract::EthLogDecode for ThemelioBridgeTestFFIEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AdminChangedFilter::decode_log(log) {
                return Ok(ThemelioBridgeTestFFIEvents::AdminChangedFilter(decoded));
            }
            if let Ok(decoded) = ApprovalForAllFilter::decode_log(log) {
                return Ok(ThemelioBridgeTestFFIEvents::ApprovalForAllFilter(decoded));
            }
            if let Ok(decoded) = BeaconUpgradedFilter::decode_log(log) {
                return Ok(ThemelioBridgeTestFFIEvents::BeaconUpgradedFilter(decoded));
            }
            if let Ok(decoded) = HeaderVerifiedFilter::decode_log(log) {
                return Ok(ThemelioBridgeTestFFIEvents::HeaderVerifiedFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(ThemelioBridgeTestFFIEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = StakesVerifiedFilter::decode_log(log) {
                return Ok(ThemelioBridgeTestFFIEvents::StakesVerifiedFilter(decoded));
            }
            if let Ok(decoded) = TokensBurnedFilter::decode_log(log) {
                return Ok(ThemelioBridgeTestFFIEvents::TokensBurnedFilter(decoded));
            }
            if let Ok(decoded) = TokensMintedFilter::decode_log(log) {
                return Ok(ThemelioBridgeTestFFIEvents::TokensMintedFilter(decoded));
            }
            if let Ok(decoded) = TransferBatchFilter::decode_log(log) {
                return Ok(ThemelioBridgeTestFFIEvents::TransferBatchFilter(decoded));
            }
            if let Ok(decoded) = TransferSingleFilter::decode_log(log) {
                return Ok(ThemelioBridgeTestFFIEvents::TransferSingleFilter(decoded));
            }
            if let Ok(decoded) = TxVerifiedFilter::decode_log(log) {
                return Ok(ThemelioBridgeTestFFIEvents::TxVerifiedFilter(decoded));
            }
            if let Ok(decoded) = UriFilter::decode_log(log) {
                return Ok(ThemelioBridgeTestFFIEvents::UriFilter(decoded));
            }
            if let Ok(decoded) = UpgradedFilter::decode_log(log) {
                return Ok(ThemelioBridgeTestFFIEvents::UpgradedFilter(decoded));
            }
            if let Ok(decoded) = LogFilter::decode_log(log) {
                return Ok(ThemelioBridgeTestFFIEvents::LogFilter(decoded));
            }
            if let Ok(decoded) = LogAddressFilter::decode_log(log) {
                return Ok(ThemelioBridgeTestFFIEvents::LogAddressFilter(decoded));
            }
            if let Ok(decoded) = LogArray1Filter::decode_log(log) {
                return Ok(ThemelioBridgeTestFFIEvents::LogArray1Filter(decoded));
            }
            if let Ok(decoded) = LogArray2Filter::decode_log(log) {
                return Ok(ThemelioBridgeTestFFIEvents::LogArray2Filter(decoded));
            }
            if let Ok(decoded) = LogArray3Filter::decode_log(log) {
                return Ok(ThemelioBridgeTestFFIEvents::LogArray3Filter(decoded));
            }
            if let Ok(decoded) = LogBytesFilter::decode_log(log) {
                return Ok(ThemelioBridgeTestFFIEvents::LogBytesFilter(decoded));
            }
            if let Ok(decoded) = LogBytes32Filter::decode_log(log) {
                return Ok(ThemelioBridgeTestFFIEvents::LogBytes32Filter(decoded));
            }
            if let Ok(decoded) = LogIntFilter::decode_log(log) {
                return Ok(ThemelioBridgeTestFFIEvents::LogIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedAddressFilter::decode_log(log) {
                return Ok(ThemelioBridgeTestFFIEvents::LogNamedAddressFilter(decoded));
            }
            if let Ok(decoded) = LogNamedArray1Filter::decode_log(log) {
                return Ok(ThemelioBridgeTestFFIEvents::LogNamedArray1Filter(decoded));
            }
            if let Ok(decoded) = LogNamedArray2Filter::decode_log(log) {
                return Ok(ThemelioBridgeTestFFIEvents::LogNamedArray2Filter(decoded));
            }
            if let Ok(decoded) = LogNamedArray3Filter::decode_log(log) {
                return Ok(ThemelioBridgeTestFFIEvents::LogNamedArray3Filter(decoded));
            }
            if let Ok(decoded) = LogNamedBytesFilter::decode_log(log) {
                return Ok(ThemelioBridgeTestFFIEvents::LogNamedBytesFilter(decoded));
            }
            if let Ok(decoded) = LogNamedBytes32Filter::decode_log(log) {
                return Ok(ThemelioBridgeTestFFIEvents::LogNamedBytes32Filter(decoded));
            }
            if let Ok(decoded) = LogNamedDecimalIntFilter::decode_log(log) {
                return Ok(ThemelioBridgeTestFFIEvents::LogNamedDecimalIntFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = LogNamedDecimalUintFilter::decode_log(log) {
                return Ok(ThemelioBridgeTestFFIEvents::LogNamedDecimalUintFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = LogNamedIntFilter::decode_log(log) {
                return Ok(ThemelioBridgeTestFFIEvents::LogNamedIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedStringFilter::decode_log(log) {
                return Ok(ThemelioBridgeTestFFIEvents::LogNamedStringFilter(decoded));
            }
            if let Ok(decoded) = LogNamedUintFilter::decode_log(log) {
                return Ok(ThemelioBridgeTestFFIEvents::LogNamedUintFilter(decoded));
            }
            if let Ok(decoded) = LogStringFilter::decode_log(log) {
                return Ok(ThemelioBridgeTestFFIEvents::LogStringFilter(decoded));
            }
            if let Ok(decoded) = LogUintFilter::decode_log(log) {
                return Ok(ThemelioBridgeTestFFIEvents::LogUintFilter(decoded));
            }
            if let Ok(decoded) = LogsFilter::decode_log(log) {
                return Ok(ThemelioBridgeTestFFIEvents::LogsFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for ThemelioBridgeTestFFIEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ThemelioBridgeTestFFIEvents::AdminChangedFilter(element) => element.fmt(f),
                ThemelioBridgeTestFFIEvents::ApprovalForAllFilter(element) => element.fmt(f),
                ThemelioBridgeTestFFIEvents::BeaconUpgradedFilter(element) => element.fmt(f),
                ThemelioBridgeTestFFIEvents::HeaderVerifiedFilter(element) => element.fmt(f),
                ThemelioBridgeTestFFIEvents::InitializedFilter(element) => element.fmt(f),
                ThemelioBridgeTestFFIEvents::StakesVerifiedFilter(element) => element.fmt(f),
                ThemelioBridgeTestFFIEvents::TokensBurnedFilter(element) => element.fmt(f),
                ThemelioBridgeTestFFIEvents::TokensMintedFilter(element) => element.fmt(f),
                ThemelioBridgeTestFFIEvents::TransferBatchFilter(element) => element.fmt(f),
                ThemelioBridgeTestFFIEvents::TransferSingleFilter(element) => element.fmt(f),
                ThemelioBridgeTestFFIEvents::TxVerifiedFilter(element) => element.fmt(f),
                ThemelioBridgeTestFFIEvents::UriFilter(element) => element.fmt(f),
                ThemelioBridgeTestFFIEvents::UpgradedFilter(element) => element.fmt(f),
                ThemelioBridgeTestFFIEvents::LogFilter(element) => element.fmt(f),
                ThemelioBridgeTestFFIEvents::LogAddressFilter(element) => element.fmt(f),
                ThemelioBridgeTestFFIEvents::LogArray1Filter(element) => element.fmt(f),
                ThemelioBridgeTestFFIEvents::LogArray2Filter(element) => element.fmt(f),
                ThemelioBridgeTestFFIEvents::LogArray3Filter(element) => element.fmt(f),
                ThemelioBridgeTestFFIEvents::LogBytesFilter(element) => element.fmt(f),
                ThemelioBridgeTestFFIEvents::LogBytes32Filter(element) => element.fmt(f),
                ThemelioBridgeTestFFIEvents::LogIntFilter(element) => element.fmt(f),
                ThemelioBridgeTestFFIEvents::LogNamedAddressFilter(element) => element.fmt(f),
                ThemelioBridgeTestFFIEvents::LogNamedArray1Filter(element) => element.fmt(f),
                ThemelioBridgeTestFFIEvents::LogNamedArray2Filter(element) => element.fmt(f),
                ThemelioBridgeTestFFIEvents::LogNamedArray3Filter(element) => element.fmt(f),
                ThemelioBridgeTestFFIEvents::LogNamedBytesFilter(element) => element.fmt(f),
                ThemelioBridgeTestFFIEvents::LogNamedBytes32Filter(element) => element.fmt(f),
                ThemelioBridgeTestFFIEvents::LogNamedDecimalIntFilter(element) => element.fmt(f),
                ThemelioBridgeTestFFIEvents::LogNamedDecimalUintFilter(element) => element.fmt(f),
                ThemelioBridgeTestFFIEvents::LogNamedIntFilter(element) => element.fmt(f),
                ThemelioBridgeTestFFIEvents::LogNamedStringFilter(element) => element.fmt(f),
                ThemelioBridgeTestFFIEvents::LogNamedUintFilter(element) => element.fmt(f),
                ThemelioBridgeTestFFIEvents::LogStringFilter(element) => element.fmt(f),
                ThemelioBridgeTestFFIEvents::LogUintFilter(element) => element.fmt(f),
                ThemelioBridgeTestFFIEvents::LogsFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `IS_SCRIPT` function with signature `IS_SCRIPT()` and selector `[248, 204, 191, 71]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "IS_SCRIPT", abi = "IS_SCRIPT()")]
    pub struct IsScriptCall;
    #[doc = "Container type for all input parameters for the `IS_TEST` function with signature `IS_TEST()` and selector `[250, 118, 38, 212]`"]
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
    #[doc = "Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address,uint256)` and selector `[0, 253, 213, 142]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "balanceOf", abi = "balanceOf(address,uint256)")]
    pub struct BalanceOfCall {
        pub account: ethers::core::types::Address,
        pub id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `balanceOfBatch` function with signature `balanceOfBatch(address[],uint256[])` and selector `[78, 18, 115, 244]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "balanceOfBatch", abi = "balanceOfBatch(address[],uint256[])")]
    pub struct BalanceOfBatchCall {
        pub accounts: ::std::vec::Vec<ethers::core::types::Address>,
        pub ids: ::std::vec::Vec<ethers::core::types::U256>,
    }
    #[doc = "Container type for all input parameters for the `burn` function with signature `burn(address,uint256,uint256,bytes32)` and selector `[102, 216, 202, 160]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "burn", abi = "burn(address,uint256,uint256,bytes32)")]
    pub struct BurnCall {
        pub account: ethers::core::types::Address,
        pub id: ethers::core::types::U256,
        pub value: ethers::core::types::U256,
        pub themelio_recipient: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `burnBatch` function with signature `burnBatch(address,uint256[],uint256[],bytes32)` and selector `[192, 43, 12, 181]`"]
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
        name = "burnBatch",
        abi = "burnBatch(address,uint256[],uint256[],bytes32)"
    )]
    pub struct BurnBatchCall {
        pub account: ethers::core::types::Address,
        pub ids: ::std::vec::Vec<ethers::core::types::U256>,
        pub values: ::std::vec::Vec<ethers::core::types::U256>,
        pub themelio_recipient: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `computeMerkleRootHelper` function with signature `computeMerkleRootHelper(bytes32,uint256,bytes32[])` and selector `[155, 125, 66, 161]`"]
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
        name = "computeMerkleRootHelper",
        abi = "computeMerkleRootHelper(bytes32,uint256,bytes32[])"
    )]
    pub struct ComputeMerkleRootHelperCall {
        pub tx_hash: [u8; 32],
        pub index: ethers::core::types::U256,
        pub proof: ::std::vec::Vec<[u8; 32]>,
    }
    #[doc = "Container type for all input parameters for the `deal` function with signature `deal(address,address,uint256)` and selector `[107, 206, 152, 155]`"]
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
    #[doc = "Container type for all input parameters for the `deal` function with signature `deal(address,address,uint256,bool)` and selector `[151, 117, 74, 233]`"]
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
    #[doc = "Container type for all input parameters for the `deal` function with signature `deal(address,uint256)` and selector `[200, 138, 94, 109]`"]
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
    #[doc = "Container type for all input parameters for the `decodeHeaderHelper` function with signature `decodeHeaderHelper(bytes)` and selector `[134, 62, 44, 49]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "decodeHeaderHelper", abi = "decodeHeaderHelper(bytes)")]
    pub struct DecodeHeaderHelperCall {
        pub header: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `decodeStakeDocHelper` function with signature `decodeStakeDocHelper(bytes)` and selector `[34, 195, 125, 27]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "decodeStakeDocHelper", abi = "decodeStakeDocHelper(bytes)")]
    pub struct DecodeStakeDocHelperCall {
        pub encoded_stake_doc: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `decodeTransactionHelper` function with signature `decodeTransactionHelper()` and selector `[49, 116, 128, 149]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "decodeTransactionHelper", abi = "decodeTransactionHelper()")]
    pub struct DecodeTransactionHelperCall;
    #[doc = "Container type for all input parameters for the `denomToStringHelper` function with signature `denomToStringHelper(uint256)` and selector `[3, 228, 125, 68]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "denomToStringHelper", abi = "denomToStringHelper(uint256)")]
    pub struct DenomToStringHelperCall {
        pub denom: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `deployCode` function with signature `deployCode(string,bytes)` and selector `[41, 206, 157, 222]`"]
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
    #[doc = "Container type for all input parameters for the `deployCode` function with signature `deployCode(string)` and selector `[154, 131, 37, 160]`"]
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
    #[doc = "Container type for all input parameters for the `failed` function with signature `failed()` and selector `[186, 65, 79, 166]`"]
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
    #[doc = "Container type for all input parameters for the `hashDatablockHelper` function with signature `hashDatablockHelper(bytes)` and selector `[169, 132, 228, 82]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "hashDatablockHelper", abi = "hashDatablockHelper(bytes)")]
    pub struct HashDatablockHelperCall {
        pub data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `headerLimbo` function with signature `headerLimbo(bytes32)` and selector `[174, 86, 122, 75]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "headerLimbo", abi = "headerLimbo(bytes32)")]
    pub struct HeaderLimboCall(pub [u8; 32]);
    #[doc = "Container type for all input parameters for the `headers` function with signature `headers(uint256)` and selector `[86, 249, 13, 121]`"]
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
    #[doc = "Container type for all input parameters for the `hoax` function with signature `hoax(address)` and selector `[35, 50, 64, 238]`"]
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
    #[doc = "Container type for all input parameters for the `hoax` function with signature `hoax(address,address)` and selector `[41, 169, 227, 0]`"]
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
    pub struct Hoax1Call {
        pub who: ethers::core::types::Address,
        pub origin: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `hoax` function with signature `hoax(address,address,uint256)` and selector `[175, 155, 190, 95]`"]
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
    #[doc = "Container type for all input parameters for the `hoax` function with signature `hoax(address,uint256)` and selector `[233, 167, 154, 123]`"]
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
    pub struct Hoax2Call {
        pub who: ethers::core::types::Address,
        pub give: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `initialize` function with signature `initialize(uint256,bytes32,bytes32)` and selector `[9, 241, 10, 60]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "initialize", abi = "initialize(uint256,bytes32,bytes32)")]
    pub struct InitializeCall {
        pub block_height: ethers::core::types::U256,
        pub transactions_hash: [u8; 32],
        pub stakes_hash: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `isApprovedForAll` function with signature `isApprovedForAll(address,address)` and selector `[233, 133, 233, 197]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isApprovedForAll", abi = "isApprovedForAll(address,address)")]
    pub struct IsApprovedForAllCall {
        pub account: ethers::core::types::Address,
        pub operator: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `mintHelper` function with signature `mintHelper(address,uint256,uint256)` and selector `[234, 140, 237, 177]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "mintHelper", abi = "mintHelper(address,uint256,uint256)")]
    pub struct MintHelperCall {
        pub account: ethers::core::types::Address,
        pub id: ethers::core::types::U256,
        pub value: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `proxiableUUID` function with signature `proxiableUUID()` and selector `[82, 209, 144, 45]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "proxiableUUID", abi = "proxiableUUID()")]
    pub struct ProxiableUUIDCall;
    #[doc = "Container type for all input parameters for the `rewind` function with signature `rewind(uint256)` and selector `[45, 108, 23, 163]`"]
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
    #[doc = "Container type for all input parameters for the `safeBatchTransferFrom` function with signature `safeBatchTransferFrom(address,address,uint256[],uint256[],bytes)` and selector `[46, 178, 194, 214]`"]
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
        name = "safeBatchTransferFrom",
        abi = "safeBatchTransferFrom(address,address,uint256[],uint256[],bytes)"
    )]
    pub struct SafeBatchTransferFromCall {
        pub from: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
        pub ids: ::std::vec::Vec<ethers::core::types::U256>,
        pub amounts: ::std::vec::Vec<ethers::core::types::U256>,
        pub data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `safeTransferFrom` function with signature `safeTransferFrom(address,address,uint256,uint256,bytes)` and selector `[242, 66, 67, 42]`"]
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
        name = "safeTransferFrom",
        abi = "safeTransferFrom(address,address,uint256,uint256,bytes)"
    )]
    pub struct SafeTransferFromCall {
        pub from: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
        pub id: ethers::core::types::U256,
        pub amount: ethers::core::types::U256,
        pub data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `setApprovalForAll` function with signature `setApprovalForAll(address,bool)` and selector `[162, 44, 180, 101]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setApprovalForAll", abi = "setApprovalForAll(address,bool)")]
    pub struct SetApprovalForAllCall {
        pub operator: ethers::core::types::Address,
        pub approved: bool,
    }
    #[doc = "Container type for all input parameters for the `skip` function with signature `skip(uint256)` and selector `[185, 192, 113, 180]`"]
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
    #[doc = "Container type for all input parameters for the `spends` function with signature `spends(bytes32)` and selector `[253, 54, 125, 240]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "spends", abi = "spends(bytes32)")]
    pub struct SpendsCall(pub [u8; 32]);
    #[doc = "Container type for all input parameters for the `startHoax` function with signature `startHoax(address,uint256)` and selector `[16, 133, 84, 242]`"]
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
    #[doc = "Container type for all input parameters for the `startHoax` function with signature `startHoax(address,address,uint256)` and selector `[59, 248, 45, 177]`"]
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
    pub struct StartHoax3Call {
        pub who: ethers::core::types::Address,
        pub origin: ethers::core::types::Address,
        pub give: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `startHoax` function with signature `startHoax(address)` and selector `[111, 89, 112, 117]`"]
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
    #[doc = "Container type for all input parameters for the `startHoax` function with signature `startHoax(address,address)` and selector `[208, 109, 130, 41]`"]
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
    pub struct StartHoax2Call {
        pub who: ethers::core::types::Address,
        pub origin: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `[1, 255, 201, 167]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
    }
    #[doc = "Container type for all input parameters for the `testBlake3DifferentialFFI` function with signature `testBlake3DifferentialFFI(bytes)` and selector `[58, 109, 207, 29]`"]
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
        name = "testBlake3DifferentialFFI",
        abi = "testBlake3DifferentialFFI(bytes)"
    )]
    pub struct TestBlake3DifferentialFFICall {
        pub data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `testDecodeIntegerDifferentialFFI` function with signature `testDecodeIntegerDifferentialFFI(uint128)` and selector `[46, 213, 219, 79]`"]
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
        name = "testDecodeIntegerDifferentialFFI",
        abi = "testDecodeIntegerDifferentialFFI(uint128)"
    )]
    pub struct TestDecodeIntegerDifferentialFFICall {
        pub integer: u128,
    }
    #[doc = "Container type for all input parameters for the `testEd25519DifferentialFFI` function with signature `testEd25519DifferentialFFI(bytes)` and selector `[13, 189, 57, 39]`"]
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
        name = "testEd25519DifferentialFFI",
        abi = "testEd25519DifferentialFFI(bytes)"
    )]
    pub struct TestEd25519DifferentialFFICall {
        pub message: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `testKeccakBigHashFFI` function with signature `testKeccakBigHashFFI()` and selector `[127, 82, 194, 113]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "testKeccakBigHashFFI", abi = "testKeccakBigHashFFI()")]
    pub struct TestKeccakBigHashFFICall;
    #[doc = "Container type for all input parameters for the `testSliceDifferentialFFI` function with signature `testSliceDifferentialFFI(bytes,uint8,uint8)` and selector `[46, 105, 97, 63]`"]
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
        name = "testSliceDifferentialFFI",
        abi = "testSliceDifferentialFFI(bytes,uint8,uint8)"
    )]
    pub struct TestSliceDifferentialFFICall {
        pub data: ethers::core::types::Bytes,
        pub start: u8,
        pub end: u8,
    }
    #[doc = "Container type for all input parameters for the `tip` function with signature `tip(address,address,uint256)` and selector `[216, 37, 85, 241]`"]
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
    #[doc = "Container type for all input parameters for the `upgradeTo` function with signature `upgradeTo(address)` and selector `[54, 89, 207, 230]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "upgradeTo", abi = "upgradeTo(address)")]
    pub struct UpgradeToCall {
        pub new_implementation: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `upgradeToAndCall` function with signature `upgradeToAndCall(address,bytes)` and selector `[79, 30, 242, 134]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "upgradeToAndCall", abi = "upgradeToAndCall(address,bytes)")]
    pub struct UpgradeToAndCallCall {
        pub new_implementation: ethers::core::types::Address,
        pub data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `uri` function with signature `uri(uint256)` and selector `[14, 137, 52, 28]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "uri", abi = "uri(uint256)")]
    pub struct UriCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `verifyHeader` function with signature `verifyHeader(uint256,bytes,bytes,bytes32[],bool)` and selector `[112, 60, 251, 240]`"]
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
        name = "verifyHeader",
        abi = "verifyHeader(uint256,bytes,bytes,bytes32[],bool)"
    )]
    pub struct VerifyHeaderCall {
        pub verifier_height: ethers::core::types::U256,
        pub header: ethers::core::types::Bytes,
        pub stakes: ethers::core::types::Bytes,
        pub signatures: ::std::vec::Vec<[u8; 32]>,
        pub first_time: bool,
    }
    #[doc = "Container type for all input parameters for the `verifyHeaderHelper` function with signature `verifyHeaderHelper(bytes32,uint256)` and selector `[106, 47, 247, 31]`"]
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
        name = "verifyHeaderHelper",
        abi = "verifyHeaderHelper(bytes32,uint256)"
    )]
    pub struct VerifyHeaderHelperCall {
        pub verifier_stakes_hash: [u8; 32],
        pub verifier_height: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `verifyStakes` function with signature `verifyStakes(bytes)` and selector `[108, 15, 217, 76]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "verifyStakes", abi = "verifyStakes(bytes)")]
    pub struct VerifyStakesCall {
        pub stakes: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `verifyTx` function with signature `verifyTx(bytes,uint256,uint256,bytes32[])` and selector `[66, 33, 123, 99]`"]
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
    #[doc = "Container type for all input parameters for the `verifyTxHelper` function with signature `verifyTxHelper(uint256,bytes32,bytes32)` and selector `[170, 154, 70, 214]`"]
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
        name = "verifyTxHelper",
        abi = "verifyTxHelper(uint256,bytes32,bytes32)"
    )]
    pub struct VerifyTxHelperCall {
        pub block_height: ethers::core::types::U256,
        pub transactions_hash: [u8; 32],
        pub stakes_hash: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `vm` function with signature `vm()` and selector `[58, 118, 132, 99]`"]
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
    pub enum ThemelioBridgeTestFFICalls {
        IsScript(IsScriptCall),
        IsTest(IsTestCall),
        BalanceOf(BalanceOfCall),
        BalanceOfBatch(BalanceOfBatchCall),
        Burn(BurnCall),
        BurnBatch(BurnBatchCall),
        ComputeMerkleRootHelper(ComputeMerkleRootHelperCall),
        DealWithToken(DealWithTokenCall),
        DealWithTokenAndAdjust(DealWithTokenAndAdjustCall),
        Deal(DealCall),
        DecodeHeaderHelper(DecodeHeaderHelperCall),
        DecodeStakeDocHelper(DecodeStakeDocHelperCall),
        DecodeTransactionHelper(DecodeTransactionHelperCall),
        DenomToStringHelper(DenomToStringHelperCall),
        DeployCodeWithArgs(DeployCodeWithArgsCall),
        DeployCode(DeployCodeCall),
        Failed(FailedCall),
        HashDatablockHelper(HashDatablockHelperCall),
        HeaderLimbo(HeaderLimboCall),
        Headers(HeadersCall),
        Hoax0(Hoax0Call),
        Hoax1(Hoax1Call),
        Hoax3(Hoax3Call),
        Hoax2(Hoax2Call),
        Initialize(InitializeCall),
        IsApprovedForAll(IsApprovedForAllCall),
        MintHelper(MintHelperCall),
        ProxiableUUID(ProxiableUUIDCall),
        Rewind(RewindCall),
        SafeBatchTransferFrom(SafeBatchTransferFromCall),
        SafeTransferFrom(SafeTransferFromCall),
        SetApprovalForAll(SetApprovalForAllCall),
        Skip(SkipCall),
        Spends(SpendsCall),
        StartHoax1(StartHoax1Call),
        StartHoax3(StartHoax3Call),
        StartHoax0(StartHoax0Call),
        StartHoax2(StartHoax2Call),
        SupportsInterface(SupportsInterfaceCall),
        TestBlake3DifferentialFFI(TestBlake3DifferentialFFICall),
        TestDecodeIntegerDifferentialFFI(TestDecodeIntegerDifferentialFFICall),
        TestEd25519DifferentialFFI(TestEd25519DifferentialFFICall),
        TestKeccakBigHashFFI(TestKeccakBigHashFFICall),
        TestSliceDifferentialFFI(TestSliceDifferentialFFICall),
        Tip(TipCall),
        UpgradeTo(UpgradeToCall),
        UpgradeToAndCall(UpgradeToAndCallCall),
        Uri(UriCall),
        VerifyHeader(VerifyHeaderCall),
        VerifyHeaderHelper(VerifyHeaderHelperCall),
        VerifyStakes(VerifyStakesCall),
        VerifyTx(VerifyTxCall),
        VerifyTxHelper(VerifyTxHelperCall),
        Vm(VmCall),
    }
    impl ethers::core::abi::AbiDecode for ThemelioBridgeTestFFICalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <IsScriptCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestFFICalls::IsScript(decoded));
            }
            if let Ok(decoded) = <IsTestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestFFICalls::IsTest(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestFFICalls::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfBatchCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestFFICalls::BalanceOfBatch(decoded));
            }
            if let Ok(decoded) = <BurnCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(ThemelioBridgeTestFFICalls::Burn(decoded));
            }
            if let Ok(decoded) =
                <BurnBatchCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestFFICalls::BurnBatch(decoded));
            }
            if let Ok(decoded) =
                <ComputeMerkleRootHelperCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestFFICalls::ComputeMerkleRootHelper(decoded));
            }
            if let Ok(decoded) =
                <DealWithTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestFFICalls::DealWithToken(decoded));
            }
            if let Ok(decoded) =
                <DealWithTokenAndAdjustCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestFFICalls::DealWithTokenAndAdjust(decoded));
            }
            if let Ok(decoded) = <DealCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(ThemelioBridgeTestFFICalls::Deal(decoded));
            }
            if let Ok(decoded) =
                <DecodeHeaderHelperCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestFFICalls::DecodeHeaderHelper(decoded));
            }
            if let Ok(decoded) =
                <DecodeStakeDocHelperCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestFFICalls::DecodeStakeDocHelper(decoded));
            }
            if let Ok(decoded) =
                <DecodeTransactionHelperCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestFFICalls::DecodeTransactionHelper(decoded));
            }
            if let Ok(decoded) =
                <DenomToStringHelperCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestFFICalls::DenomToStringHelper(decoded));
            }
            if let Ok(decoded) =
                <DeployCodeWithArgsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestFFICalls::DeployCodeWithArgs(decoded));
            }
            if let Ok(decoded) =
                <DeployCodeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestFFICalls::DeployCode(decoded));
            }
            if let Ok(decoded) = <FailedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestFFICalls::Failed(decoded));
            }
            if let Ok(decoded) =
                <HashDatablockHelperCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestFFICalls::HashDatablockHelper(decoded));
            }
            if let Ok(decoded) =
                <HeaderLimboCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestFFICalls::HeaderLimbo(decoded));
            }
            if let Ok(decoded) =
                <HeadersCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestFFICalls::Headers(decoded));
            }
            if let Ok(decoded) = <Hoax0Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestFFICalls::Hoax0(decoded));
            }
            if let Ok(decoded) = <Hoax1Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestFFICalls::Hoax1(decoded));
            }
            if let Ok(decoded) = <Hoax3Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestFFICalls::Hoax3(decoded));
            }
            if let Ok(decoded) = <Hoax2Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestFFICalls::Hoax2(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestFFICalls::Initialize(decoded));
            }
            if let Ok(decoded) =
                <IsApprovedForAllCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestFFICalls::IsApprovedForAll(decoded));
            }
            if let Ok(decoded) =
                <MintHelperCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestFFICalls::MintHelper(decoded));
            }
            if let Ok(decoded) =
                <ProxiableUUIDCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestFFICalls::ProxiableUUID(decoded));
            }
            if let Ok(decoded) = <RewindCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestFFICalls::Rewind(decoded));
            }
            if let Ok(decoded) =
                <SafeBatchTransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestFFICalls::SafeBatchTransferFrom(decoded));
            }
            if let Ok(decoded) =
                <SafeTransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestFFICalls::SafeTransferFrom(decoded));
            }
            if let Ok(decoded) =
                <SetApprovalForAllCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestFFICalls::SetApprovalForAll(decoded));
            }
            if let Ok(decoded) = <SkipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(ThemelioBridgeTestFFICalls::Skip(decoded));
            }
            if let Ok(decoded) = <SpendsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestFFICalls::Spends(decoded));
            }
            if let Ok(decoded) =
                <StartHoax1Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestFFICalls::StartHoax1(decoded));
            }
            if let Ok(decoded) =
                <StartHoax3Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestFFICalls::StartHoax3(decoded));
            }
            if let Ok(decoded) =
                <StartHoax0Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestFFICalls::StartHoax0(decoded));
            }
            if let Ok(decoded) =
                <StartHoax2Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestFFICalls::StartHoax2(decoded));
            }
            if let Ok(decoded) =
                <SupportsInterfaceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestFFICalls::SupportsInterface(decoded));
            }
            if let Ok(decoded) =
                <TestBlake3DifferentialFFICall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ThemelioBridgeTestFFICalls::TestBlake3DifferentialFFI(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <TestDecodeIntegerDifferentialFFICall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ThemelioBridgeTestFFICalls::TestDecodeIntegerDifferentialFFI(decoded));
            }
            if let Ok(decoded) =
                <TestEd25519DifferentialFFICall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ThemelioBridgeTestFFICalls::TestEd25519DifferentialFFI(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <TestKeccakBigHashFFICall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestFFICalls::TestKeccakBigHashFFI(decoded));
            }
            if let Ok(decoded) =
                <TestSliceDifferentialFFICall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ThemelioBridgeTestFFICalls::TestSliceDifferentialFFI(
                    decoded,
                ));
            }
            if let Ok(decoded) = <TipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(ThemelioBridgeTestFFICalls::Tip(decoded));
            }
            if let Ok(decoded) =
                <UpgradeToCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestFFICalls::UpgradeTo(decoded));
            }
            if let Ok(decoded) =
                <UpgradeToAndCallCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestFFICalls::UpgradeToAndCall(decoded));
            }
            if let Ok(decoded) = <UriCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(ThemelioBridgeTestFFICalls::Uri(decoded));
            }
            if let Ok(decoded) =
                <VerifyHeaderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestFFICalls::VerifyHeader(decoded));
            }
            if let Ok(decoded) =
                <VerifyHeaderHelperCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestFFICalls::VerifyHeaderHelper(decoded));
            }
            if let Ok(decoded) =
                <VerifyStakesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestFFICalls::VerifyStakes(decoded));
            }
            if let Ok(decoded) =
                <VerifyTxCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestFFICalls::VerifyTx(decoded));
            }
            if let Ok(decoded) =
                <VerifyTxHelperCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestFFICalls::VerifyTxHelper(decoded));
            }
            if let Ok(decoded) = <VmCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(ThemelioBridgeTestFFICalls::Vm(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ThemelioBridgeTestFFICalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ThemelioBridgeTestFFICalls::IsScript(element) => element.encode(),
                ThemelioBridgeTestFFICalls::IsTest(element) => element.encode(),
                ThemelioBridgeTestFFICalls::BalanceOf(element) => element.encode(),
                ThemelioBridgeTestFFICalls::BalanceOfBatch(element) => element.encode(),
                ThemelioBridgeTestFFICalls::Burn(element) => element.encode(),
                ThemelioBridgeTestFFICalls::BurnBatch(element) => element.encode(),
                ThemelioBridgeTestFFICalls::ComputeMerkleRootHelper(element) => element.encode(),
                ThemelioBridgeTestFFICalls::DealWithToken(element) => element.encode(),
                ThemelioBridgeTestFFICalls::DealWithTokenAndAdjust(element) => element.encode(),
                ThemelioBridgeTestFFICalls::Deal(element) => element.encode(),
                ThemelioBridgeTestFFICalls::DecodeHeaderHelper(element) => element.encode(),
                ThemelioBridgeTestFFICalls::DecodeStakeDocHelper(element) => element.encode(),
                ThemelioBridgeTestFFICalls::DecodeTransactionHelper(element) => element.encode(),
                ThemelioBridgeTestFFICalls::DenomToStringHelper(element) => element.encode(),
                ThemelioBridgeTestFFICalls::DeployCodeWithArgs(element) => element.encode(),
                ThemelioBridgeTestFFICalls::DeployCode(element) => element.encode(),
                ThemelioBridgeTestFFICalls::Failed(element) => element.encode(),
                ThemelioBridgeTestFFICalls::HashDatablockHelper(element) => element.encode(),
                ThemelioBridgeTestFFICalls::HeaderLimbo(element) => element.encode(),
                ThemelioBridgeTestFFICalls::Headers(element) => element.encode(),
                ThemelioBridgeTestFFICalls::Hoax0(element) => element.encode(),
                ThemelioBridgeTestFFICalls::Hoax1(element) => element.encode(),
                ThemelioBridgeTestFFICalls::Hoax3(element) => element.encode(),
                ThemelioBridgeTestFFICalls::Hoax2(element) => element.encode(),
                ThemelioBridgeTestFFICalls::Initialize(element) => element.encode(),
                ThemelioBridgeTestFFICalls::IsApprovedForAll(element) => element.encode(),
                ThemelioBridgeTestFFICalls::MintHelper(element) => element.encode(),
                ThemelioBridgeTestFFICalls::ProxiableUUID(element) => element.encode(),
                ThemelioBridgeTestFFICalls::Rewind(element) => element.encode(),
                ThemelioBridgeTestFFICalls::SafeBatchTransferFrom(element) => element.encode(),
                ThemelioBridgeTestFFICalls::SafeTransferFrom(element) => element.encode(),
                ThemelioBridgeTestFFICalls::SetApprovalForAll(element) => element.encode(),
                ThemelioBridgeTestFFICalls::Skip(element) => element.encode(),
                ThemelioBridgeTestFFICalls::Spends(element) => element.encode(),
                ThemelioBridgeTestFFICalls::StartHoax1(element) => element.encode(),
                ThemelioBridgeTestFFICalls::StartHoax3(element) => element.encode(),
                ThemelioBridgeTestFFICalls::StartHoax0(element) => element.encode(),
                ThemelioBridgeTestFFICalls::StartHoax2(element) => element.encode(),
                ThemelioBridgeTestFFICalls::SupportsInterface(element) => element.encode(),
                ThemelioBridgeTestFFICalls::TestBlake3DifferentialFFI(element) => element.encode(),
                ThemelioBridgeTestFFICalls::TestDecodeIntegerDifferentialFFI(element) => {
                    element.encode()
                }
                ThemelioBridgeTestFFICalls::TestEd25519DifferentialFFI(element) => element.encode(),
                ThemelioBridgeTestFFICalls::TestKeccakBigHashFFI(element) => element.encode(),
                ThemelioBridgeTestFFICalls::TestSliceDifferentialFFI(element) => element.encode(),
                ThemelioBridgeTestFFICalls::Tip(element) => element.encode(),
                ThemelioBridgeTestFFICalls::UpgradeTo(element) => element.encode(),
                ThemelioBridgeTestFFICalls::UpgradeToAndCall(element) => element.encode(),
                ThemelioBridgeTestFFICalls::Uri(element) => element.encode(),
                ThemelioBridgeTestFFICalls::VerifyHeader(element) => element.encode(),
                ThemelioBridgeTestFFICalls::VerifyHeaderHelper(element) => element.encode(),
                ThemelioBridgeTestFFICalls::VerifyStakes(element) => element.encode(),
                ThemelioBridgeTestFFICalls::VerifyTx(element) => element.encode(),
                ThemelioBridgeTestFFICalls::VerifyTxHelper(element) => element.encode(),
                ThemelioBridgeTestFFICalls::Vm(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ThemelioBridgeTestFFICalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ThemelioBridgeTestFFICalls::IsScript(element) => element.fmt(f),
                ThemelioBridgeTestFFICalls::IsTest(element) => element.fmt(f),
                ThemelioBridgeTestFFICalls::BalanceOf(element) => element.fmt(f),
                ThemelioBridgeTestFFICalls::BalanceOfBatch(element) => element.fmt(f),
                ThemelioBridgeTestFFICalls::Burn(element) => element.fmt(f),
                ThemelioBridgeTestFFICalls::BurnBatch(element) => element.fmt(f),
                ThemelioBridgeTestFFICalls::ComputeMerkleRootHelper(element) => element.fmt(f),
                ThemelioBridgeTestFFICalls::DealWithToken(element) => element.fmt(f),
                ThemelioBridgeTestFFICalls::DealWithTokenAndAdjust(element) => element.fmt(f),
                ThemelioBridgeTestFFICalls::Deal(element) => element.fmt(f),
                ThemelioBridgeTestFFICalls::DecodeHeaderHelper(element) => element.fmt(f),
                ThemelioBridgeTestFFICalls::DecodeStakeDocHelper(element) => element.fmt(f),
                ThemelioBridgeTestFFICalls::DecodeTransactionHelper(element) => element.fmt(f),
                ThemelioBridgeTestFFICalls::DenomToStringHelper(element) => element.fmt(f),
                ThemelioBridgeTestFFICalls::DeployCodeWithArgs(element) => element.fmt(f),
                ThemelioBridgeTestFFICalls::DeployCode(element) => element.fmt(f),
                ThemelioBridgeTestFFICalls::Failed(element) => element.fmt(f),
                ThemelioBridgeTestFFICalls::HashDatablockHelper(element) => element.fmt(f),
                ThemelioBridgeTestFFICalls::HeaderLimbo(element) => element.fmt(f),
                ThemelioBridgeTestFFICalls::Headers(element) => element.fmt(f),
                ThemelioBridgeTestFFICalls::Hoax0(element) => element.fmt(f),
                ThemelioBridgeTestFFICalls::Hoax1(element) => element.fmt(f),
                ThemelioBridgeTestFFICalls::Hoax3(element) => element.fmt(f),
                ThemelioBridgeTestFFICalls::Hoax2(element) => element.fmt(f),
                ThemelioBridgeTestFFICalls::Initialize(element) => element.fmt(f),
                ThemelioBridgeTestFFICalls::IsApprovedForAll(element) => element.fmt(f),
                ThemelioBridgeTestFFICalls::MintHelper(element) => element.fmt(f),
                ThemelioBridgeTestFFICalls::ProxiableUUID(element) => element.fmt(f),
                ThemelioBridgeTestFFICalls::Rewind(element) => element.fmt(f),
                ThemelioBridgeTestFFICalls::SafeBatchTransferFrom(element) => element.fmt(f),
                ThemelioBridgeTestFFICalls::SafeTransferFrom(element) => element.fmt(f),
                ThemelioBridgeTestFFICalls::SetApprovalForAll(element) => element.fmt(f),
                ThemelioBridgeTestFFICalls::Skip(element) => element.fmt(f),
                ThemelioBridgeTestFFICalls::Spends(element) => element.fmt(f),
                ThemelioBridgeTestFFICalls::StartHoax1(element) => element.fmt(f),
                ThemelioBridgeTestFFICalls::StartHoax3(element) => element.fmt(f),
                ThemelioBridgeTestFFICalls::StartHoax0(element) => element.fmt(f),
                ThemelioBridgeTestFFICalls::StartHoax2(element) => element.fmt(f),
                ThemelioBridgeTestFFICalls::SupportsInterface(element) => element.fmt(f),
                ThemelioBridgeTestFFICalls::TestBlake3DifferentialFFI(element) => element.fmt(f),
                ThemelioBridgeTestFFICalls::TestDecodeIntegerDifferentialFFI(element) => {
                    element.fmt(f)
                }
                ThemelioBridgeTestFFICalls::TestEd25519DifferentialFFI(element) => element.fmt(f),
                ThemelioBridgeTestFFICalls::TestKeccakBigHashFFI(element) => element.fmt(f),
                ThemelioBridgeTestFFICalls::TestSliceDifferentialFFI(element) => element.fmt(f),
                ThemelioBridgeTestFFICalls::Tip(element) => element.fmt(f),
                ThemelioBridgeTestFFICalls::UpgradeTo(element) => element.fmt(f),
                ThemelioBridgeTestFFICalls::UpgradeToAndCall(element) => element.fmt(f),
                ThemelioBridgeTestFFICalls::Uri(element) => element.fmt(f),
                ThemelioBridgeTestFFICalls::VerifyHeader(element) => element.fmt(f),
                ThemelioBridgeTestFFICalls::VerifyHeaderHelper(element) => element.fmt(f),
                ThemelioBridgeTestFFICalls::VerifyStakes(element) => element.fmt(f),
                ThemelioBridgeTestFFICalls::VerifyTx(element) => element.fmt(f),
                ThemelioBridgeTestFFICalls::VerifyTxHelper(element) => element.fmt(f),
                ThemelioBridgeTestFFICalls::Vm(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<IsScriptCall> for ThemelioBridgeTestFFICalls {
        fn from(var: IsScriptCall) -> Self {
            ThemelioBridgeTestFFICalls::IsScript(var)
        }
    }
    impl ::std::convert::From<IsTestCall> for ThemelioBridgeTestFFICalls {
        fn from(var: IsTestCall) -> Self {
            ThemelioBridgeTestFFICalls::IsTest(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for ThemelioBridgeTestFFICalls {
        fn from(var: BalanceOfCall) -> Self {
            ThemelioBridgeTestFFICalls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<BalanceOfBatchCall> for ThemelioBridgeTestFFICalls {
        fn from(var: BalanceOfBatchCall) -> Self {
            ThemelioBridgeTestFFICalls::BalanceOfBatch(var)
        }
    }
    impl ::std::convert::From<BurnCall> for ThemelioBridgeTestFFICalls {
        fn from(var: BurnCall) -> Self {
            ThemelioBridgeTestFFICalls::Burn(var)
        }
    }
    impl ::std::convert::From<BurnBatchCall> for ThemelioBridgeTestFFICalls {
        fn from(var: BurnBatchCall) -> Self {
            ThemelioBridgeTestFFICalls::BurnBatch(var)
        }
    }
    impl ::std::convert::From<ComputeMerkleRootHelperCall> for ThemelioBridgeTestFFICalls {
        fn from(var: ComputeMerkleRootHelperCall) -> Self {
            ThemelioBridgeTestFFICalls::ComputeMerkleRootHelper(var)
        }
    }
    impl ::std::convert::From<DealWithTokenCall> for ThemelioBridgeTestFFICalls {
        fn from(var: DealWithTokenCall) -> Self {
            ThemelioBridgeTestFFICalls::DealWithToken(var)
        }
    }
    impl ::std::convert::From<DealWithTokenAndAdjustCall> for ThemelioBridgeTestFFICalls {
        fn from(var: DealWithTokenAndAdjustCall) -> Self {
            ThemelioBridgeTestFFICalls::DealWithTokenAndAdjust(var)
        }
    }
    impl ::std::convert::From<DealCall> for ThemelioBridgeTestFFICalls {
        fn from(var: DealCall) -> Self {
            ThemelioBridgeTestFFICalls::Deal(var)
        }
    }
    impl ::std::convert::From<DecodeHeaderHelperCall> for ThemelioBridgeTestFFICalls {
        fn from(var: DecodeHeaderHelperCall) -> Self {
            ThemelioBridgeTestFFICalls::DecodeHeaderHelper(var)
        }
    }
    impl ::std::convert::From<DecodeStakeDocHelperCall> for ThemelioBridgeTestFFICalls {
        fn from(var: DecodeStakeDocHelperCall) -> Self {
            ThemelioBridgeTestFFICalls::DecodeStakeDocHelper(var)
        }
    }
    impl ::std::convert::From<DecodeTransactionHelperCall> for ThemelioBridgeTestFFICalls {
        fn from(var: DecodeTransactionHelperCall) -> Self {
            ThemelioBridgeTestFFICalls::DecodeTransactionHelper(var)
        }
    }
    impl ::std::convert::From<DenomToStringHelperCall> for ThemelioBridgeTestFFICalls {
        fn from(var: DenomToStringHelperCall) -> Self {
            ThemelioBridgeTestFFICalls::DenomToStringHelper(var)
        }
    }
    impl ::std::convert::From<DeployCodeWithArgsCall> for ThemelioBridgeTestFFICalls {
        fn from(var: DeployCodeWithArgsCall) -> Self {
            ThemelioBridgeTestFFICalls::DeployCodeWithArgs(var)
        }
    }
    impl ::std::convert::From<DeployCodeCall> for ThemelioBridgeTestFFICalls {
        fn from(var: DeployCodeCall) -> Self {
            ThemelioBridgeTestFFICalls::DeployCode(var)
        }
    }
    impl ::std::convert::From<FailedCall> for ThemelioBridgeTestFFICalls {
        fn from(var: FailedCall) -> Self {
            ThemelioBridgeTestFFICalls::Failed(var)
        }
    }
    impl ::std::convert::From<HashDatablockHelperCall> for ThemelioBridgeTestFFICalls {
        fn from(var: HashDatablockHelperCall) -> Self {
            ThemelioBridgeTestFFICalls::HashDatablockHelper(var)
        }
    }
    impl ::std::convert::From<HeaderLimboCall> for ThemelioBridgeTestFFICalls {
        fn from(var: HeaderLimboCall) -> Self {
            ThemelioBridgeTestFFICalls::HeaderLimbo(var)
        }
    }
    impl ::std::convert::From<HeadersCall> for ThemelioBridgeTestFFICalls {
        fn from(var: HeadersCall) -> Self {
            ThemelioBridgeTestFFICalls::Headers(var)
        }
    }
    impl ::std::convert::From<Hoax0Call> for ThemelioBridgeTestFFICalls {
        fn from(var: Hoax0Call) -> Self {
            ThemelioBridgeTestFFICalls::Hoax0(var)
        }
    }
    impl ::std::convert::From<Hoax1Call> for ThemelioBridgeTestFFICalls {
        fn from(var: Hoax1Call) -> Self {
            ThemelioBridgeTestFFICalls::Hoax1(var)
        }
    }
    impl ::std::convert::From<Hoax3Call> for ThemelioBridgeTestFFICalls {
        fn from(var: Hoax3Call) -> Self {
            ThemelioBridgeTestFFICalls::Hoax3(var)
        }
    }
    impl ::std::convert::From<Hoax2Call> for ThemelioBridgeTestFFICalls {
        fn from(var: Hoax2Call) -> Self {
            ThemelioBridgeTestFFICalls::Hoax2(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for ThemelioBridgeTestFFICalls {
        fn from(var: InitializeCall) -> Self {
            ThemelioBridgeTestFFICalls::Initialize(var)
        }
    }
    impl ::std::convert::From<IsApprovedForAllCall> for ThemelioBridgeTestFFICalls {
        fn from(var: IsApprovedForAllCall) -> Self {
            ThemelioBridgeTestFFICalls::IsApprovedForAll(var)
        }
    }
    impl ::std::convert::From<MintHelperCall> for ThemelioBridgeTestFFICalls {
        fn from(var: MintHelperCall) -> Self {
            ThemelioBridgeTestFFICalls::MintHelper(var)
        }
    }
    impl ::std::convert::From<ProxiableUUIDCall> for ThemelioBridgeTestFFICalls {
        fn from(var: ProxiableUUIDCall) -> Self {
            ThemelioBridgeTestFFICalls::ProxiableUUID(var)
        }
    }
    impl ::std::convert::From<RewindCall> for ThemelioBridgeTestFFICalls {
        fn from(var: RewindCall) -> Self {
            ThemelioBridgeTestFFICalls::Rewind(var)
        }
    }
    impl ::std::convert::From<SafeBatchTransferFromCall> for ThemelioBridgeTestFFICalls {
        fn from(var: SafeBatchTransferFromCall) -> Self {
            ThemelioBridgeTestFFICalls::SafeBatchTransferFrom(var)
        }
    }
    impl ::std::convert::From<SafeTransferFromCall> for ThemelioBridgeTestFFICalls {
        fn from(var: SafeTransferFromCall) -> Self {
            ThemelioBridgeTestFFICalls::SafeTransferFrom(var)
        }
    }
    impl ::std::convert::From<SetApprovalForAllCall> for ThemelioBridgeTestFFICalls {
        fn from(var: SetApprovalForAllCall) -> Self {
            ThemelioBridgeTestFFICalls::SetApprovalForAll(var)
        }
    }
    impl ::std::convert::From<SkipCall> for ThemelioBridgeTestFFICalls {
        fn from(var: SkipCall) -> Self {
            ThemelioBridgeTestFFICalls::Skip(var)
        }
    }
    impl ::std::convert::From<SpendsCall> for ThemelioBridgeTestFFICalls {
        fn from(var: SpendsCall) -> Self {
            ThemelioBridgeTestFFICalls::Spends(var)
        }
    }
    impl ::std::convert::From<StartHoax1Call> for ThemelioBridgeTestFFICalls {
        fn from(var: StartHoax1Call) -> Self {
            ThemelioBridgeTestFFICalls::StartHoax1(var)
        }
    }
    impl ::std::convert::From<StartHoax3Call> for ThemelioBridgeTestFFICalls {
        fn from(var: StartHoax3Call) -> Self {
            ThemelioBridgeTestFFICalls::StartHoax3(var)
        }
    }
    impl ::std::convert::From<StartHoax0Call> for ThemelioBridgeTestFFICalls {
        fn from(var: StartHoax0Call) -> Self {
            ThemelioBridgeTestFFICalls::StartHoax0(var)
        }
    }
    impl ::std::convert::From<StartHoax2Call> for ThemelioBridgeTestFFICalls {
        fn from(var: StartHoax2Call) -> Self {
            ThemelioBridgeTestFFICalls::StartHoax2(var)
        }
    }
    impl ::std::convert::From<SupportsInterfaceCall> for ThemelioBridgeTestFFICalls {
        fn from(var: SupportsInterfaceCall) -> Self {
            ThemelioBridgeTestFFICalls::SupportsInterface(var)
        }
    }
    impl ::std::convert::From<TestBlake3DifferentialFFICall> for ThemelioBridgeTestFFICalls {
        fn from(var: TestBlake3DifferentialFFICall) -> Self {
            ThemelioBridgeTestFFICalls::TestBlake3DifferentialFFI(var)
        }
    }
    impl ::std::convert::From<TestDecodeIntegerDifferentialFFICall> for ThemelioBridgeTestFFICalls {
        fn from(var: TestDecodeIntegerDifferentialFFICall) -> Self {
            ThemelioBridgeTestFFICalls::TestDecodeIntegerDifferentialFFI(var)
        }
    }
    impl ::std::convert::From<TestEd25519DifferentialFFICall> for ThemelioBridgeTestFFICalls {
        fn from(var: TestEd25519DifferentialFFICall) -> Self {
            ThemelioBridgeTestFFICalls::TestEd25519DifferentialFFI(var)
        }
    }
    impl ::std::convert::From<TestKeccakBigHashFFICall> for ThemelioBridgeTestFFICalls {
        fn from(var: TestKeccakBigHashFFICall) -> Self {
            ThemelioBridgeTestFFICalls::TestKeccakBigHashFFI(var)
        }
    }
    impl ::std::convert::From<TestSliceDifferentialFFICall> for ThemelioBridgeTestFFICalls {
        fn from(var: TestSliceDifferentialFFICall) -> Self {
            ThemelioBridgeTestFFICalls::TestSliceDifferentialFFI(var)
        }
    }
    impl ::std::convert::From<TipCall> for ThemelioBridgeTestFFICalls {
        fn from(var: TipCall) -> Self {
            ThemelioBridgeTestFFICalls::Tip(var)
        }
    }
    impl ::std::convert::From<UpgradeToCall> for ThemelioBridgeTestFFICalls {
        fn from(var: UpgradeToCall) -> Self {
            ThemelioBridgeTestFFICalls::UpgradeTo(var)
        }
    }
    impl ::std::convert::From<UpgradeToAndCallCall> for ThemelioBridgeTestFFICalls {
        fn from(var: UpgradeToAndCallCall) -> Self {
            ThemelioBridgeTestFFICalls::UpgradeToAndCall(var)
        }
    }
    impl ::std::convert::From<UriCall> for ThemelioBridgeTestFFICalls {
        fn from(var: UriCall) -> Self {
            ThemelioBridgeTestFFICalls::Uri(var)
        }
    }
    impl ::std::convert::From<VerifyHeaderCall> for ThemelioBridgeTestFFICalls {
        fn from(var: VerifyHeaderCall) -> Self {
            ThemelioBridgeTestFFICalls::VerifyHeader(var)
        }
    }
    impl ::std::convert::From<VerifyHeaderHelperCall> for ThemelioBridgeTestFFICalls {
        fn from(var: VerifyHeaderHelperCall) -> Self {
            ThemelioBridgeTestFFICalls::VerifyHeaderHelper(var)
        }
    }
    impl ::std::convert::From<VerifyStakesCall> for ThemelioBridgeTestFFICalls {
        fn from(var: VerifyStakesCall) -> Self {
            ThemelioBridgeTestFFICalls::VerifyStakes(var)
        }
    }
    impl ::std::convert::From<VerifyTxCall> for ThemelioBridgeTestFFICalls {
        fn from(var: VerifyTxCall) -> Self {
            ThemelioBridgeTestFFICalls::VerifyTx(var)
        }
    }
    impl ::std::convert::From<VerifyTxHelperCall> for ThemelioBridgeTestFFICalls {
        fn from(var: VerifyTxHelperCall) -> Self {
            ThemelioBridgeTestFFICalls::VerifyTxHelper(var)
        }
    }
    impl ::std::convert::From<VmCall> for ThemelioBridgeTestFFICalls {
        fn from(var: VmCall) -> Self {
            ThemelioBridgeTestFFICalls::Vm(var)
        }
    }
    #[doc = "Container type for all return fields from the `IS_SCRIPT` function with signature `IS_SCRIPT()` and selector `[248, 204, 191, 71]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct IsScriptReturn(pub bool);
    #[doc = "Container type for all return fields from the `IS_TEST` function with signature `IS_TEST()` and selector `[250, 118, 38, 212]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct IsTestReturn(pub bool);
    #[doc = "Container type for all return fields from the `balanceOf` function with signature `balanceOf(address,uint256)` and selector `[0, 253, 213, 142]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct BalanceOfReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `balanceOfBatch` function with signature `balanceOfBatch(address[],uint256[])` and selector `[78, 18, 115, 244]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct BalanceOfBatchReturn(pub ::std::vec::Vec<ethers::core::types::U256>);
    #[doc = "Container type for all return fields from the `computeMerkleRootHelper` function with signature `computeMerkleRootHelper(bytes32,uint256,bytes32[])` and selector `[155, 125, 66, 161]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ComputeMerkleRootHelperReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `decodeHeaderHelper` function with signature `decodeHeaderHelper(bytes)` and selector `[134, 62, 44, 49]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct DecodeHeaderHelperReturn(pub ethers::core::types::U256, pub [u8; 32], pub [u8; 32]);
    #[doc = "Container type for all return fields from the `decodeStakeDocHelper` function with signature `decodeStakeDocHelper(bytes)` and selector `[34, 195, 125, 27]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct DecodeStakeDocHelperReturn(
        pub [u8; 32],
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
    );
    #[doc = "Container type for all return fields from the `denomToStringHelper` function with signature `denomToStringHelper(uint256)` and selector `[3, 228, 125, 68]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct DenomToStringHelperReturn(pub String);
    #[doc = "Container type for all return fields from the `deployCode` function with signature `deployCode(string,bytes)` and selector `[41, 206, 157, 222]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct DeployCodeWithArgsReturn {
        pub addr: ethers::core::types::Address,
    }
    #[doc = "Container type for all return fields from the `deployCode` function with signature `deployCode(string)` and selector `[154, 131, 37, 160]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct DeployCodeReturn {
        pub addr: ethers::core::types::Address,
    }
    #[doc = "Container type for all return fields from the `failed` function with signature `failed()` and selector `[186, 65, 79, 166]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct FailedReturn(pub bool);
    #[doc = "Container type for all return fields from the `hashDatablockHelper` function with signature `hashDatablockHelper(bytes)` and selector `[169, 132, 228, 82]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct HashDatablockHelperReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `headerLimbo` function with signature `headerLimbo(bytes32)` and selector `[174, 86, 122, 75]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct HeaderLimboReturn {
        pub votes: u128,
        pub bytes_verified: u64,
        pub stake_doc_index: u64,
    }
    #[doc = "Container type for all return fields from the `headers` function with signature `headers(uint256)` and selector `[86, 249, 13, 121]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct HeadersReturn {
        pub transactions_hash: [u8; 32],
        pub stakes_hash: [u8; 32],
    }
    #[doc = "Container type for all return fields from the `isApprovedForAll` function with signature `isApprovedForAll(address,address)` and selector `[233, 133, 233, 197]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct IsApprovedForAllReturn(pub bool);
    #[doc = "Container type for all return fields from the `proxiableUUID` function with signature `proxiableUUID()` and selector `[82, 209, 144, 45]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ProxiableUUIDReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `spends` function with signature `spends(bytes32)` and selector `[253, 54, 125, 240]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct SpendsReturn(pub bool);
    #[doc = "Container type for all return fields from the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `[1, 255, 201, 167]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct SupportsInterfaceReturn(pub bool);
    #[doc = "Container type for all return fields from the `uri` function with signature `uri(uint256)` and selector `[14, 137, 52, 28]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct UriReturn(pub String);
    #[doc = "Container type for all return fields from the `verifyHeader` function with signature `verifyHeader(uint256,bytes,bytes,bytes32[],bool)` and selector `[112, 60, 251, 240]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct VerifyHeaderReturn(pub bool);
    #[doc = "Container type for all return fields from the `verifyStakes` function with signature `verifyStakes(bytes)` and selector `[108, 15, 217, 76]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct VerifyStakesReturn(pub bool);
    #[doc = "Container type for all return fields from the `verifyTx` function with signature `verifyTx(bytes,uint256,uint256,bytes32[])` and selector `[66, 33, 123, 99]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct VerifyTxReturn(pub bool);
    #[doc = "Container type for all return fields from the `vm` function with signature `vm()` and selector `[58, 118, 132, 99]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct VmReturn(pub ethers::core::types::Address);
}
