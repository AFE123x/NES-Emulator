# server.py
import socket

HOST = '73.10.245.152'  # Accept connections on all interfaces
PORT = 12345      # Port to listen on (must match client's)

with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    s.bind((HOST, PORT))
    s.listen()
    print(f"Server listening on port {PORT}...")

    conn, addr = s.accept()
    with conn:
        print(f"Connected by {addr}")
        while True:
            data = conn.recv(1024)
            if not data:
                break
            print(f"Client says: {data.decode()}")
            conn.sendall(b'Hello from server!')

