# bridge-utils
Utilities to help with the development of bridge-sol

This program asks the user for the number of leaves they want in their tree (indirectly specifying the desired merkle proof length) and creates that many random Themelio Transaction structs, turns them into hashed leaves, and creates a new merkle tree with them. It then creates a random Themelio Header struct and saves the tree's merkle root to header.transactions_hash, picks a random datablock, creates a merkle proof for it, encodes the header into Ethereum calldata format, submits the header to relayHeader() via a seth RPC call, and submits the randomly picked transaction to verifyTx() along with all other required data as a seth RPC call.
