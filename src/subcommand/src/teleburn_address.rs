// SPDX-License-Identifier: MIT
pragma solidity ^0.8.7;

contract EthereumTeleburn {
    
    // Function to generate Ethereum address with checksum
    function createAddressWithChecksum(string memory addr) public pure returns (string memory) {
        bytes memory addrBytes = bytes(addr);
        bytes32 hashedAddr = keccak256(abi.encodePacked(addr));
        
        string memory result = "0x";
        for (uint256 i = 0; i < addrBytes.length; i++) {
            uint8 value = uint8(hashedAddr[i] & 0x0F);
            if (value > 7) {
                result = string(abi.encodePacked(result, toAsciiUpper(addrBytes[i])));
            } else {
                result = string(abi.encodePacked(result, string(abi.encodePacked(addrBytes[i]))));
            }
        }
        
        return result;
    }
    
    // Helper function to convert a byte to its ASCII uppercase representation
    function toAsciiUpper(bytes1 _b) internal pure returns (bytes1) {
        if (_b >= 0x61 && _b <= 0x7A) {
            return bytes1(uint8(_b) - 32);
        }
        return _b;
    }
    
    // Function to convert InscriptionId (represented as bytes) to EthereumTeleburnAddress
    function fromInscriptionIdToTeleburnAddress(bytes memory inscriptionId) public pure returns (string memory) {
        bytes32 digest = keccak256(inscriptionId);
        
        // Taking first 20 bytes and converting to hex
        bytes memory first20Bytes = new bytes(20);
        for (uint i = 0; i < 20; i++) {
            first20Bytes[i] = digest[i];
        }
        
        string memory hexString = bytesToHexString(first20Bytes);
        return createAddressWithChecksum(hexString);
    }
    
    // Helper function to convert bytes to hex string
    function bytesToHexString(bytes memory data) internal pure returns (string memory) {
        bytes memory alphabet = "0123456789abcdef";
        
        bytes memory str = new bytes(data.length * 2);
        for (uint i = 0; i < data.length; i++) {
            str[i*2] = alphabet[uint(uint8(data[i] >> 4))];
            str[1 + i*2] = alphabet[uint(uint8(data[i] & 0x0f))];
        }
        return string(str);
    }
}
