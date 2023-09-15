
from pysvb import PyControlType, PyDemosaic
import pysvb as svb
import threading
import numpy as np
import cv2
def test_set_roi():
    svb.get_num_of_camera()

    camera = svb.PyCamera(0)
    camera.init()
    camera.set_roi_format(10,10,1980,1080,1)
    roi = camera.get_roi_format()
    print(roi)
    assert roi.startx == 10
    assert roi.starty == 10
    assert roi.width == 1980
    assert roi.height == 1080
    assert roi.bin == 1
    camera.close()
def test_set_img_type():
    svb.get_num_of_camera()

    camera = svb.PyCamera(0)
    camera.init() 
    try:
        camera.set_img_type(4)
        assert camera.get_img_type() == 4
        camera.close()
    except Exception as e:
        print(e)
        camera.close() 
def test_set_ctl_value():
    svb.get_num_of_camera()

    camera = svb.PyCamera(0)
    camera.init() 
    try:
        g = int(PyControlType.GAIN)
        camera.set_ctl_value( 50, 0)
        gain= camera.get_ctl_value(g)
        e=int(PyControlType.EXPOSURE)
        camera.set_ctl_value( e,1000000, 0)
        exp= camera.get_ctl_value(e)
        
        assert gain == 50
        assert exp == 1000000
        camera.close()
    except Exception as e:
        print(e)
        camera.close() 
def test_get_caps():
    n = svb.get_num_of_camera()
    print(n)
    camera = svb.PyCamera(0)
    camera.init()  

    n = camera.get_num_of_controls()
    print(n)
    for i in range(n):
        caps = camera.get_ctl_caps(i)
        print(caps.name, caps.default_value, caps.is_writable)
    camera.close()
 
def test_get_frame():

    n = svb.get_num_of_camera()
    print(n)
    camera = svb.PyCamera(0)
    camera.init()  

    camera.set_roi_format(0,0,4144,2822,1)

    camera.set_img_type(0)
    assert camera.get_img_type() == 0


    print(camera.get_ctl_value( 10))
    print(camera.get_ctl_value( 9))


    camera.set_ctl_value( 1,10000, 0)
    camera.set_ctl_value( 0,120, 0)
    exp= camera.get_ctl_value(1)[0]
    roi = camera.get_roi_format()
    w,h = roi.width,roi.height

    camera.start_video_capture()
    waitms = int((exp // 1000)  * 2 + 500)
    print(waitms)
    n=0
    try:
        while n < 1: 
            buf = camera.get_video_frame(waitms)
            buf = svb.debayer_buffer(camera,buf, PyDemosaic.Linear)
            img = np.frombuffer(bytes(buf) , dtype=np.uint8).reshape(h, w, 3)

            print(img[:10,:10,0])
            n+=1
            cv2.imwrite(f"../output/img{n}.png",img)
        camera.stop_video_capture()
        camera.close()
    except Exception as e:
        print(e)
        camera.stop_video_capture()
        camera.close() 