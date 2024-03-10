from picamera2.encoders import H264Encoder
from picamera2 import Picamera2
import time
import os

picam2 = Picamera2()
config = picam2.create_preview_configuration(raw={"size": picam2.sensor_resolution})
picam2.configure(config)

picam2.start()
time.sleep(2)

print(picam2.capture_array("raw").tobytes()) #Dump into stdout
