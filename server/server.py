from flask import Flask, Response, send_from_directory
from flask_socketio import SocketIO, emit

from utils import Car, stream

app = Flask(__name__)
socketio = SocketIO(app, cors_allowed_origins="*", logger=False, engineio_logger=False)
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


@socketio.event
def connection(data):
    print("connection established")
    emit("acknowledge")


@socketio.event
def control(x, y):
    print(f"control event x: {x}, y: {y}")
    car.drive(x, y)


@socketio.event
def stop():
    print("stop event")
    car.stop()


def start(host="0.0.0.0", port="3000"):
    socketio.run(app, host=host, port=port)


if __name__ == "__main__":
    start()
