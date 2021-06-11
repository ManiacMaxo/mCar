from flask import Flask, render_template, Response
from flask_socketio import SocketIO, emit

from utils import Car, stream

app = Flask(__name__)
socketio = SocketIO(app)
car = Car()


@app.route("/")
def main():
    return render_template("index.html")


@app.route("/video_feed")
def video_feed():
    return Response(stream(), mimetype="multipart/x-mixed-replace; boundary=frame")


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
