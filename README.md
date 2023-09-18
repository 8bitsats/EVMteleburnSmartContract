# EVMteleburnSmartContract
Bitcoin Ordinals EVM Teleburn Smart Contract in Solidity 
The contract uses OpenZeppelin's ERC721 interface for NFT operations
Solidity has built-in support for the Keccak256 hash function, which is similar to the SHA3 hash function used in the original Rust code.
Solidity doesn't have native support for ASCII case manipulation, so a helper function toAsciiUpper is used to convert a byte to its ASCII uppercase representation.
This is a basic example and doesn't cover all aspects of the Rust code you provided, such as tests.
The teleburn function performs two main operations: it burns the NFT (assuming the NFT contract allows for this and complies with the ERC721 standard) and records the "inscription" details.
An event Teleburned is emitted when an NFT is successfully teleburned. This event can be monitored by off-chain services to reflect the operation on Bitcoin ordinals.
The getInscription function allows for querying the inscription details for a specific Ethereum address.
The teleburn function performs two main operations: it burns the NFT (assuming the NFT contract allows for this and complies with the ERC721 standard) and records the "inscription" details.
An event Teleburned is emitted when an NFT is successfully teleburned. This event can be monitored by off-chain services to reflect the operation on Bitcoin ordinals.
The getInscription function allows for querying the inscription details for a specific Ethereum address.
used a struct to capture the details of each inscription, similar to your Rust code.
included a recordInscription function to record the details of the teleburn operation.
also included a getInscription function to fetch these details.
An event Teleburned is emitted whenever an inscription is recorded, which can be used by off-chain services to trigger corresponding actions on Bitcoin.
The concept of Sat and SatPoint are not clear, so they are just represented as uint256 and bytes32 respectively.
The previousInscriptionId is captured, linking inscriptions in a kind of chain for each user.

raph/casey notes
People are interested in burning high-value Ethereum NFTs, so this feature needs to be extremely safe:

 add tests
 review cryptographic assumptions
 generate ethereum address with checksum (no ethereum dependencies for this, must be implemented manually)
 manual end-to-end test (we can just make sure that you can derive genesis inscription ID from the ethereum address)
 make sure that the index is synced and that the inscription that you're burning is in your wallet
 add ability to reverse teleburn: get the inscription id from the teleburn address
 show teleburn address on inscription page
 only accept ethereum addresses with checksum when deserializing
