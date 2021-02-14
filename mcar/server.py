from flask import Flask, redirect, render_template, request
from flask_socketio import SocketIO, emit, send

from mcar.Car import Car

app = Flask(__name__)
socketio = SocketIO(app)
car = Car()


@app.route('/')
def main():
    return render_template('index.html')


@socketio.on('connection')
def handle_connection(data):
    print('connection established')
    emit('acknowledge')


@socketio.on('control')
def handle_control(x, y):
    print(x, y)
    car.drive(x, y)


@socketio.on('stop')
def handle_stop():
    print('stop')
    car.stop()


def start():
    socketio.run(app, host='0.0.0.0', port='80')
