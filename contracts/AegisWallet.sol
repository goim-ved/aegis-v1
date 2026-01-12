// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/ReentrancyGuard.sol";
import "./interfaces/IAegisID.sol";
import "./interfaces/IAegisRules.sol";

/**
 * @title AegisWallet
 * @dev A smart account that holds funds and restricts spending to verified Identity holders.
 * Part 1 of AEGIS Infrastructure.
 */
contract AegisWallet is Ownable, ReentrancyGuard {
    IAegisID public immutable aegisID;
    IAegisRules public rulesContract;

    event Executed(address indexed target, uint256 value, bytes data);
    event Received(address indexed sender, uint256 amount);
    event RulesUpdated(address indexed rules);

    constructor(address _aegisID) Ownable(msg.sender) {
        require(_aegisID != address(0), "Invalid AegisID address");
        aegisID = IAegisID(_aegisID);
    }

    /**
     * @dev Receive ETH (bonding/funding)
     */
    receive() external payable {
        emit Received(msg.sender, msg.value);
    }

    /**
     * @dev Execute a transaction (Transfer ETH, Tokens, or call other contracts).
     * @param target The address to call.
     * @param value The amount of ETH to send.
     * @param data The data to execute.
     * 
     * Requirement: Caller must hold at least 1 AegisID token (Proof of Compliance).
     */
    function execute(address target, uint256 value, bytes calldata data) external payable nonReentrant {
        // 1. Identity Check
        require(aegisID.balanceOf(msg.sender) > 0 || msg.sender == owner(), "AegisWallet: Caller not verified Agent or Owner");

        // 2. Budget Check
        if (address(rulesContract) != address(0)) {
            require(rulesContract.checkTransaction(msg.sender, value), "AegisWallet: Rule denied transaction");
        }
        
        // 3. Execution
        (bool success, ) = target.call{value: value}(data);
        require(success, "AegisWallet: Execution failed");

        emit Executed(target, value, data);
    }

    /**
     * @dev Execute ERC20 Transfer with Rule Checks
     * Prevents Governance Loophole where users could drain tokens without ETH value checks.
     */
    function executeERC20(address token, address to, uint256 amount) external nonReentrant {
        // 1. Identity Check
        require(aegisID.balanceOf(msg.sender) > 0 || msg.sender == owner(), "AegisWallet: Caller not verified Agent or Owner");

        // 2. Budget Check (Count token amount as spend)
        // Note: For production, we need an Oracle to convert Token->USD for unified limits.
        // For MVP, we assume 1:1 parity or separate limits per token?
        // Let's assume we treat the amount raw for now (Simple MVP Rule).
        if (address(rulesContract) != address(0)) {
            require(rulesContract.checkTransaction(msg.sender, amount), "AegisWallet: Rule denied transaction");
        }

        // 3. Execution
        IERC20(token).transfer(to, amount);
        emit Executed(token, 0, abi.encodeWithSignature("transfer(address,uint256)", to, amount));
    }

    /**
     * @dev Withdraw ERC20 tokens (Emergency or De-bonding)
     */
    function withdrawToken(address token, uint256 amount) external onlyOwner nonReentrant {
        IERC20(token).transfer(msg.sender, amount);
    }

    function setRulesContract(address _rules) external onlyOwner {
        rulesContract = IAegisRules(_rules);
        emit RulesUpdated(_rules);
    }
}
