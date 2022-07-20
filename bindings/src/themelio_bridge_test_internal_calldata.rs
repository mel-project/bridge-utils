pub use themeliobridgetestinternalcalldata_mod::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod themeliobridgetestinternalcalldata_mod {
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
    #[doc = "ThemelioBridgeTestInternalCalldata was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static THEMELIOBRIDGETESTINTERNALCALLDATA_ABI: ethers::contract::Lazy<
        ethers::core::abi::Abi,
    > = ethers::contract::Lazy::new(|| {
        serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_address\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256[]\",\"name\":\"val\",\"type\":\"uint256[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_array\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"int256[]\",\"name\":\"val\",\"type\":\"int256[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_array\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"val\",\"type\":\"address[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_array\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_bytes\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_bytes32\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"\",\"type\":\"int256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_int\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"val\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_address\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256[]\",\"name\":\"val\",\"type\":\"uint256[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_array\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256[]\",\"name\":\"val\",\"type\":\"int256[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_array\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"address[]\",\"name\":\"val\",\"type\":\"address[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_array\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"val\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_bytes\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes32\",\"name\":\"val\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_bytes32\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256\",\"name\":\"val\",\"type\":\"int256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"decimals\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_decimal_int\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"val\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"decimals\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_decimal_uint\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256\",\"name\":\"val\",\"type\":\"int256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_int\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"val\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_string\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"val\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_uint\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_string\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_uint\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"logs\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"IS_SCRIPT\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"IS_TEST\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"give\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deal\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"give\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"adjust\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deal\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"give\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deal\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"what\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"args\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deployCode\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"addr\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"what\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deployCode\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"addr\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"failed\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"who\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"hoax\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"who\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"origin\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"hoax\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"who\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"origin\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"give\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"hoax\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"who\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"give\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"hoax\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"time\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"rewind\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setUp\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"time\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"skip\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"who\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"give\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"startHoax\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"who\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"origin\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"give\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"startHoax\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"who\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"startHoax\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"who\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"origin\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"startHoax\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"testBatchBurn\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"testComputeMerkleRoot\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"testDecodeStakeDoc\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"testDecodeTransaction\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"testDeploy\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"testHashDatablock\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"testVerifyHeaderMultiStake\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"testVerifyHeadersMultiTx\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"testVerifyStakes\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"testverifyHeader\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"give\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"tip\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"vm\",\"outputs\":[{\"internalType\":\"contract Vm\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
    });
    pub struct ThemelioBridgeTestInternalCalldata<M: Clone>(ethers::contract::Contract<M>);
    impl<M: Clone> Clone for ThemelioBridgeTestInternalCalldata<M> {
        fn clone(&self) -> Self {
            ThemelioBridgeTestInternalCalldata(self.0.clone())
        }
    }
    impl<M: Clone> std::ops::Deref for ThemelioBridgeTestInternalCalldata<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware + Clone> std::fmt::Debug for ThemelioBridgeTestInternalCalldata<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ThemelioBridgeTestInternalCalldata))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware + Clone> ThemelioBridgeTestInternalCalldata<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                THEMELIOBRIDGETESTINTERNALCALLDATA_ABI.clone(),
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
        #[doc = "Calls the contract's `rewind` (0x2d6c17a3) function"]
        pub fn rewind(
            &self,
            time: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([45, 108, 23, 163], time)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setUp` (0x0a9254e4) function"]
        pub fn set_up(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([10, 146, 84, 228], ())
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
        #[doc = "Calls the contract's `testBatchBurn` (0x53ebef70) function"]
        pub fn test_batch_burn(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([83, 235, 239, 112], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `testComputeMerkleRoot` (0x07fc92f9) function"]
        pub fn test_compute_merkle_root(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([7, 252, 146, 249], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `testDecodeStakeDoc` (0xcc3082b5) function"]
        pub fn test_decode_stake_doc(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([204, 48, 130, 181], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `testDecodeTransaction` (0x4869b7f6) function"]
        pub fn test_decode_transaction(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([72, 105, 183, 246], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `testDeploy` (0x354b2735) function"]
        pub fn test_deploy(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([53, 75, 39, 53], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `testHashDatablock` (0xf8551b18) function"]
        pub fn test_hash_datablock(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([248, 85, 27, 24], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `testVerifyHeaderMultiStake` (0xac7ca6af) function"]
        pub fn test_verify_header_multi_stake(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([172, 124, 166, 175], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `testVerifyHeadersMultiTx` (0xb94b08bf) function"]
        pub fn test_verify_headers_multi_tx(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([185, 75, 8, 191], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `testVerifyStakes` (0x079936ac) function"]
        pub fn test_verify_stakes(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([7, 153, 54, 172], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `testverifyHeader` (0x860f5c88) function"]
        pub fn testverify_header(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([134, 15, 92, 136], ())
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
        #[doc = "Calls the contract's `vm` (0x3a768463) function"]
        pub fn vm(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([58, 118, 132, 99], ())
                .expect("method not found (this should never happen)")
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
        pub fn events(
            &self,
        ) -> ethers::contract::builders::Event<M, ThemelioBridgeTestInternalCalldataEvents>
        {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware + Clone> From<ethers::contract::Contract<M>>
        for ThemelioBridgeTestInternalCalldata<M>
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
    pub enum ThemelioBridgeTestInternalCalldataEvents {
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
    impl ethers::contract::EthLogDecode for ThemelioBridgeTestInternalCalldataEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = LogFilter::decode_log(log) {
                return Ok(ThemelioBridgeTestInternalCalldataEvents::LogFilter(decoded));
            }
            if let Ok(decoded) = LogAddressFilter::decode_log(log) {
                return Ok(ThemelioBridgeTestInternalCalldataEvents::LogAddressFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = LogArray1Filter::decode_log(log) {
                return Ok(ThemelioBridgeTestInternalCalldataEvents::LogArray1Filter(
                    decoded,
                ));
            }
            if let Ok(decoded) = LogArray2Filter::decode_log(log) {
                return Ok(ThemelioBridgeTestInternalCalldataEvents::LogArray2Filter(
                    decoded,
                ));
            }
            if let Ok(decoded) = LogArray3Filter::decode_log(log) {
                return Ok(ThemelioBridgeTestInternalCalldataEvents::LogArray3Filter(
                    decoded,
                ));
            }
            if let Ok(decoded) = LogBytesFilter::decode_log(log) {
                return Ok(ThemelioBridgeTestInternalCalldataEvents::LogBytesFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = LogBytes32Filter::decode_log(log) {
                return Ok(ThemelioBridgeTestInternalCalldataEvents::LogBytes32Filter(
                    decoded,
                ));
            }
            if let Ok(decoded) = LogIntFilter::decode_log(log) {
                return Ok(ThemelioBridgeTestInternalCalldataEvents::LogIntFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = LogNamedAddressFilter::decode_log(log) {
                return Ok(ThemelioBridgeTestInternalCalldataEvents::LogNamedAddressFilter(decoded));
            }
            if let Ok(decoded) = LogNamedArray1Filter::decode_log(log) {
                return Ok(ThemelioBridgeTestInternalCalldataEvents::LogNamedArray1Filter(decoded));
            }
            if let Ok(decoded) = LogNamedArray2Filter::decode_log(log) {
                return Ok(ThemelioBridgeTestInternalCalldataEvents::LogNamedArray2Filter(decoded));
            }
            if let Ok(decoded) = LogNamedArray3Filter::decode_log(log) {
                return Ok(ThemelioBridgeTestInternalCalldataEvents::LogNamedArray3Filter(decoded));
            }
            if let Ok(decoded) = LogNamedBytesFilter::decode_log(log) {
                return Ok(ThemelioBridgeTestInternalCalldataEvents::LogNamedBytesFilter(decoded));
            }
            if let Ok(decoded) = LogNamedBytes32Filter::decode_log(log) {
                return Ok(ThemelioBridgeTestInternalCalldataEvents::LogNamedBytes32Filter(decoded));
            }
            if let Ok(decoded) = LogNamedDecimalIntFilter::decode_log(log) {
                return Ok(
                    ThemelioBridgeTestInternalCalldataEvents::LogNamedDecimalIntFilter(decoded),
                );
            }
            if let Ok(decoded) = LogNamedDecimalUintFilter::decode_log(log) {
                return Ok(
                    ThemelioBridgeTestInternalCalldataEvents::LogNamedDecimalUintFilter(decoded),
                );
            }
            if let Ok(decoded) = LogNamedIntFilter::decode_log(log) {
                return Ok(ThemelioBridgeTestInternalCalldataEvents::LogNamedIntFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = LogNamedStringFilter::decode_log(log) {
                return Ok(ThemelioBridgeTestInternalCalldataEvents::LogNamedStringFilter(decoded));
            }
            if let Ok(decoded) = LogNamedUintFilter::decode_log(log) {
                return Ok(ThemelioBridgeTestInternalCalldataEvents::LogNamedUintFilter(decoded));
            }
            if let Ok(decoded) = LogStringFilter::decode_log(log) {
                return Ok(ThemelioBridgeTestInternalCalldataEvents::LogStringFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = LogUintFilter::decode_log(log) {
                return Ok(ThemelioBridgeTestInternalCalldataEvents::LogUintFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = LogsFilter::decode_log(log) {
                return Ok(ThemelioBridgeTestInternalCalldataEvents::LogsFilter(
                    decoded,
                ));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for ThemelioBridgeTestInternalCalldataEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ThemelioBridgeTestInternalCalldataEvents::LogFilter(element) => element.fmt(f),
                ThemelioBridgeTestInternalCalldataEvents::LogAddressFilter(element) => {
                    element.fmt(f)
                }
                ThemelioBridgeTestInternalCalldataEvents::LogArray1Filter(element) => {
                    element.fmt(f)
                }
                ThemelioBridgeTestInternalCalldataEvents::LogArray2Filter(element) => {
                    element.fmt(f)
                }
                ThemelioBridgeTestInternalCalldataEvents::LogArray3Filter(element) => {
                    element.fmt(f)
                }
                ThemelioBridgeTestInternalCalldataEvents::LogBytesFilter(element) => element.fmt(f),
                ThemelioBridgeTestInternalCalldataEvents::LogBytes32Filter(element) => {
                    element.fmt(f)
                }
                ThemelioBridgeTestInternalCalldataEvents::LogIntFilter(element) => element.fmt(f),
                ThemelioBridgeTestInternalCalldataEvents::LogNamedAddressFilter(element) => {
                    element.fmt(f)
                }
                ThemelioBridgeTestInternalCalldataEvents::LogNamedArray1Filter(element) => {
                    element.fmt(f)
                }
                ThemelioBridgeTestInternalCalldataEvents::LogNamedArray2Filter(element) => {
                    element.fmt(f)
                }
                ThemelioBridgeTestInternalCalldataEvents::LogNamedArray3Filter(element) => {
                    element.fmt(f)
                }
                ThemelioBridgeTestInternalCalldataEvents::LogNamedBytesFilter(element) => {
                    element.fmt(f)
                }
                ThemelioBridgeTestInternalCalldataEvents::LogNamedBytes32Filter(element) => {
                    element.fmt(f)
                }
                ThemelioBridgeTestInternalCalldataEvents::LogNamedDecimalIntFilter(element) => {
                    element.fmt(f)
                }
                ThemelioBridgeTestInternalCalldataEvents::LogNamedDecimalUintFilter(element) => {
                    element.fmt(f)
                }
                ThemelioBridgeTestInternalCalldataEvents::LogNamedIntFilter(element) => {
                    element.fmt(f)
                }
                ThemelioBridgeTestInternalCalldataEvents::LogNamedStringFilter(element) => {
                    element.fmt(f)
                }
                ThemelioBridgeTestInternalCalldataEvents::LogNamedUintFilter(element) => {
                    element.fmt(f)
                }
                ThemelioBridgeTestInternalCalldataEvents::LogStringFilter(element) => {
                    element.fmt(f)
                }
                ThemelioBridgeTestInternalCalldataEvents::LogUintFilter(element) => element.fmt(f),
                ThemelioBridgeTestInternalCalldataEvents::LogsFilter(element) => element.fmt(f),
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
    #[doc = "Container type for all input parameters for the `setUp` function with signature `setUp()` and selector `[10, 146, 84, 228]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setUp", abi = "setUp()")]
    pub struct SetUpCall;
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
    #[doc = "Container type for all input parameters for the `testBatchBurn` function with signature `testBatchBurn()` and selector `[83, 235, 239, 112]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "testBatchBurn", abi = "testBatchBurn()")]
    pub struct TestBatchBurnCall;
    #[doc = "Container type for all input parameters for the `testComputeMerkleRoot` function with signature `testComputeMerkleRoot()` and selector `[7, 252, 146, 249]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "testComputeMerkleRoot", abi = "testComputeMerkleRoot()")]
    pub struct TestComputeMerkleRootCall;
    #[doc = "Container type for all input parameters for the `testDecodeStakeDoc` function with signature `testDecodeStakeDoc()` and selector `[204, 48, 130, 181]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "testDecodeStakeDoc", abi = "testDecodeStakeDoc()")]
    pub struct TestDecodeStakeDocCall;
    #[doc = "Container type for all input parameters for the `testDecodeTransaction` function with signature `testDecodeTransaction()` and selector `[72, 105, 183, 246]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "testDecodeTransaction", abi = "testDecodeTransaction()")]
    pub struct TestDecodeTransactionCall;
    #[doc = "Container type for all input parameters for the `testDeploy` function with signature `testDeploy()` and selector `[53, 75, 39, 53]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "testDeploy", abi = "testDeploy()")]
    pub struct TestDeployCall;
    #[doc = "Container type for all input parameters for the `testHashDatablock` function with signature `testHashDatablock()` and selector `[248, 85, 27, 24]`"]
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
    #[doc = "Container type for all input parameters for the `testVerifyHeaderMultiStake` function with signature `testVerifyHeaderMultiStake()` and selector `[172, 124, 166, 175]`"]
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
        name = "testVerifyHeaderMultiStake",
        abi = "testVerifyHeaderMultiStake()"
    )]
    pub struct TestVerifyHeaderMultiStakeCall;
    #[doc = "Container type for all input parameters for the `testVerifyHeadersMultiTx` function with signature `testVerifyHeadersMultiTx()` and selector `[185, 75, 8, 191]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "testVerifyHeadersMultiTx", abi = "testVerifyHeadersMultiTx()")]
    pub struct TestVerifyHeadersMultiTxCall;
    #[doc = "Container type for all input parameters for the `testVerifyStakes` function with signature `testVerifyStakes()` and selector `[7, 153, 54, 172]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "testVerifyStakes", abi = "testVerifyStakes()")]
    pub struct TestVerifyStakesCall;
    #[doc = "Container type for all input parameters for the `testverifyHeader` function with signature `testverifyHeader()` and selector `[134, 15, 92, 136]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "testverifyHeader", abi = "testverifyHeader()")]
    pub struct TestverifyHeaderCall;
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
    pub enum ThemelioBridgeTestInternalCalldataCalls {
        IsScript(IsScriptCall),
        IsTest(IsTestCall),
        DealWithToken(DealWithTokenCall),
        DealWithTokenAndAdjust(DealWithTokenAndAdjustCall),
        Deal(DealCall),
        DeployCodeWithArgs(DeployCodeWithArgsCall),
        DeployCode(DeployCodeCall),
        Failed(FailedCall),
        Hoax0(Hoax0Call),
        Hoax1(Hoax1Call),
        Hoax3(Hoax3Call),
        Hoax2(Hoax2Call),
        Rewind(RewindCall),
        SetUp(SetUpCall),
        Skip(SkipCall),
        StartHoax1(StartHoax1Call),
        StartHoax3(StartHoax3Call),
        StartHoax0(StartHoax0Call),
        StartHoax2(StartHoax2Call),
        TestBatchBurn(TestBatchBurnCall),
        TestComputeMerkleRoot(TestComputeMerkleRootCall),
        TestDecodeStakeDoc(TestDecodeStakeDocCall),
        TestDecodeTransaction(TestDecodeTransactionCall),
        TestDeploy(TestDeployCall),
        TestHashDatablock(TestHashDatablockCall),
        TestVerifyHeaderMultiStake(TestVerifyHeaderMultiStakeCall),
        TestVerifyHeadersMultiTx(TestVerifyHeadersMultiTxCall),
        TestVerifyStakes(TestVerifyStakesCall),
        TestverifyHeader(TestverifyHeaderCall),
        Tip(TipCall),
        Vm(VmCall),
    }
    impl ethers::core::abi::AbiDecode for ThemelioBridgeTestInternalCalldataCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <IsScriptCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestInternalCalldataCalls::IsScript(decoded));
            }
            if let Ok(decoded) = <IsTestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestInternalCalldataCalls::IsTest(decoded));
            }
            if let Ok(decoded) =
                <DealWithTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestInternalCalldataCalls::DealWithToken(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <DealWithTokenAndAdjustCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestInternalCalldataCalls::DealWithTokenAndAdjust(decoded));
            }
            if let Ok(decoded) = <DealCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(ThemelioBridgeTestInternalCalldataCalls::Deal(decoded));
            }
            if let Ok(decoded) =
                <DeployCodeWithArgsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestInternalCalldataCalls::DeployCodeWithArgs(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <DeployCodeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestInternalCalldataCalls::DeployCode(decoded));
            }
            if let Ok(decoded) = <FailedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestInternalCalldataCalls::Failed(decoded));
            }
            if let Ok(decoded) = <Hoax0Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestInternalCalldataCalls::Hoax0(decoded));
            }
            if let Ok(decoded) = <Hoax1Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestInternalCalldataCalls::Hoax1(decoded));
            }
            if let Ok(decoded) = <Hoax3Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestInternalCalldataCalls::Hoax3(decoded));
            }
            if let Ok(decoded) = <Hoax2Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestInternalCalldataCalls::Hoax2(decoded));
            }
            if let Ok(decoded) = <RewindCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestInternalCalldataCalls::Rewind(decoded));
            }
            if let Ok(decoded) = <SetUpCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestInternalCalldataCalls::SetUp(decoded));
            }
            if let Ok(decoded) = <SkipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(ThemelioBridgeTestInternalCalldataCalls::Skip(decoded));
            }
            if let Ok(decoded) =
                <StartHoax1Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestInternalCalldataCalls::StartHoax1(decoded));
            }
            if let Ok(decoded) =
                <StartHoax3Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestInternalCalldataCalls::StartHoax3(decoded));
            }
            if let Ok(decoded) =
                <StartHoax0Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestInternalCalldataCalls::StartHoax0(decoded));
            }
            if let Ok(decoded) =
                <StartHoax2Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestInternalCalldataCalls::StartHoax2(decoded));
            }
            if let Ok(decoded) =
                <TestBatchBurnCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestInternalCalldataCalls::TestBatchBurn(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <TestComputeMerkleRootCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestInternalCalldataCalls::TestComputeMerkleRoot(decoded));
            }
            if let Ok(decoded) =
                <TestDecodeStakeDocCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestInternalCalldataCalls::TestDecodeStakeDoc(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <TestDecodeTransactionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestInternalCalldataCalls::TestDecodeTransaction(decoded));
            }
            if let Ok(decoded) =
                <TestDeployCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestInternalCalldataCalls::TestDeploy(decoded));
            }
            if let Ok(decoded) =
                <TestHashDatablockCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestInternalCalldataCalls::TestHashDatablock(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <TestVerifyHeaderMultiStakeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(
                    ThemelioBridgeTestInternalCalldataCalls::TestVerifyHeaderMultiStake(decoded),
                );
            }
            if let Ok(decoded) =
                <TestVerifyHeadersMultiTxCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(
                    ThemelioBridgeTestInternalCalldataCalls::TestVerifyHeadersMultiTx(decoded),
                );
            }
            if let Ok(decoded) =
                <TestVerifyStakesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestInternalCalldataCalls::TestVerifyStakes(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <TestverifyHeaderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ThemelioBridgeTestInternalCalldataCalls::TestverifyHeader(
                    decoded,
                ));
            }
            if let Ok(decoded) = <TipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(ThemelioBridgeTestInternalCalldataCalls::Tip(decoded));
            }
            if let Ok(decoded) = <VmCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(ThemelioBridgeTestInternalCalldataCalls::Vm(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ThemelioBridgeTestInternalCalldataCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ThemelioBridgeTestInternalCalldataCalls::IsScript(element) => element.encode(),
                ThemelioBridgeTestInternalCalldataCalls::IsTest(element) => element.encode(),
                ThemelioBridgeTestInternalCalldataCalls::DealWithToken(element) => element.encode(),
                ThemelioBridgeTestInternalCalldataCalls::DealWithTokenAndAdjust(element) => {
                    element.encode()
                }
                ThemelioBridgeTestInternalCalldataCalls::Deal(element) => element.encode(),
                ThemelioBridgeTestInternalCalldataCalls::DeployCodeWithArgs(element) => {
                    element.encode()
                }
                ThemelioBridgeTestInternalCalldataCalls::DeployCode(element) => element.encode(),
                ThemelioBridgeTestInternalCalldataCalls::Failed(element) => element.encode(),
                ThemelioBridgeTestInternalCalldataCalls::Hoax0(element) => element.encode(),
                ThemelioBridgeTestInternalCalldataCalls::Hoax1(element) => element.encode(),
                ThemelioBridgeTestInternalCalldataCalls::Hoax3(element) => element.encode(),
                ThemelioBridgeTestInternalCalldataCalls::Hoax2(element) => element.encode(),
                ThemelioBridgeTestInternalCalldataCalls::Rewind(element) => element.encode(),
                ThemelioBridgeTestInternalCalldataCalls::SetUp(element) => element.encode(),
                ThemelioBridgeTestInternalCalldataCalls::Skip(element) => element.encode(),
                ThemelioBridgeTestInternalCalldataCalls::StartHoax1(element) => element.encode(),
                ThemelioBridgeTestInternalCalldataCalls::StartHoax3(element) => element.encode(),
                ThemelioBridgeTestInternalCalldataCalls::StartHoax0(element) => element.encode(),
                ThemelioBridgeTestInternalCalldataCalls::StartHoax2(element) => element.encode(),
                ThemelioBridgeTestInternalCalldataCalls::TestBatchBurn(element) => element.encode(),
                ThemelioBridgeTestInternalCalldataCalls::TestComputeMerkleRoot(element) => {
                    element.encode()
                }
                ThemelioBridgeTestInternalCalldataCalls::TestDecodeStakeDoc(element) => {
                    element.encode()
                }
                ThemelioBridgeTestInternalCalldataCalls::TestDecodeTransaction(element) => {
                    element.encode()
                }
                ThemelioBridgeTestInternalCalldataCalls::TestDeploy(element) => element.encode(),
                ThemelioBridgeTestInternalCalldataCalls::TestHashDatablock(element) => {
                    element.encode()
                }
                ThemelioBridgeTestInternalCalldataCalls::TestVerifyHeaderMultiStake(element) => {
                    element.encode()
                }
                ThemelioBridgeTestInternalCalldataCalls::TestVerifyHeadersMultiTx(element) => {
                    element.encode()
                }
                ThemelioBridgeTestInternalCalldataCalls::TestVerifyStakes(element) => {
                    element.encode()
                }
                ThemelioBridgeTestInternalCalldataCalls::TestverifyHeader(element) => {
                    element.encode()
                }
                ThemelioBridgeTestInternalCalldataCalls::Tip(element) => element.encode(),
                ThemelioBridgeTestInternalCalldataCalls::Vm(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ThemelioBridgeTestInternalCalldataCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ThemelioBridgeTestInternalCalldataCalls::IsScript(element) => element.fmt(f),
                ThemelioBridgeTestInternalCalldataCalls::IsTest(element) => element.fmt(f),
                ThemelioBridgeTestInternalCalldataCalls::DealWithToken(element) => element.fmt(f),
                ThemelioBridgeTestInternalCalldataCalls::DealWithTokenAndAdjust(element) => {
                    element.fmt(f)
                }
                ThemelioBridgeTestInternalCalldataCalls::Deal(element) => element.fmt(f),
                ThemelioBridgeTestInternalCalldataCalls::DeployCodeWithArgs(element) => {
                    element.fmt(f)
                }
                ThemelioBridgeTestInternalCalldataCalls::DeployCode(element) => element.fmt(f),
                ThemelioBridgeTestInternalCalldataCalls::Failed(element) => element.fmt(f),
                ThemelioBridgeTestInternalCalldataCalls::Hoax0(element) => element.fmt(f),
                ThemelioBridgeTestInternalCalldataCalls::Hoax1(element) => element.fmt(f),
                ThemelioBridgeTestInternalCalldataCalls::Hoax3(element) => element.fmt(f),
                ThemelioBridgeTestInternalCalldataCalls::Hoax2(element) => element.fmt(f),
                ThemelioBridgeTestInternalCalldataCalls::Rewind(element) => element.fmt(f),
                ThemelioBridgeTestInternalCalldataCalls::SetUp(element) => element.fmt(f),
                ThemelioBridgeTestInternalCalldataCalls::Skip(element) => element.fmt(f),
                ThemelioBridgeTestInternalCalldataCalls::StartHoax1(element) => element.fmt(f),
                ThemelioBridgeTestInternalCalldataCalls::StartHoax3(element) => element.fmt(f),
                ThemelioBridgeTestInternalCalldataCalls::StartHoax0(element) => element.fmt(f),
                ThemelioBridgeTestInternalCalldataCalls::StartHoax2(element) => element.fmt(f),
                ThemelioBridgeTestInternalCalldataCalls::TestBatchBurn(element) => element.fmt(f),
                ThemelioBridgeTestInternalCalldataCalls::TestComputeMerkleRoot(element) => {
                    element.fmt(f)
                }
                ThemelioBridgeTestInternalCalldataCalls::TestDecodeStakeDoc(element) => {
                    element.fmt(f)
                }
                ThemelioBridgeTestInternalCalldataCalls::TestDecodeTransaction(element) => {
                    element.fmt(f)
                }
                ThemelioBridgeTestInternalCalldataCalls::TestDeploy(element) => element.fmt(f),
                ThemelioBridgeTestInternalCalldataCalls::TestHashDatablock(element) => {
                    element.fmt(f)
                }
                ThemelioBridgeTestInternalCalldataCalls::TestVerifyHeaderMultiStake(element) => {
                    element.fmt(f)
                }
                ThemelioBridgeTestInternalCalldataCalls::TestVerifyHeadersMultiTx(element) => {
                    element.fmt(f)
                }
                ThemelioBridgeTestInternalCalldataCalls::TestVerifyStakes(element) => {
                    element.fmt(f)
                }
                ThemelioBridgeTestInternalCalldataCalls::TestverifyHeader(element) => {
                    element.fmt(f)
                }
                ThemelioBridgeTestInternalCalldataCalls::Tip(element) => element.fmt(f),
                ThemelioBridgeTestInternalCalldataCalls::Vm(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<IsScriptCall> for ThemelioBridgeTestInternalCalldataCalls {
        fn from(var: IsScriptCall) -> Self {
            ThemelioBridgeTestInternalCalldataCalls::IsScript(var)
        }
    }
    impl ::std::convert::From<IsTestCall> for ThemelioBridgeTestInternalCalldataCalls {
        fn from(var: IsTestCall) -> Self {
            ThemelioBridgeTestInternalCalldataCalls::IsTest(var)
        }
    }
    impl ::std::convert::From<DealWithTokenCall> for ThemelioBridgeTestInternalCalldataCalls {
        fn from(var: DealWithTokenCall) -> Self {
            ThemelioBridgeTestInternalCalldataCalls::DealWithToken(var)
        }
    }
    impl ::std::convert::From<DealWithTokenAndAdjustCall> for ThemelioBridgeTestInternalCalldataCalls {
        fn from(var: DealWithTokenAndAdjustCall) -> Self {
            ThemelioBridgeTestInternalCalldataCalls::DealWithTokenAndAdjust(var)
        }
    }
    impl ::std::convert::From<DealCall> for ThemelioBridgeTestInternalCalldataCalls {
        fn from(var: DealCall) -> Self {
            ThemelioBridgeTestInternalCalldataCalls::Deal(var)
        }
    }
    impl ::std::convert::From<DeployCodeWithArgsCall> for ThemelioBridgeTestInternalCalldataCalls {
        fn from(var: DeployCodeWithArgsCall) -> Self {
            ThemelioBridgeTestInternalCalldataCalls::DeployCodeWithArgs(var)
        }
    }
    impl ::std::convert::From<DeployCodeCall> for ThemelioBridgeTestInternalCalldataCalls {
        fn from(var: DeployCodeCall) -> Self {
            ThemelioBridgeTestInternalCalldataCalls::DeployCode(var)
        }
    }
    impl ::std::convert::From<FailedCall> for ThemelioBridgeTestInternalCalldataCalls {
        fn from(var: FailedCall) -> Self {
            ThemelioBridgeTestInternalCalldataCalls::Failed(var)
        }
    }
    impl ::std::convert::From<Hoax0Call> for ThemelioBridgeTestInternalCalldataCalls {
        fn from(var: Hoax0Call) -> Self {
            ThemelioBridgeTestInternalCalldataCalls::Hoax0(var)
        }
    }
    impl ::std::convert::From<Hoax1Call> for ThemelioBridgeTestInternalCalldataCalls {
        fn from(var: Hoax1Call) -> Self {
            ThemelioBridgeTestInternalCalldataCalls::Hoax1(var)
        }
    }
    impl ::std::convert::From<Hoax3Call> for ThemelioBridgeTestInternalCalldataCalls {
        fn from(var: Hoax3Call) -> Self {
            ThemelioBridgeTestInternalCalldataCalls::Hoax3(var)
        }
    }
    impl ::std::convert::From<Hoax2Call> for ThemelioBridgeTestInternalCalldataCalls {
        fn from(var: Hoax2Call) -> Self {
            ThemelioBridgeTestInternalCalldataCalls::Hoax2(var)
        }
    }
    impl ::std::convert::From<RewindCall> for ThemelioBridgeTestInternalCalldataCalls {
        fn from(var: RewindCall) -> Self {
            ThemelioBridgeTestInternalCalldataCalls::Rewind(var)
        }
    }
    impl ::std::convert::From<SetUpCall> for ThemelioBridgeTestInternalCalldataCalls {
        fn from(var: SetUpCall) -> Self {
            ThemelioBridgeTestInternalCalldataCalls::SetUp(var)
        }
    }
    impl ::std::convert::From<SkipCall> for ThemelioBridgeTestInternalCalldataCalls {
        fn from(var: SkipCall) -> Self {
            ThemelioBridgeTestInternalCalldataCalls::Skip(var)
        }
    }
    impl ::std::convert::From<StartHoax1Call> for ThemelioBridgeTestInternalCalldataCalls {
        fn from(var: StartHoax1Call) -> Self {
            ThemelioBridgeTestInternalCalldataCalls::StartHoax1(var)
        }
    }
    impl ::std::convert::From<StartHoax3Call> for ThemelioBridgeTestInternalCalldataCalls {
        fn from(var: StartHoax3Call) -> Self {
            ThemelioBridgeTestInternalCalldataCalls::StartHoax3(var)
        }
    }
    impl ::std::convert::From<StartHoax0Call> for ThemelioBridgeTestInternalCalldataCalls {
        fn from(var: StartHoax0Call) -> Self {
            ThemelioBridgeTestInternalCalldataCalls::StartHoax0(var)
        }
    }
    impl ::std::convert::From<StartHoax2Call> for ThemelioBridgeTestInternalCalldataCalls {
        fn from(var: StartHoax2Call) -> Self {
            ThemelioBridgeTestInternalCalldataCalls::StartHoax2(var)
        }
    }
    impl ::std::convert::From<TestBatchBurnCall> for ThemelioBridgeTestInternalCalldataCalls {
        fn from(var: TestBatchBurnCall) -> Self {
            ThemelioBridgeTestInternalCalldataCalls::TestBatchBurn(var)
        }
    }
    impl ::std::convert::From<TestComputeMerkleRootCall> for ThemelioBridgeTestInternalCalldataCalls {
        fn from(var: TestComputeMerkleRootCall) -> Self {
            ThemelioBridgeTestInternalCalldataCalls::TestComputeMerkleRoot(var)
        }
    }
    impl ::std::convert::From<TestDecodeStakeDocCall> for ThemelioBridgeTestInternalCalldataCalls {
        fn from(var: TestDecodeStakeDocCall) -> Self {
            ThemelioBridgeTestInternalCalldataCalls::TestDecodeStakeDoc(var)
        }
    }
    impl ::std::convert::From<TestDecodeTransactionCall> for ThemelioBridgeTestInternalCalldataCalls {
        fn from(var: TestDecodeTransactionCall) -> Self {
            ThemelioBridgeTestInternalCalldataCalls::TestDecodeTransaction(var)
        }
    }
    impl ::std::convert::From<TestDeployCall> for ThemelioBridgeTestInternalCalldataCalls {
        fn from(var: TestDeployCall) -> Self {
            ThemelioBridgeTestInternalCalldataCalls::TestDeploy(var)
        }
    }
    impl ::std::convert::From<TestHashDatablockCall> for ThemelioBridgeTestInternalCalldataCalls {
        fn from(var: TestHashDatablockCall) -> Self {
            ThemelioBridgeTestInternalCalldataCalls::TestHashDatablock(var)
        }
    }
    impl ::std::convert::From<TestVerifyHeaderMultiStakeCall>
        for ThemelioBridgeTestInternalCalldataCalls
    {
        fn from(var: TestVerifyHeaderMultiStakeCall) -> Self {
            ThemelioBridgeTestInternalCalldataCalls::TestVerifyHeaderMultiStake(var)
        }
    }
    impl ::std::convert::From<TestVerifyHeadersMultiTxCall>
        for ThemelioBridgeTestInternalCalldataCalls
    {
        fn from(var: TestVerifyHeadersMultiTxCall) -> Self {
            ThemelioBridgeTestInternalCalldataCalls::TestVerifyHeadersMultiTx(var)
        }
    }
    impl ::std::convert::From<TestVerifyStakesCall> for ThemelioBridgeTestInternalCalldataCalls {
        fn from(var: TestVerifyStakesCall) -> Self {
            ThemelioBridgeTestInternalCalldataCalls::TestVerifyStakes(var)
        }
    }
    impl ::std::convert::From<TestverifyHeaderCall> for ThemelioBridgeTestInternalCalldataCalls {
        fn from(var: TestverifyHeaderCall) -> Self {
            ThemelioBridgeTestInternalCalldataCalls::TestverifyHeader(var)
        }
    }
    impl ::std::convert::From<TipCall> for ThemelioBridgeTestInternalCalldataCalls {
        fn from(var: TipCall) -> Self {
            ThemelioBridgeTestInternalCalldataCalls::Tip(var)
        }
    }
    impl ::std::convert::From<VmCall> for ThemelioBridgeTestInternalCalldataCalls {
        fn from(var: VmCall) -> Self {
            ThemelioBridgeTestInternalCalldataCalls::Vm(var)
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
