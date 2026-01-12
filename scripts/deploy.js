const hre = require("hardhat");

async function main() {
    const AegisID = await hre.ethers.getContractFactory("AegisID");
    const aegisId = await AegisID.deploy();

    await aegisId.waitForDeployment();

    const address = await aegisId.getAddress();
    console.log(`AegisID deployed to: ${address}`);
}

main().catch((error) => {
    console.error(error);
    process.exitCode = 1;
});
