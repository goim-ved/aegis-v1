const hre = require("hardhat");

async function main() {
    console.log("Starting Part 1 Deployment...");

    // 1. Deploy AegisID (Identity Registry)
    const aegisID = await hre.ethers.deployContract("AegisID");
    await aegisID.waitForDeployment();
    const aegisIDAddr = await aegisID.getAddress();
    console.log(`AegisID deployed to: ${aegisIDAddr}`);

    // 2. Deploy AegisWallet (Financial Core)
    const aegisWallet = await hre.ethers.deployContract("AegisWallet", [aegisIDAddr]);
    await aegisWallet.waitForDeployment();
    console.log(`AegisWallet deployed to: ${await aegisWallet.getAddress()}`);

    console.log("Deployment Complete!");
}

main().catch((error) => {
    console.error(error);
    process.exitCode = 1;
});
