import http.server
import socketserver
import argparse

parser = argparse.ArgumentParser(description='sample http server with args');

parser.add_argument('statusCode', type=int, default=200, help='statu code')

args = parser.parse_args()

class HttpRequestHandler(http.server.SimpleHTTPRequestHandler):
    def do_GET(self):
        print("Server started at localhost:" + str(PORT))

        self.send_response(args.statusCode)
        self.end_headers()
        html = f"<html><head></head><body><p>this is status code { args.statusCode } page!</p></body></html>"
        self.wfile.write(bytes(html, "utf8"))
        return

handler = HttpRequestHandler

PORT = 8888
server = socketserver.TCPServer(("", PORT), handler)

server.serve_forever()
