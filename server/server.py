#!/usr/bin/env python3
from pathlib import Path

from fastapi import FastAPI
from fastapi.responses import StreamingResponse
from fastapi.staticfiles import StaticFiles
from fastapi_socketio import SocketManager

from utils import stream

app = FastAPI()
socket_manager = SocketManager(app=app)


static_path = Path(__file__).parent / "client/public"
app.mount("/", StaticFiles(directory=static_path), name="static")


@app.get("/api/video_feed")
def video_feed():
    return StreamingResponse(
        content=stream(), media_type="multipart/x-mixed-replace; boundary=frame"
    )
