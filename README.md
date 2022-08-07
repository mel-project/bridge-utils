# bridge-utils
Utilities to help with the development of bridge-sol

This program creates two random Themelio headers, a verifier which is submitted as the genesis
header (which is sent as an argument to the constructor) and a header to be verified, which is
signed by a group of random stakedocs which have been staked during its epoch. The stakes are then
hashed and become the genesis header's stakes hash, which can then be used to verify the second
header. It also creates a dense Merkle tree of Themelio transactions and saves the Merkle root as
the transactions hash of the second header, for use in transaction verification. A random
transaction from the tree is picked to be verified, and is formatted with the Ethereum recipient of
the Themelio assets which are to be bridged. Once the transaction is verified, ERC-1155 tokens are
minted to the Ethereum recipient. The final step in the bridge life cycle is burning the tokens,
which emits an event which will be used as a receipt to thaw the frozen assets on Themelio (the
Themelio recipient is specified as an argument in the burn() and burnBatch() functions). This CLI
program takes 2 flags, `--num-stakedocs` and `--num-transactions` (indirectly specifying the
desired merkle proof length), and uses that many random Themelio stakedoc and transaction structs
in its operations.

Each time it is run a new ThemelioProxy contract will be created, but you can use environment
variables to specify the addresses of predeployed Ed25519, Blake3, and ThemelioBridge contracts
to avoid re-deploying those every time.

Use the env.example file to see the environment variables required to run the program.