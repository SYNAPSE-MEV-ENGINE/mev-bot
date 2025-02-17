const hre = require("hardhat");

async function main() {
  console.log("Deploying contracts...");

  // Deploy PriceOracle
  const PriceOracle = await hre.ethers.getContractFactory("PriceOracle");
  const priceOracle = await PriceOracle.deploy();
  await priceOracle.deployed();
  console.log("PriceOracle deployed to:", priceOracle.address);

  // Deploy FlashLoanHandler
  const FlashLoanHandler = await hre.ethers.getContractFactory("FlashLoanHandler");
  const flashLoanHandler = await FlashLoanHandler.deploy();
  await flashLoanHandler.deployed();
  console.log("FlashLoanHandler deployed to:", flashLoanHandler.address);

  // Deploy SandwichExecutor
  const SandwichExecutor = await hre.ethers.getContractFactory("SandwichExecutor");
  const sandwichExecutor = await SandwichExecutor.deploy(
    flashLoanHandler.address,
    priceOracle.address
  );
  await sandwichExecutor.deployed();
  console.log("SandwichExecutor deployed to:", sandwichExecutor.address);

  // Wait for block confirmations for verification
  console.log("Waiting for block confirmations...");
  await priceOracle.deployTransaction.wait(6);
  await flashLoanHandler.deployTransaction.wait(6);
  await sandwichExecutor.deployTransaction.wait(6);

  // Verify contracts
  console.log("Verifying contracts...");
  
  try {
    await hre.run("verify:verify", {
      address: priceOracle.address,
      constructorArguments: [],
    });
  } catch (error) {
    console.error("Error verifying PriceOracle:", error.message);
  }

  try {
    await hre.run("verify:verify", {
      address: flashLoanHandler.address,
      constructorArguments: [],
    });
  } catch (error) {
    console.error("Error verifying FlashLoanHandler:", error.message);
  }

  try {
    await hre.run("verify:verify", {
      address: sandwichExecutor.address,
      constructorArguments: [flashLoanHandler.address, priceOracle.address],
    });
  } catch (error) {
    console.error("Error verifying SandwichExecutor:", error.message);
  }

  console.log("Deployment and verification completed!");
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
