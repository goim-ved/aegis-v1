// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/**
 * @dev Interface for the AegisID Identity Registry.
 * Required for AegisWallet to verify if a caller holds a valid identity.
 */
interface IAegisID {
    /**
     * @dev Returns the owner of the `tokenId` token.
     * Requirements:
     * - `tokenId` must exist.
     */
    function ownerOf(uint256 tokenId) external view returns (address owner);

    /**
     * @dev Returns the number of tokens in `owner`'s account.
     */
    function balanceOf(address owner) external view returns (uint256 balance);
}
