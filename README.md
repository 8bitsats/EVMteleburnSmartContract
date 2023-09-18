# EVMteleburnSmartContract
Bitcoin Ordinals EVM Teleburn Smart Contract in Solidity 
The teleburn function performs two main operations: it burns the NFT (assuming the NFT contract allows for this and complies with the ERC721 standard) and records the "inscription" details.
An event Teleburned is emitted when an NFT is successfully teleburned. This event can be monitored by off-chain services to reflect the operation on Bitcoin ordinals.
The getInscription function allows for querying the inscription details for a specific Ethereum address.
