const { expect } = require("chai");
const { ethers } = require("hardhat");
const {
  deployMockToken,
  deployMockDEX,
  setupDEXLiquidity,
  createMockVictimTx,
  getCurrentGasPrice,
  calculateOptimalGasPrice
} = require("./utils/test-utils");

describe("SandwichExecutor Integration Tests", function () {
  let sandwichExecutor;
  let flashLoanHandler;
  let priceOracle;
  let mockDEX;
  let mockAavePool;
  let token0;
  let token1;
  let owner;
  let addr1;
  let addr2;

  // 1 ether = 1e18 wei
  const ONE_ETHER = "1000000000000000000";
  const INITIAL_LIQUIDITY = "1000000000000000000000000"; // 1M tokens
  const VICTIM_SWAP_AMOUNT = "1000000000000000000000"; // 1000 tokens

  beforeEach(async function () {
    [owner, addr1, addr2] = await ethers.getSigners();

    // Deploy mock tokens
    token0 = await deployMockToken("Token0", "TK0");
    token1 = await deployMockToken("Token1", "TK1");

    // Deploy mock DEX
    mockDEX = await deployMockDEX();

    // Deploy mock Aave pool
    const MockAavePool = await ethers.getContractFactory("MockAavePool");
    mockAavePool = await MockAavePool.deploy();
    await mockAavePool.waitForDeployment();

    // Setup initial liquidity
    await setupDEXLiquidity(mockDEX, token0, token1, INITIAL_LIQUIDITY, INITIAL_LIQUIDITY);

    // Deploy core contracts
    const PriceOracle = await ethers.getContractFactory("PriceOracle");
    priceOracle = await PriceOracle.deploy();
    await priceOracle.waitForDeployment();

    const FlashLoanHandler = await ethers.getContractFactory("FlashLoanHandler");
    flashLoanHandler = await FlashLoanHandler.deploy(await mockAavePool.getAddress());
    await flashLoanHandler.waitForDeployment();

    const SandwichExecutor = await ethers.getContractFactory("SandwichExecutor");
    sandwichExecutor = await SandwichExecutor.deploy(
      await flashLoanHandler.getAddress(),
      await priceOracle.getAddress()
    );
    await sandwichExecutor.waitForDeployment();

    // Setup price feeds
    await priceOracle.setPriceFeed(await token0.getAddress(), await addr1.getAddress()); // Mock price feed address
    await priceOracle.setPriceFeed(await token1.getAddress(), await addr2.getAddress()); // Mock price feed address

    // Fund the mock Aave pool with tokens for flash loans
    await token0.mint(await mockAavePool.getAddress(), INITIAL_LIQUIDITY);
    await token1.mint(await mockAavePool.getAddress(), INITIAL_LIQUIDITY);
  });

  describe("Sandwich Attack Simulation", function () {
    it("Should execute a profitable sandwich attack", async function () {
      // Create victim transaction
      const victimTx = await createMockVictimTx(mockDEX, token0, token1, VICTIM_SWAP_AMOUNT);

      // Calculate optimal gas price
      const baseGasPrice = await getCurrentGasPrice();
      const optimalGasPrice = await calculateOptimalGasPrice(baseGasPrice);

      // Get initial balances
      const initialBalance0 = await token0.balanceOf(await sandwichExecutor.getAddress());
      const initialBalance1 = await token1.balanceOf(await sandwichExecutor.getAddress());

      // Execute sandwich
      await sandwichExecutor.executeSandwich(
        await token0.getAddress(),
        await token1.getAddress(),
        ethers.BigNumber.from(VICTIM_SWAP_AMOUNT).mul(2), // Frontrun amount
        optimalGasPrice,
        victimTx,
        { gasPrice: optimalGasPrice }
      );

      // Get final balances
      const finalBalance0 = await token0.balanceOf(await sandwichExecutor.getAddress());
      const finalBalance1 = await token1.balanceOf(await sandwichExecutor.getAddress());

      // Verify profit
      expect(finalBalance0.sub(initialBalance0).add(finalBalance1.sub(initialBalance1)))
        .to.be.gt(0, "Sandwich attack should be profitable");
    });

    it("Should revert when price impact is too high", async function () {
      const victimTx = await createMockVictimTx(mockDEX, token0, token1, VICTIM_SWAP_AMOUNT);
      const baseGasPrice = await getCurrentGasPrice();
      const optimalGasPrice = await calculateOptimalGasPrice(baseGasPrice);

      // Try to execute with too large frontrun amount
      await expect(
        sandwichExecutor.executeSandwich(
          await token0.getAddress(),
          await token1.getAddress(),
          ethers.BigNumber.from(VICTIM_SWAP_AMOUNT).mul(10), // Too large frontrun amount
          optimalGasPrice,
          victimTx
        )
      ).to.be.revertedWith("Price impact too high");
    });

    it("Should handle multiple DEX interactions correctly", async function () {
      // Setup additional DEX with different liquidity
      const mockDEX2 = await deployMockDEX();
      await setupDEXLiquidity(
        mockDEX2,
        token0,
        token1,
        ethers.BigNumber.from(INITIAL_LIQUIDITY).div(2),
        ethers.BigNumber.from(INITIAL_LIQUIDITY).div(2)
      );

      const victimTx = await createMockVictimTx(mockDEX, token0, token1, VICTIM_SWAP_AMOUNT);
      const baseGasPrice = await getCurrentGasPrice();
      const optimalGasPrice = await calculateOptimalGasPrice(baseGasPrice);

      // Execute sandwich across both DEXes
      const tx = await sandwichExecutor.executeSandwich(
        await token0.getAddress(),
        await token1.getAddress(),
        VICTIM_SWAP_AMOUNT,
        optimalGasPrice,
        victimTx
      );

      await expect(tx).to.not.be.reverted;
    });
  });

  describe("Risk Management", function () {
    it("Should enforce minimum profit threshold", async function () {
      const smallAmount = "100000000000000000"; // 0.1 tokens
      const victimTx = await createMockVictimTx(mockDEX, token0, token1, smallAmount);
      const baseGasPrice = await getCurrentGasPrice();
      const optimalGasPrice = await calculateOptimalGasPrice(baseGasPrice);

      await expect(
        sandwichExecutor.executeSandwich(
          await token0.getAddress(),
          await token1.getAddress(),
          smallAmount,
          optimalGasPrice,
          victimTx
        )
      ).to.be.revertedWith("Insufficient profit");
    });

    it("Should handle failed transactions gracefully", async function () {
      const victimTx = "0x"; // Invalid transaction data
      const baseGasPrice = await getCurrentGasPrice();
      const optimalGasPrice = await calculateOptimalGasPrice(baseGasPrice);

      await expect(
        sandwichExecutor.executeSandwich(
          await token0.getAddress(),
          await token1.getAddress(),
          VICTIM_SWAP_AMOUNT,
          optimalGasPrice,
          victimTx
        )
      ).to.be.reverted;
    });
  });
});
