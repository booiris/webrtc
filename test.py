import socket, json

s = socket.socket()
host = "127.0.0.1"
port = 8087

s.connect((host, 8086))
a = {"client": {"id": 122223, "ip": "123", "port": "121233"}, "aim_user": 123}
b = json.dumps(a)
print(b)
s.send(bytes(b, encoding="utf8"))
s.shutdown(socket.SHUT_WR)
b = s.recv(1024)
print(b)