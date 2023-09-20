
from pysvb import SVBControlType, PyDemosaic,SVBImgType
import pysvb as svb
import threading
import numpy as np
import cv2
def test_set_roi():
    n= svb.get_num_of_camera()
    print(n)
    camera = svb.SVBCamera(0)
    camera.init()
    roi = camera.get_roi_format()
    print(roi)
    assert roi.startx == 0
    assert roi.starty == 0
    assert roi.width == 4144
    assert roi.height == 2822
    assert roi.bin == 1 

    camera.set_roi_format(0,0,4080,2080,1)
    roi = camera.get_roi_format()
    print(roi)
    assert roi.startx == 0
    assert roi.starty == 0
    assert roi.width == 4080
    assert roi.height == 2080
    assert roi.bin == 1 

    camera.set_resolution(2000,1200)
    camera.set_bin(2)
    roi = camera.get_roi_format()
    assert  roi.width == 2000 
    assert  roi.height == 1200 
    assert  roi.bin ==2 
    camera.close()
def test_set_img_type():
    svb.get_num_of_camera()

    camera = svb.SVBCamera(0)
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

    camera = svb.SVBCamera(0)
    camera.init() 
    try:
        g = int(SVBControlType.GAIN)
        camera.set_ctl_value( 50, 0)
        gain= camera.get_ctl_value(g)
        e=int(SVBControlType.EXPOSURE)
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
    camera = svb.SVBCamera(0)
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
    camera = svb.SVBCamera(0)
    camera.init()  

    camera.set_roi_format(0,0,600,400,1)

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
        while n < 100: 
            buf = camera.get_raw_frame()
            buf = svb.debayer_buffer(camera,buf,PyDemosaic.Linear)
            img = np.frombuffer(bytes(buf) , dtype=np.uint8).reshape(h, w, 3)

            print(img[:10,:10,0])
            n+=1
            #cv2.imwrite(f"../output/img{n}.png",img)
        camera.stop_video_capture()
        camera.close()
    except Exception as e:
        print(e)
        camera.stop_video_capture()
        camera.close() 

def test_info():
    n = svb.get_num_of_camera()
    print(n)
    camera = svb.SVBCamera(0)
    camera.init()  

    info = camera.get_info()
    print(info.friendly_name)
    prop= camera.get_prop()
    print(prop.max_width,prop.max_height,prop.supported_bins,prop.max_bit_depth, prop.supported_video_formats)
    camera.close()
