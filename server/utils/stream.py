import cv2

camera = cv2.VideoCapture(0)


def stream():
    while True:
        success, frame = camera.read()  # read the camera frame
        frame = cv2.flip(frame, -1)
        if not success:
            break
        _, buffer = cv2.imencode(".jpg", frame)
        frame = buffer.tobytes()
        yield (
            b"--frame\r\n" b"Content-Type: image/jpeg\r\n\r\n" + frame + b"\r\n"
        )  # concat frame one by one and show result
