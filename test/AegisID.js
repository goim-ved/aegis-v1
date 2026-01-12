const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("AegisID", function () {
    it("Should mint a token and set URI", async function () {
        const [owner, otherAccount] = await ethers.getSigners();
        const AegisID = await ethers.getContractFactory("AegisID");
        const aegisId = await AegisID.deploy();

        await aegisId.mint(owner.address, "http://example.com/token/1");
        expect(await aegisId.tokenURI(0)).to.equal("http://example.com/token/1");
    });

    it("Should prevent transfers (Soulbound)", async function () {
        const [owner, otherAccount] = await ethers.getSigners();
        const AegisID = await ethers.getContractFactory("AegisID");
        const aegisId = await AegisID.deploy();

        await aegisId.mint(owner.address, "http://example.com/token/1"); // Fixed variable reference

        // Attempt transfer
        await expect(
            aegisId.transferFrom(owner.address, otherAccount.address, 0)
        ).to.be.revertedWith("AegisID: Soulbound token cannot be transferred");
    });
});
