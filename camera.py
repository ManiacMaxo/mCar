import picamera
from time import sleep

with picamera.PiCamera() as camera:
    # with picamera.PiCameraCircularIO(camera, seconds=10 ) as stream:
    #     camera.start_recording(stream, format='h264' )
    #     camera.wait_recording(10)
    #     camera.stop_recording()
    #     stream.copy_to('test.mp4')
    camera.start_recording('video.h264')
    sleep(5)
    camera.stop_recording()
