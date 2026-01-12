// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/**
 * @dev Interface for Governance Rules.
 * Determine if a transaction is allowed based on policy.
 */
interface IAegisRules {
    /**
     * @dev checks if a transaction is allowed for a given agent.
     * Returns true if allowed.
     * Reverts or returns false if denied.
     */
    function checkTransaction(address agent, uint256 amount) external returns (bool);
}
