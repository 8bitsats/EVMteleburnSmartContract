// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC721/IERC721.sol";

contract Teleburn {
    // Events
    event Teleburned(address indexed sender, uint256 tokenId, bytes32 inscriptionId, address nftAddress);

    // Struct to hold InscriptionId and NFT details
    struct Inscription {
        bytes32 inscriptionId;
        address nftAddress;
        uint256 tokenId;
    }

    // Mapping from Ethereum address to inscription details
    mapping(address => Inscription) public inscriptions;

    /**
     * @dev Teleburn an NFT: Burn on Ethereum and record the event for off-chain Bitcoin ordinal action
     * @param inscriptionId An ID to inscribe the teleburn operation
     * @param nftAddress The address of the NFT contract
     * @param tokenId The ID of the token to be teleburned
     */
    function teleburn(bytes32 inscriptionId, address nftAddress, uint256 tokenId) external {
        // Burn the NFT. Assume the NFT contract complies with ERC721.
        IERC721 nftContract = IERC721(nftAddress);
        require(nftContract.ownerOf(tokenId) == msg.sender, "You must own the NFT");
        nftContract.burn(tokenId);

        // Record the inscription details
        inscriptions[msg.sender] = Inscription({
            inscriptionId: inscriptionId,
            nftAddress: nftAddress,
            tokenId: tokenId
        });

        // Emit the teleburn event
        emit Teleburned(msg.sender, tokenId, inscriptionId, nftAddress);
    }

    /**
     * @dev Retrieve the inscription details for a specific address
     * @param user The address of the user
     */
    function getInscription(address user) external view returns (bytes32, address, uint256) {
        Inscription memory inscription = inscriptions[user];
        return (inscription.inscriptionId, inscription.nftAddress, inscription.tokenId);
    }
}
