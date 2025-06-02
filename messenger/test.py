import socket
from secrets import token_urlsafe
from random import getrandbits

PORT = 8085

data = {
    "from": hex(getrandbits(10)),
    "to": hex(getrandbits(12)),
    "contents": token_urlsafe()
}

with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    s.connect(("127.0.0.1", 8085))
    s.send(
        "\r\n".join(data.values()).encode()
    )
