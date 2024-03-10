from picamera2.encoders import H264Encoder
from picamera2 import Picamera2
import time
import io
import sys
import os

class FakeStdOut:
    def write(self, text):
        pass

    def flush(self):
        pass


# Temp disable stdout
original_stdout = sys.stdout
sys.stdout = FakeStdOut()

picam2 = Picamera2()
config = picam2.create_preview_configuration(raw={"size": picam2.sensor_resolution})
picam2.configure(config)

picam2.start()
time.sleep(2)

image = picam2.capture_image("main").convert('RGB')
img_byte_arr = io.BytesIO()
image.save(img_byte_arr, format='PNG')

# Use original stdout in order to dump image data
with os.fdopen(original_stdout.fileno(), "wb", closefd=False) as stdout:
    stdout.write(img_byte_arr.getvalue())
