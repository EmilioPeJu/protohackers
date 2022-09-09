#!/usr/bin/env python3
import socket
import threading
MAX_BACKLOG = 5
PORT = 7


def handle_client(client_sock, addr):
    print(f"{addr} connected")
    while True:
        data = client_sock.recv(4096)
        if data == b'':
            break
        client_sock.sendall(data)
    print(f"{addr} disconnected")
    client_sock.close()


def main():
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM, 0)
    sock.bind(('', PORT))
    sock.listen(MAX_BACKLOG)
    while True:
        client_sock, addr = sock.accept()
        threading.Thread(None, handle_client, args=(client_sock, addr)).start()


if __name__ == "__main__":
    main()
