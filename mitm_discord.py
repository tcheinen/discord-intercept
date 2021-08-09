import zlib
from base64 import b64encode

class DecompressWebsocket:
    def __init__(self):
        self.buffer = bytearray()
        self.inflator = zlib.decompressobj()

    def websocket_message(self, flow):
        ZLIB_SUFFIX = b'\x00\x00\xff\xff'

        message = flow.websocket.messages[-1]
        if message.from_client: # i only care about incoming messages
            return
        content = message.content
        self.buffer.extend(content)
        if len(content) < 4 or content[-4:] != ZLIB_SUFFIX:
            return
        msg = self.inflator.decompress(self.buffer)
        self.buffer = bytearray()
        print(b64encode(msg).decode())


addons = [
    DecompressWebsocket()
]