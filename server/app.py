#!/usr/bin/env python3
from pathlib import Path

from fastapi import FastAPI, WebSocket
from fastapi.responses import StreamingResponse
from fastapi.staticfiles import StaticFiles

from .utils import car, stream

app = FastAPI()


static_path = Path(__file__).parent.parent / "client/dist"


@app.get("/api/feed")
def video_feed():
    return StreamingResponse(
        content=stream(), media_type="multipart/x-mixed-replace; boundary=frame"
    )


@app.websocket("/")
async def websocket_endpoint(websocket: WebSocket):
    await websocket.accept()

    while True:
        try:
            data = await websocket.receive_json()
        except:
            continue

        event = data.get("event")
        if event == "control":
            car.drive(data["data"]["x"], data["data"]["y"])

        elif event == "stop":
            car.stop()


app.mount("/", StaticFiles(directory=static_path), name="static")
