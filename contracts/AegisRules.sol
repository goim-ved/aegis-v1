// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/access/Ownable.sol";
import "./interfaces/IAegisRules.sol";

contract AegisRules is Ownable, IAegisRules {
    struct AgentRule {
        uint256 dailyLimit;
        uint256 spentToday;
        uint256 lastResetTime;
    }

    mapping(address => AgentRule) public agentRules;

    event LimitSet(address indexed agent, uint256 limit);
    event TransactionChecked(address indexed agent, uint256 amount, uint256 remaining);

    constructor() Ownable(msg.sender) {}

    function setLimit(address agent, uint256 limit) external onlyOwner {
        agentRules[agent].dailyLimit = limit;
        // Don't reset spentToday automatically? Or maybe we should?
        // For now, simple update.
        emit LimitSet(agent, limit);
    }

    function checkTransaction(address agent, uint256 amount) external override returns (bool) {
        AgentRule storage rule = agentRules[agent];
        
        // 1. Check if day has passed (24h rolling or midnight reset)
        // Simple 24h checks from last reset
        if (block.timestamp >= rule.lastResetTime + 1 days) {
            rule.spentToday = 0;
            rule.lastResetTime = block.timestamp;
        }

        // 2. Check limits
        require(rule.spentToday + amount <= rule.dailyLimit, "AegisRules: Daily limit exceeded");

        // 3. Update State
        rule.spentToday += amount;
        
        emit TransactionChecked(agent, amount, rule.dailyLimit - rule.spentToday);
        return true;
    }
}
