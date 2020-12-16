from libpyton import PyTonClient

client = PyTonClient("54.158.97.195:3031", "uNRRL+6enQjuiZ/s6Z+vO7yxUUR7uxdfzIy+RxkECrc=")

test: bytes = b"asd"
client.send_message(test)
