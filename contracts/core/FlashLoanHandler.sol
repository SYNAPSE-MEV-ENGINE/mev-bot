// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@aave/core-v3/contracts/flashloan/base/FlashLoanSimpleReceiverBase.sol";
import "@aave/core-v3/contracts/interfaces/IPoolAddressesProvider.sol";

contract FlashLoanHandler is FlashLoanSimpleReceiverBase, ReentrancyGuard, Ownable {
    address private constant AAVE_POOL_ADDRESSES_PROVIDER = 0xa97684ead0e402dC232d5A977953DF7ECBaB3CDb; // Polygon mainnet

    constructor() FlashLoanSimpleReceiverBase(IPoolAddressesProvider(AAVE_POOL_ADDRESSES_PROVIDER)) {
    }

    function executeOperation(
        address asset,
        uint256 amount,
        uint256 premium,
        address initiator,
        bytes calldata params
    ) external override returns (bool) {
        // Decode params for sandwich execution
        (address target, bytes memory data) = abi.decode(params, (address, bytes));
        
        // Execute sandwich strategy
        (bool success, ) = target.call(data);
        require(success, "Sandwich execution failed");

        // Approve repayment
        uint256 amountToRepay = amount + premium;
        IERC20(asset).approve(address(POOL), amountToRepay);

        return true;
    }

    function requestFlashLoan(
        address asset,
        uint256 amount,
        address target,
        bytes calldata data
    ) external onlyOwner nonReentrant {
        bytes memory params = abi.encode(target, data);
        POOL.flashLoanSimple(
            address(this),
            asset,
            amount,
            params,
            0 // referralCode
        );
    }

    // Emergency withdrawal function
    function emergencyWithdraw(address token) external onlyOwner {
        uint256 balance = IERC20(token).balanceOf(address(this));
        require(balance > 0, "No balance to withdraw");
        IERC20(token).transfer(owner(), balance);
    }
}
