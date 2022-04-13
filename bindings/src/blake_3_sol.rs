pub use blake3sol_mod::*;
#[allow(clippy::too_many_arguments)]
mod blake3sol_mod {
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
    #[doc = "Blake3Sol was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static BLAKE3SOL_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"struct Hasher\",\"name\":\"self\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct ChunkState\",\"name\":\"chunk_state\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint32[8]\",\"name\":\"chaining_value\",\"type\":\"uint32[8]\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"chunk_counter\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"block_bytes\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"block_len\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"blocks_compressed\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"flags\",\"type\":\"uint32\",\"components\":[]}]},{\"internalType\":\"uint32[8]\",\"name\":\"key_words\",\"type\":\"uint32[8]\",\"components\":[]},{\"internalType\":\"uint32[8][54]\",\"name\":\"cv_stack\",\"type\":\"uint32[8][54]\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"cv_stack_len\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"flags\",\"type\":\"uint32\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"finalize\",\"outputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"context\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"new_derive_key\",\"outputs\":[{\"internalType\":\"struct Hasher\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct ChunkState\",\"name\":\"chunk_state\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint32[8]\",\"name\":\"chaining_value\",\"type\":\"uint32[8]\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"chunk_counter\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"block_bytes\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"block_len\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"blocks_compressed\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"flags\",\"type\":\"uint32\",\"components\":[]}]},{\"internalType\":\"uint32[8]\",\"name\":\"key_words\",\"type\":\"uint32[8]\",\"components\":[]},{\"internalType\":\"uint32[8][54]\",\"name\":\"cv_stack\",\"type\":\"uint32[8][54]\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"cv_stack_len\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"flags\",\"type\":\"uint32\",\"components\":[]}]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"new_hasher\",\"outputs\":[{\"internalType\":\"struct Hasher\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct ChunkState\",\"name\":\"chunk_state\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint32[8]\",\"name\":\"chaining_value\",\"type\":\"uint32[8]\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"chunk_counter\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"block_bytes\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"block_len\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"blocks_compressed\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"flags\",\"type\":\"uint32\",\"components\":[]}]},{\"internalType\":\"uint32[8]\",\"name\":\"key_words\",\"type\":\"uint32[8]\",\"components\":[]},{\"internalType\":\"uint32[8][54]\",\"name\":\"cv_stack\",\"type\":\"uint32[8][54]\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"cv_stack_len\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"flags\",\"type\":\"uint32\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"key\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"new_keyed\",\"outputs\":[{\"internalType\":\"struct Hasher\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct ChunkState\",\"name\":\"chunk_state\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint32[8]\",\"name\":\"chaining_value\",\"type\":\"uint32[8]\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"chunk_counter\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"block_bytes\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"block_len\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"blocks_compressed\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"flags\",\"type\":\"uint32\",\"components\":[]}]},{\"internalType\":\"uint32[8]\",\"name\":\"key_words\",\"type\":\"uint32[8]\",\"components\":[]},{\"internalType\":\"uint32[8][54]\",\"name\":\"cv_stack\",\"type\":\"uint32[8][54]\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"cv_stack_len\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"flags\",\"type\":\"uint32\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"struct Hasher\",\"name\":\"self\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct ChunkState\",\"name\":\"chunk_state\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint32[8]\",\"name\":\"chaining_value\",\"type\":\"uint32[8]\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"chunk_counter\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"block_bytes\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"block_len\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"blocks_compressed\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"flags\",\"type\":\"uint32\",\"components\":[]}]},{\"internalType\":\"uint32[8]\",\"name\":\"key_words\",\"type\":\"uint32[8]\",\"components\":[]},{\"internalType\":\"uint32[8][54]\",\"name\":\"cv_stack\",\"type\":\"uint32[8][54]\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"cv_stack_len\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"flags\",\"type\":\"uint32\",\"components\":[]}]},{\"internalType\":\"bytes\",\"name\":\"input\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"update_hasher\",\"outputs\":[{\"internalType\":\"struct Hasher\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct ChunkState\",\"name\":\"chunk_state\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint32[8]\",\"name\":\"chaining_value\",\"type\":\"uint32[8]\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"chunk_counter\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"block_bytes\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"block_len\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"blocks_compressed\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"flags\",\"type\":\"uint32\",\"components\":[]}]},{\"internalType\":\"uint32[8]\",\"name\":\"key_words\",\"type\":\"uint32[8]\",\"components\":[]},{\"internalType\":\"uint32[8][54]\",\"name\":\"cv_stack\",\"type\":\"uint32[8][54]\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"cv_stack_len\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"flags\",\"type\":\"uint32\",\"components\":[]}]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static BLAKE3SOL_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x61028060405260026080908152600660a052600360c052600a60e0526007610100526000610120819052600461014052600d61016052600161018052600b6101a052600c6101c05260056101e052600961020052600e61022052600f6102405260086102605262000072916010620000f2565b506040805161010081018252636a09e667815263bb67ae856020820152633c6ef3729181019190915263a54ff53a606082015263510e527f6080820152639b05688c60a0820152631f83d9ab60c0820152635be0cd1960e0820152620000dd9060019060086200018c565b50348015620000eb57600080fd5b5062000232565b6001830191839082156200017a5791602002820160005b838211156200014957835183826101000a81548160ff021916908360ff160217905550926020019260010160208160000104928301926001030262000109565b8015620001785782816101000a81549060ff021916905560010160208160000104928301926001030262000149565b505b50620001889291506200021b565b5090565b6001830191839082156200017a5791602002820160005b83821115620001e957835183826101000a81548163ffffffff021916908363ffffffff1602179055509260200192600401602081600301049283019260010302620001a3565b8015620001785782816101000a81549063ffffffff0219169055600401602081600301049283019260010302620001e9565b5b808211156200018857600081556001016200021c565b61202d80620002426000396000f3fe608060405234801561001057600080fd5b50600436106100575760003560e01c80630d056d9b1461005c57806312a63b5d1461007a578063821586271461008d5780638855ee95146100a0578063c51d2cd7146100c0575b600080fd5b6100646100d3565b60405161007191906117dd565b60405180910390f35b610064610088366004611900565b610144565b61006461009b366004611c6b565b610218565b6100b36100ae366004611cd3565b610334565b6040516100719190611d0f565b6100646100ce366004611900565b610363565b6100db611614565b6040805161010081019182905261013f91600190600890826000855b82829054906101000a900463ffffffff1663ffffffff16815260200190600401906020826003010492830192600103820291508084116100f7579050505050505060006103cf565b905090565b61014c611614565b604080516101008101918290526000916101b391906001906008908286855b82829054906101000a900463ffffffff1663ffffffff168152602001906004019060208260030104928301926001038202915080841161016b579050505050505060206103cf565b90506101c0818585610218565b506040805161010080825261012082019092526000916020820181803683370190505090506101ef8282610424565b6101f7611655565b61020182826104ab565b61020c8160406103cf565b93505050505b92915050565b610220611614565b60005b63ffffffff811683111561032b578451610400906102409061058c565b63ffffffff16141561029f57600061026361025e87600001516105b1565b610630565b86516020015190915060009061027a906001611d38565b905061028787838361066d565b61029a87602001518289608001516106af565b875250505b60006102ae866000015161058c565b6102ba90610400611d63565b905060006102d7826102d263ffffffff861688611d88565b610716565b90503660008763ffffffff8616886102ef8887611d9f565b63ffffffff169261030293929190611dbe565b915091506103158960000151838361073a565b506103208386611d9f565b945050505050610223565b50929392505050565b604080516020808252818301909252606091600091906020820181803683370190505090506102128382610424565b61036b611614565b610373611655565b600084848080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152509293506103bb92508391508490506104ab565b6103c68260106103cf565b95945050505050565b6103d7611614565b6103df611674565b6040518060a001604052806103f6866000876106af565b8152602001858152602001828152602001600060ff1681526020018463ffffffff1681525091505092915050565b600061043383600001516105b1565b606084015190915060ff165b63ffffffff81161561049b57610456600182611d63565b905061049484604001518263ffffffff166036811061047757610477611de8565b602002015161048584610630565b866020015187608001516108e0565b915061043f565b6104a582846109e7565b50505050565b60208251111561051e5760405162461bcd60e51b815260206004820152603360248201527f4461746120627974657320697320746f6f206c6f6e6720746f20636f6e7665726044820152727420746f203820342d6279746520776f72647360681b60648201526084015b60405180910390fd5b60005b6004835161052f9190611e14565b8160ff1610156105875761055083610548836004611e28565b60ff16610a61565b828260ff166008811061056557610565611de8565b63ffffffff90921660209290920201528061057f81611e51565b915050610521565b505050565b60008160600151826080015160406105a49190611e28565b60ff166102129190611d9f565b6105b96116a2565b6105c16116dd565b6105cf836040015182610b3a565b6040518060a001604052808460000151815260200182815260200184602001516001600160401b03168152602001846060015163ffffffff168152602001600261061886610c21565b8660a00151171763ffffffff16815250915050919050565b610638611655565b600061065b83600001518460200151856040015186606001518760800151610c48565b905061066681610ee7565b9392505050565b600181166106a55761069161068184610f5e565b8385602001518660800151610faa565b915060011c677fffffffffffffff1661066d565b6105878383610fc1565b6106b76116fc565b6040805181815260608101825260009160208201818036833750506040805160c0810182529788526001600160401b03969096166020880152509385019390935250600060608401819052608084015263ffffffff1660a08301525090565b60008163ffffffff168363ffffffff161015610733575081610212565b5080610212565b6000805b63ffffffff81168311156108d857606085015163ffffffff16604014156107e6576107676116dd565b610775866040015182610b3a565b855160208701516107a19161079c91849060406107918c610c21565b8c60a0015117610c48565b610ee7565b8652608086018051600191906107b8908390611e71565b60ff169052506040805181815260608101825290602082018180368337505050604087015250600060608601525b60608501516000906107f9906040611d63565b90506000610811826102d263ffffffff861688611d88565b905060005b8163ffffffff168163ffffffff1610156108a95786866108368387611d9f565b63ffffffff1681811061084b5761084b611de8565b9050013560f81c60f81b886040015189606001518361086a9190611d9f565b63ffffffff168151811061088057610880611de8565b60200101906001600160f81b031916908160001a905350806108a181611e96565b915050610816565b5080876060018181516108bc9190611d9f565b63ffffffff169052506108cf8184611d9f565b9250505061073e565b509392505050565b6108e86116a2565b6108f06116dd565b60005b60088160ff16101561095057868160ff166008811061091457610914611de8565b6020020151828260ff166010811061092e5761092e611de8565b63ffffffff90921660209290920201528061094881611e51565b9150506108f3565b5060085b60108160ff1610156109b157858160ff166008811061097557610975611de8565b6020020151828260ff166010811061098f5761098f611de8565b63ffffffff9092166020929092020152806109a981611e51565b915050610954565b506040805160a081018252948552602085019190915260008482015260608401525060041763ffffffff16608082015292915050565b6000610a0a83600001518460200151600086606001516008886080015117610c48565b905060005b60088163ffffffff1610156104a557610a4f828263ffffffff1660108110610a3957610a39611de8565b602002015184610a4a846004611eba565b611003565b80610a5981611e96565b915050610a0f565b6000610a6e826004611ee6565b83511015610abe5760405162461bcd60e51b815260206004820152601f60248201527f6c655f62797465735f6765745f75696e7433325f6f75744f66426f756e6473006044820152606401610515565b6000805b60048160ff1610156108d857610ad9816008611e28565b60ff168585610ae9846003611efe565b60ff16610af69190611ee6565b81518110610b0657610b06611de8565b0160200151610b26916001600160f81b0319909116901c60e01c83611d9f565b915080610b3281611e51565b915050610ac2565b6040825111158015610b57575060048251610b559190611f21565b155b610bc05760405162461bcd60e51b815260206004820152603460248201527f4461746120627974657320697320746f6f206c6f6e6720746f20636f6e7665726044820152737420746f20313620342d6279746520776f72647360601b6064820152608401610515565b60005b60048351610bd19190611e14565b8160ff16101561058757610bea83610548836004611e28565b828260ff1660108110610bff57610bff611de8565b63ffffffff909216602092909202015280610c1981611e51565b915050610bc3565b6000816080015160ff1660001415610c3b57506001919050565b506000919050565b919050565b610c506116dd565b610c586116dd565b60005b60108160ff161015610cb857868160ff1660108110610c7c57610c7c611de8565b6020020151828260ff1660108110610c9657610c96611de8565b63ffffffff909216602092909202015280610cb081611e51565b915050610c5b565b50600060405180610200016040528089600060088110610cda57610cda611de8565b6020908102919091015163ffffffff90811683528b8201518116838301526040808d01518216908401526060808d01518216908401526080808d015182169084015260a0808d015182169084015260c0808d015182169084015260e0808d0151821690840152600154808216610100850152640100000000810482166101208501526801000000000000000081048216610140850152600160601b900481166101608401528981166101808401529089901c81166101a08301528781166101c083015286166101e0909101529050610db28183611088565b610dbb82611170565b610dc58183611088565b610dce82611170565b610dd88183611088565b610de182611170565b610deb8183611088565b610df482611170565b610dfe8183611088565b610e0782611170565b610e118183611088565b610e1a82611170565b610e248183611088565b60005b60088160ff161015610edb5781610e3f826008611e71565b60ff1660108110610e5257610e52611de8565b6020020151828260ff1660108110610e6c57610e6c611de8565b6020020180519190911863ffffffff1690528860ff821660088110610e9357610e93611de8565b602002015182610ea4836008611e71565b60ff1660108110610eb757610eb7611de8565b6020020180519190911863ffffffff16905280610ed381611e51565b915050610e27565b50979650505050505050565b610eef611655565b610ef7611655565b60005b60088160ff161015610f5757838160ff1660108110610f1b57610f1b611de8565b6020020151828260ff1660088110610f3557610f35611de8565b63ffffffff909216602092909202015280610f4f81611e51565b915050610efa565b5092915050565b610f66611655565b600182606001818151610f799190611efe565b60ff908116909152604084015160608501519092501660368110610f9f57610f9f611de8565b602002015192915050565b610fb2611655565b6103c661025e868686866108e0565b808260400151836060015160ff1660368110610fdf57610fdf611de8565b602002015260608201805160019190610ff9908390611e71565b60ff169052505050565b60005b60048160ff1610156104a55761101d816008611e28565b611028906002612011565b6110389063ffffffff8616611e14565b60f81b8361104960ff841685611d9f565b63ffffffff168151811061105f5761105f611de8565b60200101906001600160f81b031916908160001a9053508061108081611e51565b915050611006565b6110a982600060046008600c868460200201518760015b6020020151611261565b6110c582600160056009600d866002602002015187600361109f565b6110e18260026006600a600e866004602002015187600561109f565b6110fd8260036007600b600f866006602002015187600761109f565b6111198260006005600a600f866008602002015187600961109f565b6111358260016006600b600c86600a602002015187600b61109f565b61115182600260076008600d86600c602002015187600d61109f565b61116c82600360046009600e8681602002015187600f61109f565b5050565b6111786116dd565b60005b60108160ff161015611200578260008260ff166010811061119e5761119e611de8565b602081049091015460ff601f9092166101000a900416601081106111c4576111c4611de8565b6020020151828260ff16601081106111de576111de611de8565b63ffffffff9092166020929092020152806111f881611e51565b91505061117b565b5060005b60108160ff16101561058757818160ff166010811061122557611225611de8565b6020020151838260ff166010811061123f5761123f611de8565b63ffffffff90921660209290920201528061125981611e51565b915050611204565b81878663ffffffff166010811061127a5761127a611de8565b6020020151888863ffffffff166010811061129757611297611de8565b60200201510101878763ffffffff16601081106112b6576112b6611de8565b602002019063ffffffff16908163ffffffff1681525050611312878763ffffffff16601081106112e8576112e8611de8565b6020020151888563ffffffff166010811061130557611305611de8565b60200201511860106115df565b878463ffffffff166010811061132a5761132a611de8565b602002019063ffffffff16908163ffffffff1681525050868363ffffffff166010811061135957611359611de8565b6020020151878563ffffffff166010811061137657611376611de8565b602002015101878563ffffffff166010811061139457611394611de8565b602002019063ffffffff16908163ffffffff16815250506113f0878563ffffffff16601081106113c6576113c6611de8565b6020020151888763ffffffff16601081106113e3576113e3611de8565b602002015118600c6115df565b878663ffffffff166010811061140857611408611de8565b602002019063ffffffff16908163ffffffff168152505080878663ffffffff166010811061143857611438611de8565b6020020151888863ffffffff166010811061145557611455611de8565b60200201510101878763ffffffff166010811061147457611474611de8565b602002019063ffffffff16908163ffffffff16815250506114d0878763ffffffff16601081106114a6576114a6611de8565b6020020151888563ffffffff16601081106114c3576114c3611de8565b60200201511860086115df565b878463ffffffff16601081106114e8576114e8611de8565b602002019063ffffffff16908163ffffffff1681525050868363ffffffff166010811061151757611517611de8565b6020020151878563ffffffff166010811061153457611534611de8565b602002015101878563ffffffff166010811061155257611552611de8565b602002019063ffffffff16908163ffffffff16815250506115ae878563ffffffff166010811061158457611584611de8565b6020020151888763ffffffff16601081106115a1576115a1611de8565b60200201511860076115df565b878663ffffffff16601081106115c6576115c6611de8565b63ffffffff909216602092909202015250505050505050565b600060e083901b6115f1836020611efe565b6001600160e01b031990911660ff84811682901c92161b1760e01c905092915050565b6040518060a001604052806116276116fc565b8152602001611634611655565b8152602001611641611674565b815260006020820181905260409091015290565b6040518061010001604052806008906020820280368337509192915050565b604051806106c001604052806036905b61168c611655565b8152602001906001900390816116845790505090565b6040518060a001604052806116b5611655565b81526020016116c26116dd565b81526000602082018190526040820181905260609091015290565b6040518061020001604052806010906020820280368337509192915050565b6040518060c0016040528061170f611655565b815260006020820181905260606040830181905282018190526080820181905260a09091015290565b8060005b60088110156104a557815163ffffffff1684526020938401939091019060010161173c565b6000815180845260005b818110156117875760208185018101518683018201520161176b565b81811115611799576000602083870101525b50601f01601f19169290920160200192915050565b8060005b60368110156104a5576117c6848351611738565b6101009390930192602091909101906001016117b2565b60208152600082516137608060208501526117fd61378085018351611738565b6001600160401b0360208301511661388085015260408201516101a06138a086015261182d613920860182611761565b905063ffffffff6060840151166138c086015260ff6080840151166138e086015260a0830151925061186861390086018463ffffffff169052565b6020860151925061187c6040860184611738565b604086015192506118916101408601846117ae565b606086015160ff1661374086015260809095015163ffffffff169301929092525090919050565b60008083601f8401126118ca57600080fd5b5081356001600160401b038111156118e157600080fd5b6020830191508360208285010111156118f957600080fd5b9250929050565b6000806020838503121561191357600080fd5b82356001600160401b0381111561192957600080fd5b611935858286016118b8565b90969095509350505050565b634e487b7160e01b600052604160045260246000fd5b60405160a081016001600160401b038111828210171561197957611979611941565b60405290565b60405160c081016001600160401b038111828210171561197957611979611941565b803563ffffffff81168114610c4357600080fd5b600082601f8301126119c657600080fd5b6040516101008082018281106001600160401b03821117156119ea576119ea611941565b604052830181858211156119fd57600080fd5b845b82811015611a1e57611a10816119a1565b8252602091820191016119ff565b509195945050505050565b80356001600160401b0381168114610c4357600080fd5b600082601f830112611a5157600080fd5b81356001600160401b0380821115611a6b57611a6b611941565b604051601f8301601f19908116603f01168101908282118183101715611a9357611a93611941565b81604052838152866020858801011115611aac57600080fd5b836020870160208301376000602085830101528094505050505092915050565b803560ff81168114610c4357600080fd5b600082601f830112611aee57600080fd5b6040516106c081018181106001600160401b0382111715611b1157611b11611941565b60405280613600840185811115611b2757600080fd5b845b81811015611a1e57611b3b87826119b5565b835260209092019161010001611b29565b60006137608284031215611b5f57600080fd5b611b67611957565b905081356001600160401b0380821115611b8057600080fd5b908301906101a08286031215611b9557600080fd5b611b9d61197f565b611ba786846119b5565b8152611bb66101008401611a29565b60208201526101208084013583811115611bcf57600080fd5b611bdb88828701611a40565b604084015250611bee61014085016119a1565b6060830152611c006101608501611acc565b6080830152611c1261018085016119a1565b60a0830152818552611c2787602088016119b5565b6020860152611c3887828801611add565b604086015250505050611c4e6137208301611acc565b6060820152611c6061374083016119a1565b608082015292915050565b600080600060408486031215611c8057600080fd5b83356001600160401b0380821115611c9757600080fd5b611ca387838801611b4c565b94506020860135915080821115611cb957600080fd5b50611cc6868287016118b8565b9497909650939450505050565b600060208284031215611ce557600080fd5b81356001600160401b03811115611cfb57600080fd5b611d0784828501611b4c565b949350505050565b6020815260006106666020830184611761565b634e487b7160e01b600052601160045260246000fd5b60006001600160401b03808316818516808303821115611d5a57611d5a611d22565b01949350505050565b600063ffffffff83811690831681811015611d8057611d80611d22565b039392505050565b600082821015611d9a57611d9a611d22565b500390565b600063ffffffff808316818516808303821115611d5a57611d5a611d22565b60008085851115611dce57600080fd5b83861115611ddb57600080fd5b5050820193919092039150565b634e487b7160e01b600052603260045260246000fd5b634e487b7160e01b600052601260045260246000fd5b600082611e2357611e23611dfe565b500490565b600060ff821660ff84168160ff0481118215151615611e4957611e49611d22565b029392505050565b600060ff821660ff811415611e6857611e68611d22565b60010192915050565b600060ff821660ff84168060ff03821115611e8e57611e8e611d22565b019392505050565b600063ffffffff80831681811415611eb057611eb0611d22565b6001019392505050565b600063ffffffff80831681851681830481118215151615611edd57611edd611d22565b02949350505050565b60008219821115611ef957611ef9611d22565b500190565b600060ff821660ff841680821015611f1857611f18611d22565b90039392505050565b600082611f3057611f30611dfe565b500690565b600181815b80851115611f70578160001904821115611f5657611f56611d22565b80851615611f6357918102915b93841c9390800290611f3a565b509250929050565b600082611f8757506001610212565b81611f9457506000610212565b8160018114611faa5760028114611fb457611fd0565b6001915050610212565b60ff841115611fc557611fc5611d22565b50506001821b610212565b5060208310610133831016604e8410600b8410161715611ff3575081810a610212565b611ffd8383611f35565b8060001904821115611e4957611e49611d22565b600061066660ff841683611f7856fea164736f6c634300080a000a" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct Blake3Sol<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for Blake3Sol<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for Blake3Sol<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Blake3Sol))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> Blake3Sol<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), BLAKE3SOL_ABI.clone(), client).into()
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
                BLAKE3SOL_ABI.clone(),
                BLAKE3SOL_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `finalize` (0x8855ee95) function"]
        pub fn finalize(
            &self,
            self_: Hasher,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Bytes> {
            self.0
                .method_hash([136, 85, 238, 149], (self_,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `new_derive_key` (0x12a63b5d) function"]
        pub fn new_derive_key(
            &self,
            context: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, Hasher> {
            self.0
                .method_hash([18, 166, 59, 93], context)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `new_hasher` (0x0d056d9b) function"]
        pub fn new_hasher(&self) -> ethers::contract::builders::ContractCall<M, Hasher> {
            self.0
                .method_hash([13, 5, 109, 155], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `new_keyed` (0xc51d2cd7) function"]
        pub fn new_keyed(
            &self,
            key: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, Hasher> {
            self.0
                .method_hash([197, 29, 44, 215], key)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `update_hasher` (0x82158627) function"]
        pub fn update_hasher(
            &self,
            self_: Hasher,
            input: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, Hasher> {
            self.0
                .method_hash([130, 21, 134, 39], (self_, input))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for Blake3Sol<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `finalize`function with signature `finalize(((uint32[8],uint64,bytes,uint32,uint8,uint32),uint32[8],uint32[8][54],uint8,uint32))` and selector `[136, 85, 238, 149]`"]
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
        name = "finalize",
        abi = "finalize(((uint32[8],uint64,bytes,uint32,uint8,uint32),uint32[8],uint32[8][54],uint8,uint32))"
    )]
    pub struct FinalizeCall {
        pub self_: Hasher,
    }
    #[doc = "Container type for all input parameters for the `new_derive_key`function with signature `new_derive_key(bytes)` and selector `[18, 166, 59, 93]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "new_derive_key", abi = "new_derive_key(bytes)")]
    pub struct NewDeriveKeyCall {
        pub context: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `new_hasher`function with signature `new_hasher()` and selector `[13, 5, 109, 155]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "new_hasher", abi = "new_hasher()")]
    pub struct NewHasherCall;
    #[doc = "Container type for all input parameters for the `new_keyed`function with signature `new_keyed(bytes)` and selector `[197, 29, 44, 215]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "new_keyed", abi = "new_keyed(bytes)")]
    pub struct NewKeyedCall {
        pub key: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `update_hasher`function with signature `update_hasher(((uint32[8],uint64,bytes,uint32,uint8,uint32),uint32[8],uint32[8][54],uint8,uint32),bytes)` and selector `[130, 21, 134, 39]`"]
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
        name = "update_hasher",
        abi = "update_hasher(((uint32[8],uint64,bytes,uint32,uint8,uint32),uint32[8],uint32[8][54],uint8,uint32),bytes)"
    )]
    pub struct UpdateHasherCall {
        pub self_: Hasher,
        pub input: ethers::core::types::Bytes,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum Blake3SolCalls {
        Finalize(FinalizeCall),
        NewDeriveKey(NewDeriveKeyCall),
        NewHasher(NewHasherCall),
        NewKeyed(NewKeyedCall),
        UpdateHasher(UpdateHasherCall),
    }
    impl ethers::core::abi::AbiDecode for Blake3SolCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <FinalizeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(Blake3SolCalls::Finalize(decoded));
            }
            if let Ok(decoded) =
                <NewDeriveKeyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(Blake3SolCalls::NewDeriveKey(decoded));
            }
            if let Ok(decoded) =
                <NewHasherCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(Blake3SolCalls::NewHasher(decoded));
            }
            if let Ok(decoded) =
                <NewKeyedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(Blake3SolCalls::NewKeyed(decoded));
            }
            if let Ok(decoded) =
                <UpdateHasherCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(Blake3SolCalls::UpdateHasher(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for Blake3SolCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Blake3SolCalls::Finalize(element) => element.encode(),
                Blake3SolCalls::NewDeriveKey(element) => element.encode(),
                Blake3SolCalls::NewHasher(element) => element.encode(),
                Blake3SolCalls::NewKeyed(element) => element.encode(),
                Blake3SolCalls::UpdateHasher(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for Blake3SolCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                Blake3SolCalls::Finalize(element) => element.fmt(f),
                Blake3SolCalls::NewDeriveKey(element) => element.fmt(f),
                Blake3SolCalls::NewHasher(element) => element.fmt(f),
                Blake3SolCalls::NewKeyed(element) => element.fmt(f),
                Blake3SolCalls::UpdateHasher(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<FinalizeCall> for Blake3SolCalls {
        fn from(var: FinalizeCall) -> Self {
            Blake3SolCalls::Finalize(var)
        }
    }
    impl ::std::convert::From<NewDeriveKeyCall> for Blake3SolCalls {
        fn from(var: NewDeriveKeyCall) -> Self {
            Blake3SolCalls::NewDeriveKey(var)
        }
    }
    impl ::std::convert::From<NewHasherCall> for Blake3SolCalls {
        fn from(var: NewHasherCall) -> Self {
            Blake3SolCalls::NewHasher(var)
        }
    }
    impl ::std::convert::From<NewKeyedCall> for Blake3SolCalls {
        fn from(var: NewKeyedCall) -> Self {
            Blake3SolCalls::NewKeyed(var)
        }
    }
    impl ::std::convert::From<UpdateHasherCall> for Blake3SolCalls {
        fn from(var: UpdateHasherCall) -> Self {
            Blake3SolCalls::UpdateHasher(var)
        }
    }
    #[doc = "`ChunkState(uint32[8],uint64,bytes,uint32,uint8,uint32)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ChunkState {
        pub chaining_value: [u32; 8],
        pub chunk_counter: u64,
        pub block_bytes: ethers::core::types::Bytes,
        pub block_len: u32,
        pub blocks_compressed: u8,
        pub flags: u32,
    }
    #[doc = "`Hasher((uint32[8],uint64,bytes,uint32,uint8,uint32),uint32[8],uint32[8][54],uint8,uint32)`"]
    #[derive(
        Clone,
        Debug,
        //Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct Hasher {
        pub chunk_state: ChunkState,
        pub key_words: [u32; 8],
        pub cv_stack: [[u32; 8]; 54],
        pub cv_stack_len: u8,
        pub flags: u32,
    }

    impl Default for Hasher {
        fn default() -> Hasher {
            Hasher {
                chunk_state: Default::default(),
                key_words: Default::default(),
                cv_stack: [[0; 8]; 54],
                cv_stack_len: Default::default(),
                flags: Default::default(),
            }
        }
    }
}
