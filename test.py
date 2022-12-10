import socket, json

s = socket.socket()
host = "127.0.0.1"
port = 8087

s.connect((host, 8086))
a = {"client": {"id": 123, "ip": "123", "port": "123"}, "aim_user": 1234}
b = json.dumps(a)
print(b)
s.send(bytes(b, encoding="utf8"))
s.shutdown(socket.SHUT_WR)
b = s.recv(1024)
print(b)