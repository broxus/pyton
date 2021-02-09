import sys
sys.path.append("target/debug/")

from pyton import get_contract_info

with open("Test.tvc", 'rb') as file:
    tvc = file.read()

    info = get_contract_info(tvc)
    print(f"Code hash: {info.code_hash}")
    print(f"Data hash: {info.data_hash}")
