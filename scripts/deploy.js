const hre = require("hardhat");

async function main() {
    console.log("Starting deployment of Aegis Identity System...");
    const AegisID = await hre.ethers.getContractFactory("AegisID");
    const aegisId = await AegisID.deploy();

    await aegisId.waitForDeployment();

    const address = await aegisId.getAddress();
    console.log(`>> Success! AegisID deployed at: ${address}`);
    // Design Note: We might need to auto-verify on Etherscan in prod.
}

main().catch((error) => {
    console.error(error);
    process.exitCode = 1;
});
