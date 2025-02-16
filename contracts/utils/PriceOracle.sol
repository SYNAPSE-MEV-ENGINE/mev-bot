// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@chainlink/contracts/src/v0.8/interfaces/AggregatorV3Interface.sol";

contract PriceOracle is Ownable {
    mapping(address => address) public priceFeeds;
    mapping(address => uint256) public lastUpdated;
    uint256 public constant PRICE_FRESHNESS_THRESHOLD = 3600; // 1 hour

    event PriceFeedUpdated(address token, address feed);
    event PriceUpdated(address token, uint256 price);

    constructor() {
        // Initialize with some common Polygon price feeds
        priceFeeds[0x0d500B1d8E8eF31E21C99d1Db9A6444d3ADf1270] = 0xAB594600376Ec9fD91F8e885dADF0CE036862dE0; // MATIC/USD
        priceFeeds[0x7ceB23fD6bC0adD59E62ac25578270cFf1b9f619] = 0xF9680D99D6C9589e2a93a78A04A279e509205945; // WETH/USD
    }

    function updatePriceFeed(address token, address feed) external onlyOwner {
        require(feed != address(0), "Invalid feed address");
        priceFeeds[token] = feed;
        emit PriceFeedUpdated(token, feed);
    }

    function getPrice(address token0, address token1) external view returns (uint256) {
        require(priceFeeds[token0] != address(0), "Price feed not found for token0");
        require(priceFeeds[token1] != address(0), "Price feed not found for token1");

        uint256 price0 = getChainlinkPrice(priceFeeds[token0]);
        uint256 price1 = getChainlinkPrice(priceFeeds[token1]);

        return (price0 * 1e18) / price1;
    }

    function getChainlinkPrice(address feed) internal view returns (uint256) {
        AggregatorV3Interface priceFeed = AggregatorV3Interface(feed);
        (
            uint80 roundID,
            int256 price,
            uint256 startedAt,
            uint256 timeStamp,
            uint80 answeredInRound
        ) = priceFeed.latestRoundData();

        require(timeStamp != 0, "Round not complete");
        require(price > 0, "Invalid price");
        require(
            block.timestamp - timeStamp < PRICE_FRESHNESS_THRESHOLD,
            "Stale price"
        );

        return uint256(price);
    }

    function simulateTradeImpact(
        address token0,
        address token1,
        uint256 amount,
        bytes calldata tradeData
    ) external view returns (uint256) {
        // Implement trade impact simulation
        // This could involve querying DEX contracts or using historical data
        uint256 basePrice = this.getPrice(token0, token1);
        
        // Simple impact model - can be enhanced based on liquidity depth
        uint256 impact = (amount * 100) / 1e18; // 0.01% impact per unit
        return basePrice * (10000 - impact) / 10000;
    }
}
