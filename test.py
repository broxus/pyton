import sys
sys.path.append("target/debug/")

from libpyton import PyTonClient

client = PyTonClient(address="54.158.97.195:3032", key="uNRRL+6enQjuiZ/s6Z+vO7yxUUR7uxdfzIy+RxkECrc=")

test: bytes = b"asd"
client.send_message(test)
