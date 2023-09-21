import datetime
import time
from pprint import pprint

import base64
import hmac
import hashlib
import os
from dotenv import load_dotenv

from dydx3 import Client
from dydx3.constants import ORDER_SIDE_BUY
from dydx3.constants import ORDER_SIDE_SELL
from dydx3.constants import ORDER_TYPE_MARKET
from dydx3.constants import MARKET_ETH_USD
from dydx3.constants import TIME_IN_FORCE_FOK
from dydx3.constants import TIME_IN_FORCE_GTT
from dydx3.constants import TIME_IN_FORCE_IOC
from dydx3.constants import API_HOST_GOERLI
from dydx3.constants import NETWORK_ID_GOERLI
from dydx3.constants import API_HOST_MAINNET
from dydx3.constants import NETWORK_ID_MAINNET
from dydx3.starkex.order import SignableOrder

from dydx3.starkex.starkex_resources.python_signature import py_sign
from dydx3.starkex.starkex_resources.python_signature import py_pedersen_hash
from dydx3.starkex.helpers import serialize_signature

load_dotenv()

privateClient = Client(
    network_id=NETWORK_ID_MAINNET,
    host=API_HOST_MAINNET,
    default_ethereum_address=os.getenv('ETHEREUM_ADDRESS_MAINNET'),
    eth_private_key=os.getenv('ETH_PRIVATE_KEY_MAINNET'),
    stark_private_key=os.getenv('STARK_PRIVATE_KEY_MAINNET'),
    api_key_credentials={
        'key': os.getenv('API_KEY_MAINNET'),
        'secret': os.getenv('API_SECRET_MAINNET'),
        'passphrase': os.getenv('API_PASSPHRASE_MAINNET')
    }
)

timestamp = datetime.datetime.utcnow().isoformat()
signature = privateClient.private.sign(
        iso_timestamp=timestamp,
        request_path='/v3/accounts',
        method='GET',
        data={
        }
)

account = privateClient.private.get_account()
account = account.data['account']

order_to_sign = SignableOrder(
                network_id=NETWORK_ID_GOERLI,
                position_id=184552,
                client_id="2",
                market=MARKET_ETH_USD,
                side=ORDER_SIDE_BUY,
                human_size='0.01',
                human_price=1500,
                limit_fee=0.02,
                expiration_epoch_seconds=100,
            )

#print("[+] Hash: " + str(order_to_sign.hash))
print('[+] Position Id: ' + str(account['positionId']))
#(r, s) = py_sign(order_to_sign.hash, int(privateClient.stark_private_key, 16))
#signature = serialize_signature(r, s)
#print('[+] Expected signature for above hash: ' + signature)

def create_market_order(market, side, size, price):
    try:
        order_params = {
            'position_id': account['positionId'],
            'market': market,
            'side': side,
            'order_type': ORDER_TYPE_MARKET,
            'post_only': False,
            'size': size,
            'price': price,
            'limit_fee': '0.01',
            'expiration_epoch_seconds': time.time() + 86400,
            'time_in_force': TIME_IN_FORCE_IOC,}
        order_response = privateClient.private.create_order(**order_params)
        pprint(order_response.data)
    except Exception as e:
            print(e)

create_market_order(MARKET_ETH_USD, ORDER_SIDE_BUY, '0.01', '1500');
