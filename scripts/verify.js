const hre = require("hardhat");

async function verifyContract(address, constructorArguments = []) {
  try {
    await hre.run("verify:verify", {
      address: address,
      constructorArguments: constructorArguments,
    });
    console.log("Contract verified successfully");
  } catch (error) {
    console.error("Error verifying contract:", error.message);
  }
}

async function main() {
  // Get contract addresses from command line arguments
  const [, , contract, address, ...args] = process.argv;

  if (!contract || !address) {
    console.error("Please provide contract name and address");
    console.log("Usage: npx hardhat run scripts/verify.js --network polygon <CONTRACT_NAME> <CONTRACT_ADDRESS> [CONSTRUCTOR_ARGS...]");
    process.exit(1);
  }

  console.log(`Verifying ${contract} at ${address} with args:`, args);
  await verifyContract(address, args);
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
