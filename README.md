# bridge-utils
Utilities to help with the development of bridge-sol
The program asks the user for the number of leaves they want in the tree (that way you can basically specify how long you want the merkle proofs to be) and it will create that many random Themelio Transaction structs, turn them into hashed leaves, and create a new merkle tree with them. It will then create a random Themelio Header struct and save the tree's merkle root to header.transactions_hash, pick a random leaf, create a merkle proof for it, encode the header to Ethereum calldata format, submit the header to relayHeader() via a seth RPC call, and submit randomly picked transaction to verifyTx() along with all other required data as a seth RPC call.
