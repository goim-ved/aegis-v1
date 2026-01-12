// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/token/ERC721/extensions/ERC721URIStorage.sol";
import "@openzeppelin/contracts/access/AccessControl.sol";

/// @title AegisID (Soulbound Identity)
/// @author Aegis Protocol Team
/// @notice Represents a non-transferable identity verification badge.
/// @dev Implementation uses OpenZeppelin's ERC721 but overrides `_update` to prevent transfers.
contract AegisID is ERC721, ERC721URIStorage, AccessControl {
    bytes32 public constant MINTER_ROLE = keccak256("MINTER_ROLE");
    uint256 private _nextTokenId;

    constructor() ERC721("AegisID", "AEGIS") {
        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
        _grantRole(MINTER_ROLE, msg.sender);
    }

    /// @notice Issues a new Identity Token to a verified entity.
    /// @dev Only callable by accounts with MINTER_ROLE (usually the Compliance Backend).
    /// @param to The address of the verified entity.
    /// @param uri IPFS or Off-chain URL containing the non-sensitive metadata (e.g. "KYC Level 2").
    function mint(address to, string memory uri) public onlyRole(MINTER_ROLE) {
        uint256 tokenId = _nextTokenId++;
        _safeMint(to, tokenId);
        _setTokenURI(tokenId, uri);
    }

    // Hook: Validates that the token is being minted or burned, acting as a "Soulbound" guard.
    // We strictly forbid transfers between two non-zero addresses.
    function _update(address to, uint256 tokenId, address auth)
        internal
        override(ERC721)
        returns (address)
    {
        address from = _ownerOf(tokenId);
        // Allow minting (from == 0) and burning (to == 0), but disallow transfers (from != 0 && to != 0)
        require(from == address(0) || to == address(0), "AegisID: Soulbound token cannot be transferred");
        return super._update(to, tokenId, auth);
    }

    function tokenURI(uint256 tokenId)
        public
        view
        override(ERC721, ERC721URIStorage)
        returns (string memory)
    {
        return super.tokenURI(tokenId);
    }

    function supportsInterface(bytes4 interfaceId)
        public
        view
        override(ERC721, ERC721URIStorage, AccessControl)
        returns (bool)
    {
        return super.supportsInterface(interfaceId);
    }
}
