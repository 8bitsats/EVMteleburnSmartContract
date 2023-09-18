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
