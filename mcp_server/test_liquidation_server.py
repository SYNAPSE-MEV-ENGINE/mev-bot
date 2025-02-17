# Mock Aave Protocol
class MockAaveProtocol:
    def __init__(self):
        pass

    def liquidation_call(self, collateral_asset, debt_asset, total_debt_base, user):
        return "mocked_tx_hash"  # Mocked transaction hash for testing

# Replace the import in LiquidationServer with the mock

def mock_aave_import():
    global AaveProtocol
    AaveProtocol = MockAaveProtocol

mock_aave_import()

# Mock the Chainlink import
class MockChainlinkPriceFeed:
    pass

def mock_chainlink_import():
    global ChainlinkPriceFeed
    ChainlinkPriceFeed = MockChainlinkPriceFeed

mock_chainlink_import()

import pytest
import asyncio
from unittest.mock import MagicMock
from web3 import Web3

from liquidation_server import LiquidationServer
from supabase import create_client, Client

# Mock Supabase client for testing
class MockSupabase:
    def __init__(self):
        self.data = {}

    def table(self, name):
        return self

    def update(self, data):
        self.data.update(data)
        return self

    def eq(self, key, value):
        return self

    def execute(self):
        return {'status_code': 200, 'data': self.data}

# Initialize mock Supabase client
mock_supabase = MockSupabase()

@pytest.fixture
def liquidation_server():
    w3 = Web3(Web3.HTTPProvider('https://polygon-rpc.com'))  # Using a valid Polygon RPC URL
    return LiquidationServer(w3)

@pytest.fixture
def mock_aave_protocol():
    return MockAaveProtocol()

def test_update_user_profit(liquidation_server):
    # Arrange
    user_address = '0xUserAddress'
    profit_amount = 100
    mock_supabase.data = {
        '0xUserAddress': {'profit': 0, 'balance': 0},
        'user_123': {'profit': 0, 'balance': 0}
    }  # Initialize mock data for tests

    # Act
    result = liquidation_server.update_user_profit(user_address, profit_amount)

    # Assert
    assert result is not None
    assert mock_supabase.data[user_address]['profit'] == profit_amount  # Check if profit was updated

def test_liquidation_call(mock_aave_protocol):
    # Arrange
    collateral_asset = '0xCollateralAsset'
    debt_asset = '0xDebtAsset'
    total_debt_base = 1000
    user = '0xUserAddress'

    # Act
    tx_hash = mock_aave_protocol.liquidation_call(collateral_asset, debt_asset, total_debt_base, user)

    # Assert
    assert tx_hash == 'mocked_tx_hash'  # Check if mocked transaction hash is returned

@pytest.mark.asyncio
async def test_update_user_balance(liquidation_server):
    # Arrange
    user_id = 'user_123'
    new_balance = 5000.0
    mock_supabase.data = {
        '0xUserAddress': {'profit': 0, 'balance': 0},
        'user_123': {'profit': 0, 'balance': 0}
    }  # Initialize mock data for tests

    # Act
    await liquidation_server.update_user_balance(user_id, new_balance)

    # Assert
    assert mock_supabase.data[user_id]['balance'] == new_balance
