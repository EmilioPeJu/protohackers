#!/usr/bin/env python3
import argparse
import socket
import struct


def recv_value(sock):
    result = sock.recv(4)
    (val,) = struct.unpack(">I", result)
    print(val)


def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument('--ip', default="127.0.0.1")
    return parser.parse_args()


def main():
    args = parse_args()
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM, 0)
    sock.connect((args.ip, 1234))
    sock.sendall(b"\x49\x00\x00\x30\x39\x00\x00\x00\x65")
    sock.sendall(b"\x49\x00\x00\x30\x3a\x00\x00\x00\x66")
    sock.sendall(b"\x49\x00\x00\x30\x3b\x00\x00\x00\x64")
    sock.sendall(b"\x49\x00\x00\xa0\x00\x00\x00\x00\x05")
    sock.sendall(b"\x51\x00\x00\x30\x00\x00\x00\x40\x00")
    recv_value(sock)
    sock.sendall(b"\x51\x00\x00\x40\x00\x00\x00\x30\x00")
    recv_value(sock)
    sock.close()


if __name__ == "__main__":
    main()
