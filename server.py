from flask import Flask, request, make_response
from flask_cors import CORS
import subprocess
import sys
import socket
import signal
import os
import time

app = Flask(__name__)
CORS(app)

cmd = "gramine-sgx ./sgx-revm"
proc = None

def tcp_server(host='0.0.0.0', port=5556):
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

@app.route('/', methods=['POST'])
def http_server():
    line = request.data.decode('utf-8').strip()
    print('recv:', line)
    proc.stdin.write(line.encode()+b'\n')
    proc.stdin.flush()
    try:
        full_result = ""
        while True:
            result = proc.stdout.readline().strip()
            full_result += result.decode('utf-8') + "\n"
            if not result.startswith(b'{"depth"'): break
        print("send:", full_result)
        response = make_response(full_result, 200)
        response.mimetype = "text/plain"
        return response
    except Exception as e:
        print(e)
        response = make_response("invalid command\n", 400)
        response.mimetype = "text/plain"
        return response

if __name__ == '__main__':
    if len(sys.argv) < 2:
        print("Usage: server.py tcp|http [port]")
        exit(-1)

    port = int(sys.argv[2]) if len(sys.argv) == 3 else 5556

    os.setpgrp()

    try:
        proc = subprocess.Popen(cmd,shell=True,stdout=subprocess.PIPE,
                                stdin=subprocess.PIPE)
        
        if sys.argv[1] == "tcp":
            tcp_server(port=port)
        elif sys.argv[1] == "http":
            app.run(host='0.0.0.0', port=port)
        else:
            print("Usage: server.py tcp|http [port]")
    finally:
        os.killpg(0, signal.SIGKILL)
