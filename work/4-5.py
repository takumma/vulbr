import http.server
import socketserver

class HttpRequestHandler(http.server.SimpleHTTPRequestHandler):
    def do_GET(self):
        print("Server started at localhost:" + str(PORT))

        self.send_response(0)
        self.end_headers()
        html = f"<html><head></head><body><p>this is status code 0 page!</p></body></html>"
        self.wfile.write(bytes(html, "utf8"))
        return

handler = HttpRequestHandler

PORT = 8888
server = socketserver.TCPServer(("", PORT), handler)

server.serve_forever()
