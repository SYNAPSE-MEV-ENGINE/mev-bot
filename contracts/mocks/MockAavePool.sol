// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@aave/core-v3/contracts/interfaces/IPool.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";

abstract contract MockAavePool is IPool {
    function supply(
        address asset,
        uint256 amount,
        address onBehalfOf,
        uint16 referralCode
    ) external override {}

    function withdraw(
        address asset,
        uint256 amount,
        address to
    ) external override returns (uint256) {
        return amount;
    }

    function borrow(
        address asset,
        uint256 amount,
        uint256 interestRateMode,
        uint16 referralCode,
        address onBehalfOf
    ) external override {}

    function repay(
        address asset,
        uint256 amount,
        uint256 interestRateMode,
        address onBehalfOf
    ) external override returns (uint256) {
        return amount;
    }

    function flashLoanSimple(
        address receiverAddress,
        address asset,
        uint256 amount,
        uint16 referralCode
    ) external override {}

    function backUnbacked(address, uint256, uint256) external override returns (uint256) {}

    // Removed override from these functions
    function rebalanceStableBorrowRateWithFee(address, address, address, uint256) external {}  
    function liquidationCallWithFee(address, address, address, uint256, bool, address, uint256) external {}  
    function borrowWithPermit(address, uint256, uint256, uint16, address, uint256, uint8, bytes32, bytes32) external {}  

    function getFee(bool) external view returns (uint256) {
        return 0;
    }

    function ADDRESSES_PROVIDER() external view override returns (IPoolAddressesProvider) {
        return IPoolAddressesProvider(address(0));
    }

    // Implement other required functions or leave them unimplemented if abstract
    function configureEModeCategory(uint8 id, DataTypes.EModeCategory memory config) external override {}
    function deposit(address asset, uint256 amount, address onBehalfOf, uint16 referralCode) external override {}
    function dropReserve(address asset) external override {}
    function finalizeTransfer(address asset, address to, uint256 amount, address originator) external override {} 
    function getReserveAddressById(uint16 id) external view override returns (address) {}
    function getUserAccountData(address user) external view override returns (uint256, uint256, uint256, uint256, uint256) {} 
    function initReserve(address asset, uint256 amount, uint256 decimals, address interestRateStrategyAddress) external override {} 
    function rescueTokens(address token, address to, uint256 amount) external override {}
    function resetIsolationModeTotalDebt(address asset) external override {}
    function setConfiguration(address asset, uint256 configuration) external override {} 
    function setReserveInterestRateStrategyAddress(address asset, address rateStrategyAddress) external override {}
    function updateBridgeProtocolFee(uint256 bridgeProtocolFee) external override {}
    function updateFlashloanPremiums(uint256 flashloanPremiums) external override {} 
}
