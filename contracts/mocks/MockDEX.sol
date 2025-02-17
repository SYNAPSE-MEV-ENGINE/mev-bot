// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";

contract MockDEX is ReentrancyGuard {
    mapping(address => mapping(address => uint256)) public reserves;
    uint256 private constant PRECISION = 1e18;

    event LiquidityAdded(address token0, address token1, uint256 amount0, uint256 amount1);
    event Swap(address tokenIn, address tokenOut, uint256 amountIn, uint256 amountOut);

    function addLiquidity(
        address token0,
        address token1,
        uint256 amount0,
        uint256 amount1
    ) external nonReentrant {
        require(token0 != address(0) && token1 != address(0), "Invalid tokens");
        require(amount0 > 0 && amount1 > 0, "Invalid amounts");

        IERC20(token0).transferFrom(msg.sender, address(this), amount0);
        IERC20(token1).transferFrom(msg.sender, address(this), amount1);

        reserves[token0][token1] += amount0;
        reserves[token1][token0] += amount1;

        emit LiquidityAdded(token0, token1, amount0, amount1);
    }

    function swapExactTokensForTokens(
        uint256 amountIn,
        uint256 amountOutMin,
        address[] calldata path,
        address to,
        uint256 deadline
    ) external nonReentrant returns (uint256[] memory amounts) {
        require(deadline >= block.timestamp, "Expired");
        require(path.length >= 2, "Invalid path");

        amounts = new uint256[](path.length);
        amounts[0] = amountIn;

        for (uint256 i = 0; i < path.length - 1; i++) {
            amounts[i + 1] = calculateAmountOut(
                amounts[i],
                reserves[path[i]][path[i + 1]],
                reserves[path[i + 1]][path[i]]
            );
        }

        require(amounts[amounts.length - 1] >= amountOutMin, "Insufficient output");

        // Transfer tokens
        IERC20(path[0]).transferFrom(msg.sender, address(this), amounts[0]);
        IERC20(path[path.length - 1]).transfer(to, amounts[amounts.length - 1]);

        // Update reserves
        for (uint256 i = 0; i < path.length - 1; i++) {
            reserves[path[i]][path[i + 1]] += amounts[i];
            reserves[path[i + 1]][path[i]] -= amounts[i + 1];
        }

        emit Swap(path[0], path[path.length - 1], amounts[0], amounts[amounts.length - 1]);
        return amounts;
    }

    function calculateAmountOut(
        uint256 amountIn,
        uint256 reserveIn,
        uint256 reserveOut
    ) public pure returns (uint256) {
        require(amountIn > 0, "Insufficient input");
        require(reserveIn > 0 && reserveOut > 0, "Insufficient liquidity");

        uint256 amountInWithFee = amountIn * 997; // 0.3% fee
        uint256 numerator = amountInWithFee * reserveOut;
        uint256 denominator = (reserveIn * 1000) + amountInWithFee;

        return numerator / denominator;
    }

    function getReserves(address token0, address token1) external view returns (uint256, uint256) {
        return (reserves[token0][token1], reserves[token1][token0]);
    }
}
