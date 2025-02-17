const { ethers } = require("hardhat");

// Mock ERC20 token for testing
async function deployMockToken(name, symbol) {
  const MockToken = await ethers.getContractFactory("MockERC20");
  const token = await MockToken.deploy(name, symbol);
  await token.waitForDeployment();
  return token;
}

// Mock DEX for testing sandwich attacks
async function deployMockDEX() {
  const MockDEX = await ethers.getContractFactory("MockDEX");
  const dex = await MockDEX.deploy();
  await dex.waitForDeployment();
  return dex;
}

// Setup liquidity in the mock DEX
async function setupDEXLiquidity(dex, token0, token1, amount0, amount1) {
  const dexAddress = await dex.getAddress();
  const token0Address = await token0.getAddress();
  const token1Address = await token1.getAddress();

  const approveTx0 = await token0.approve(dexAddress, amount0);
  await approveTx0.wait();
  
  const approveTx1 = await token1.approve(dexAddress, amount1);
  await approveTx1.wait();
  
  const addLiquidityTx = await dex.addLiquidity(token0Address, token1Address, amount0, amount1);
  await addLiquidityTx.wait();
}

// Create a mock victim transaction
async function createMockVictimTx(dex, token0, token1, amount) {
  const token0Address = await token0.getAddress();
  const token1Address = await token1.getAddress();

  return dex.interface.encodeFunctionData("swapExactTokensForTokens", [
    amount,
    0, // Min amount out
    [token0Address, token1Address],
    ethers.constants.AddressZero,
    Math.floor(Date.now() / 1000) + 60 * 10 // 10 minutes deadline
  ]);
}

// Simulate chain state
async function mineBlocks(count) {
  for (let i = 0; i < count; i++) {
    await ethers.provider.send("evm_mine");
  }
}

// Get current gas price
async function getCurrentGasPrice() {
  const block = await ethers.provider.getBlock("latest");
  return block.baseFeePerGas || await ethers.provider.getGasPrice();
}

// Calculate optimal gas price for MEV transaction
async function calculateOptimalGasPrice(baseGasPrice) {
  // Add 20% premium for MEV transactions
  return baseGasPrice.mul(120).div(100);
}

module.exports = {
  deployMockToken,
  deployMockDEX,
  setupDEXLiquidity,
  createMockVictimTx,
  mineBlocks,
  getCurrentGasPrice,
  calculateOptimalGasPrice
};
