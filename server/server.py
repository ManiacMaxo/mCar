from flask import Flask, Response, send_from_directory
from flask_socketio import SocketIO, emit

from utils import Car, stream

app = Flask(__name__)
socketio = SocketIO(app, cors_allowed_origins="*")
car = Car()


@app.route("/")
def main():
    return send_from_directory("../client/public", "index.html")


@app.route("/api/video_feed")
def video_feed():
    return Response(stream(), mimetype="multipart/x-mixed-replace; boundary=frame")


@app.route("/<path:path>")
def wildcard(path):
    return send_from_directory("../client/public", path)


@socketio.on("connection")
def handle_connection(data):
    print("connection established")
    emit("acknowledge")


@socketio.on("control")
def handle_control(x, y):
    print(x, y)
    car.drive(x, y)


@socketio.on("stop")
def handle_stop():
    print("stop")
    car.stop()


if __name__ == "__main__":
    socketio.run(app, host="0.0.0.0", port="3000")
