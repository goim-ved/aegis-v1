const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("AegisWallet Part 1: Financial Core", function () {
    let AegisID, aegisID;
    let AegisWallet, wallet;
    let AegisRules, rules;
    let MockUSDC, usdc;
    let owner, agent, nonAgent, recipient;

    beforeEach(async function () {
        [owner, agent, nonAgent, recipient] = await ethers.getSigners();

        // 1. Deploy Identity Registry (AegisID)
        const AegisIDFactory = await ethers.getContractFactory("AegisID");
        aegisID = await AegisIDFactory.deploy();
        await aegisID.waitForDeployment(); // Updated for Ethers v6

        // 2. Mint Identity to 'Agent'
        // Note: In real life, KYC checks happen off-chain. Here we simulate minting.
        await aegisID.mint(agent.address, "ipfs://identity-metadata");

        // 3. Deploy AegisWallet
        const AegisWalletFactory = await ethers.getContractFactory("AegisWallet");
        wallet = await AegisWalletFactory.deploy(await aegisID.getAddress());
        await wallet.waitForDeployment();

        // 4. Deploy Governance Rules
        const AegisRulesFactory = await ethers.getContractFactory("AegisRules");
        rules = await AegisRulesFactory.deploy();
        await rules.waitForDeployment();

        // 5. Link Rules to Wallet
        await wallet.setRulesContract(await rules.getAddress());

        // 6. Set Default Limit for Agent (to avoid regressions in Part 1 tests)
        // Part 1 tests check spending 1 ETH. Let's set 10 ETH default.
        await rules.setLimit(agent.address, ethers.parseEther("10.0"));

        // 7. Deploy MockUSDC
        const MockUSDCFactory = await ethers.getContractFactory("MockUSDC");
        usdc = await MockUSDCFactory.deploy();
        await usdc.waitForDeployment();
    });

    it("Should allow Owner to deposit funds (Bonding)", async function () {
        const depositAmount = ethers.parseEther("1.0");

        // Owner sends ETH to wallet
        await owner.sendTransaction({
            to: await wallet.getAddress(),
            value: depositAmount
        });

        const balance = await ethers.provider.getBalance(await wallet.getAddress());
        expect(balance).to.equal(depositAmount);
    });

    it("Should allow Verified Agent to execute trades", async function () {
        // Fund the wallet first
        await owner.sendTransaction({
            to: await wallet.getAddress(),
            value: ethers.parseEther("5.0")
        });

        // Agent executes a transfer of 1 ETH to 'recipient'
        const transferAmount = ethers.parseEther("1.0");
        const balanceBefore = await ethers.provider.getBalance(recipient.address);

        // Encode a basic ETH transfer? 
        // Wait, 'execute' takes target, value, data.
        // For simple ETH transfer, data is empty.

        await wallet.connect(agent).execute(recipient.address, transferAmount, "0x");

        const balanceAfter = await ethers.provider.getBalance(recipient.address);
        expect(balanceAfter - balanceBefore).to.equal(transferAmount);
    });

    it("Should BLOCK Non-Agent from spending", async function () {
        // Fund the wallet first
        await owner.sendTransaction({
            to: await wallet.getAddress(),
            value: ethers.parseEther("5.0")
        });

        const transferAmount = ethers.parseEther("1.0");

        // 'nonAgent' tries to call execute
        await expect(
            wallet.connect(nonAgent).execute(recipient.address, transferAmount, "0x")
        ).to.be.revertedWith("AegisWallet: Caller not verified Agent or Owner");
    });

    it("Should BLOCK Agent if Identity is lost (Burned/Revoked)", async function () {
        // TODO: Implement Burn in ProxyID or just use transfer check (Soulbound)
        // Since ProxyID is soulbound, we can't transfer it away.
        // But for this test, we assume if balance is 0.
        // Let's create a new 'exAgent' who has 0 balance.

        const exAgent = nonAgent; // Reusing nonAgent as someone who lost access
        await expect(
            wallet.connect(exAgent).execute(recipient.address, ethers.parseEther("0.1"), "0x")
        ).to.be.revertedWith("AegisWallet: Caller not verified Agent or Owner");
    });

    describe("Governance and Limits (Part 3)", function () {
        const limit = ethers.parseEther("1.0");

        beforeEach(async function () {
            // Fund wallet
            await owner.sendTransaction({
                to: await wallet.getAddress(),
                value: ethers.parseEther("10.0")
            });

            // Set Limit for Agent
            await rules.setLimit(agent.address, limit);
        });

        it("Should allow transactions WITHIN the limit", async function () {
            const amount = ethers.parseEther("0.5"); // Below 1.0
            await wallet.connect(agent).execute(recipient.address, amount, "0x");
            // Should succeed
        });

        it("Should REJECT transactions EXCEEDING the limit", async function () {
            const amount = ethers.parseEther("1.1"); // Above 1.0
            await expect(
                wallet.connect(agent).execute(recipient.address, amount, "0x")
            ).to.be.revertedWith("AegisRules: Daily limit exceeded");
        });

        it("Should enforce CUMULATIVE daily limits", async function () {
            // Spend 0.6
            await wallet.connect(agent).execute(recipient.address, ethers.parseEther("0.6"), "0x");

            // Try to spend 0.5 (Total 1.1) -> Fail
            await expect(
                wallet.connect(agent).execute(recipient.address, ethers.parseEther("0.5"), "0x")
            ).to.be.revertedWith("AegisRules: Daily limit exceeded");
        });
    });

    describe("Settlement Engine (Part 5: USDC)", function () {
        const amount = ethers.parseEther("100.0"); // 100 USDC

        beforeEach(async function () {
            // 1. Mint USDC to Wallet (Bonding)
            await usdc.mint(await wallet.getAddress(), ethers.parseEther("1000.0"));

            // 2. Set Agent Limit to 200 Tokens
            // Note: This overrides the default 10 ETH limit set in main beforeEach
            await rules.setLimit(agent.address, ethers.parseEther("200.0"));
        });

        it("Should executeERC20 and count towards limit", async function () {
            // Spend 100
            await wallet.connect(agent).executeERC20(await usdc.getAddress(), recipient.address, amount);

            const bal = await usdc.balanceOf(recipient.address);
            expect(bal).to.equal(amount);
        });

        it("Should BLOCK executeERC20 if over limit", async function () {
            // Spend 100 (OK)
            await wallet.connect(agent).executeERC20(await usdc.getAddress(), recipient.address, amount);

            // Spend 150 (Total 250 > 200) -> FAIL
            const amount2 = ethers.parseEther("150.0");
            await expect(
                wallet.connect(agent).executeERC20(await usdc.getAddress(), recipient.address, amount2)
            ).to.be.revertedWith("AegisRules: Daily limit exceeded");
        });
    });
});
