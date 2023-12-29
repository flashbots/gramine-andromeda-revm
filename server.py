import socket
import sys
import subprocess
import os
import time

def server(host='0.0.0.0', port=5556):
    
    cmd = "gramine-sgx ./sgx-revm"
    proc = subprocess.Popen(cmd,shell=True,stdout=subprocess.PIPE,
                            stdin=subprocess.PIPE)

    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
        s.bind((host,port))
        s.listen()
        while True:
            print(f"Server is listening on {host}:{port}")
            conn, addr = s.accept()
            print(f"Connected by {addr}")
            file = conn.makefile("r")
            while True:
                line = file.readline().strip()
                if not line: break
                print('recv:', line)
                proc.stdin.write(line.encode()+b'\n')
                proc.stdin.flush()
                try:
                    while True:
                        result = proc.stdout.readline().strip()
                        print('send:', result.decode('utf-8'))
                        conn.sendall(bytes(result.decode('utf-8') + "\n", "utf-8"))
                        if not result.startswith(b'{"depth"'): break
                except Exception as e:
                    print(e)
                    conn.sendall("invalid command\n")
                    break

if __name__ == '__main__':
    server()
