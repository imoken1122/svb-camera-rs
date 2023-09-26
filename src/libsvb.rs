#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
//#![allow(improper_ctypes)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
use log::{error, info};
use std::fmt;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SVBError {
    #[error("Success")]
    Success,

    #[error("Invalid index: no camera connected or index value out of boundary")]
    InvalidIndex,

    #[error("Invalid ID")]
    InvalidId,

    #[error("Invalid control type")]
    InvalidControlType,

    #[error("Camera closed: camera didn't open")]
    CameraClosed,

    #[error("Camera removed: failed to find the camera, maybe the camera has been removed")]
    CameraRemoved,

    #[error("Invalid path: cannot find the path of the file")]
    InvalidPath,

    #[error("Invalid file format")]
    InvalidFileFormat,

    #[error("Invalid size: wrong video format size")]
    InvalidSize,

    #[error("Invalid image type: unsupported image format")]
    InvalidImgType,

    #[error("Out of boundary: the start position is out of boundary")]
    OutOfBoundary,

    #[error("Timeout")]
    Timeout,

    #[error("Invalid sequence: stop capture first")]
    InvalidSequence,

    #[error("Buffer too small: buffer size is not big enough")]
    BufferTooSmall,

    #[error("Video mode active")]
    VideoModeActive,

    #[error("Exposure in progress")]
    ExposureInProgress,

    #[error("General error: general error, e.g., value is out of valid range")]
    GeneralError,

    #[error("Invalid mode: the current mode is wrong")]
    InvalidMode,

    #[error("Invalid direction: invalid guide direction")]
    InvalidDirection,

    #[error("Unknown sensor type: unknown sensor type")]
    UnknownSensorType,


    #[error("Overflow resolution: overflow resolution")]
    OverFlowResolution,
    
    #[error("Unknown error")]
    UnknownError,
}
pub fn convert_err_code(code: i32) -> SVBError {
    match code {
        0 => SVBError::Success,
        1 => SVBError::InvalidIndex,
        2 => SVBError::InvalidId,
        3 => SVBError::InvalidControlType,
        4 => SVBError::CameraClosed,
        5 => SVBError::CameraRemoved,
        6 => SVBError::InvalidPath,
        7 => SVBError::InvalidFileFormat,
        8 => SVBError::InvalidSize,
        9 => SVBError::InvalidImgType,
        10 => SVBError::OutOfBoundary,
        11 => SVBError::Timeout,
        12 => SVBError::InvalidSequence,
        13 => SVBError::BufferTooSmall,
        14 => SVBError::VideoModeActive,
        15 => SVBError::ExposureInProgress,
        16 => SVBError::GeneralError,
        17 => SVBError::InvalidMode,
        18 => SVBError::InvalidDirection,
        _ => SVBError::UnknownSensorType, // 不明なエラーコードの場合
    }
}

pub type SVBControlValue = i64;

#[derive(Debug, Copy, Clone)]
pub struct ControlTypeState {
    pub value: SVBControlValue,
    pub is_auto: i32,
}
#[derive(Debug, Copy, Clone)]
pub struct ROIFormat {
    pub startx: i32,
    pub starty: i32,
    pub width: i32,
    pub height: i32,
    pub bin: i32,}
impl ROIFormat {
    pub fn new() -> Self {
        Self { 
            startx :0 ,
            starty : 0,
            width : 0,
            height : 0,
            bin : 0
        }
    }
}
impl fmt::Display for ROIFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\nStart X: {}\nStart Y: {}\nWidth: {}\nHeight: {}\nBin: {}\n",
            self.startx, self.starty, self.width, self.height, self.bin,
        )
    }
}
impl SVB_SN {
    pub fn new() -> Self {
        Self { id: [0; 64] }
    }
}

impl SVB_CAMERA_INFO {
    pub fn new() -> SVB_CAMERA_INFO {
        Self {
            FriendlyName: [0; 32],
            CameraSN: [0; 32],
            PortType: [0; 32],
            DeviceID: 0u32,
            CameraID: 0i32,
        }
    }
}
impl fmt::Display for SVB_CAMERA_INFO {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a: Vec<u8> = self.FriendlyName.iter().map(|&x| x as u8).collect();
        let b: Vec<u8> = self.CameraSN.iter().map(|&x| x as u8).collect();
        let c: Vec<u8> = self.PortType.iter().map(|&x| x as u8).collect();
        write!(
            f,
            "\nFriendlyName: {:?}\nCameraSN: {:?}\nPortType: {:?}\nDeviceID: {}\nCameraID: {}\n",
            String::from_utf8_lossy(&a),
            String::from_utf8_lossy(&b),
            String::from_utf8_lossy(&c),
            self.DeviceID,
            self.CameraID
        )
    }
}
impl SVB_CAMERA_PROPERTY {
    pub fn new() -> SVB_CAMERA_PROPERTY {
        Self {
            MaxHeight: 0,
            MaxWidth: 0,
            IsColorCam: 0,
            BayerPattern: 0,
            SupportedBins: [0; 16],
            SupportedVideoFormat: [0; 8],
            MaxBitDepth: 0,
            IsTriggerCam: 0,
        }
    }
}
impl fmt::Display for SVB_CAMERA_PROPERTY {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\nMaxHeight: {}\nMaxWidth: {}\nIsColorCam: {}\nBayerPattern: {}\nMaxBitDepth: {}\nIsTriggerCam: {}\nSupportedBins: {:?}\nSupportedVideoFormat: {:?}\n",
            self.MaxHeight,
            self.MaxWidth,
            self.IsColorCam,
            self.BayerPattern,
            self.MaxBitDepth,
            self.IsTriggerCam,
            self.SupportedBins,
            self.SupportedVideoFormat
        )
    }
}

impl SVB_CONTROL_CAPS {
    pub fn new() -> Self {
        Self {
            Name: [0; 64],
            Description: [0; 128],
            MaxValue: 0,
            MinValue: 0,
            DefaultValue: 0,
            IsAutoSupported: 0,
            IsWritable: 0,
            ControlType: 0,
            Unused: [0; 32],
        }
    }
}

impl fmt::Display for SVB_CONTROL_CAPS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name: Vec<u8> = self.Name.iter().map(|&x| x as u8).collect();
        let doc: Vec<u8> = self.Description.iter().map(|&x| x as u8).collect();
        write!(f, "\nName: {}\nDescription: {}\nMaxValue: {}\nMinValue: {}\nDefaultValue: {}\nIsAutoSupported: {}\nIsWritable: {}\nControlType: {}\n",
            String::from_utf8_lossy(&name),
            String::from_utf8_lossy(&doc),
               self.MaxValue,
               self.MinValue,
               self.DefaultValue,
               self.IsAutoSupported,
               self.IsWritable,
               self.ControlType,)
    }
}

pub fn _get_num_of_connected_cameras() -> i32 {
    unsafe { SVBGetNumOfConnectedCameras() }
}

/***************************************************************************
Descriptions:
    open the camera before any operation to the camera, this will not affect the camera which is capturing
    All APIs below need to open the camera at first.

Paras:
    int CameraID: this is get from the camera property use the API SVBGetCameraInfo

return:
SVB_SUCCESS: Operation is successful
SVB_ERROR_INVALID_ID  : no camera of this ID is connected or ID value is out of boundary
SVB_ERROR_CAMERA_REMOVED: failed to find the camera, maybe camera has been removed

***************************************************************************/
pub fn _open_camera(camera_id: i32) -> SVBError {
    convert_err_code(unsafe { SVBOpenCamera(camera_id) })
}

/***************************************************************************
Descriptions:
    open the camera before any operation to the camera, this will not affect the camera which is capturing
    All APIs below need to open the camera at first.

Paras:
    int CameraID: this is get from the camera property use the API SVBGetCameraInfo

return:
SVB_SUCCESS: Operation is successful
SVB_ERROR_INVALID_ID  : no camera of this ID is connected or ID value is out of boundary
SVB_ERROR_CAMERA_REMOVED: failed to find the camera, maybe camera has been removed

*************************************************************/
pub fn _close_camera(camera_id: i32) -> SVBError {
    convert_err_code(unsafe { SVBCloseCamera(camera_id) })
}

/***************************************************************************
Descriptions:
Get number of controls available for this camera. the camera need be opened at first.



Paras:
int CameraID: this is get from the camera property use the API SVBGetCameraInfo
int * piNumberOfControls: pointer to an int to save the number of controls

return:
SVB_SUCCESS : Operation is successful
SVB_ERROR_CAMERA_CLOSED : camera didn't open
SVB_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary
***************************************************************************/

pub fn _get_num_of_controls(camera_id: i32, num_ctls: &mut i32) -> i32 {
    unsafe { SVBGetNumOfControls(camera_id, num_ctls) }
}

/***************************************************************************
Descriptions:
Get controls property available for this camera. the camera need be opened at first.
user need to malloc and maintain the buffer.



Paras:
int CameraID: this is get from the camera property use the API SVBGetCameraInfo
int iControlIndex: index of control, NOT control type
SVB_CONTROL_CAPS * pControlCaps: Pointer to structure containing the property of the control
user need to malloc the buffer

return:
SVB_SUCCESS : Operation is successful
SVB_ERROR_CAMERA_CLOSED : camera didn't open
SVB_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary
***************************************************************************/
pub fn _get_ctl_caps(camera_id: i32, ctl_idx: i32, ctl_caps: &mut SVB_CONTROL_CAPS) -> i32 {
    unsafe { SVBGetControlCaps(camera_id, ctl_idx, ctl_caps) }
}

/***************************************************************************
Descriptions:
get the property of the connected cameras
here is the sample code:

Paras:
int CameraID: this is get from the camera property use the API SVBGetCameraProperty
SVB_CAMERA_PROPERTY *pCameraProperty: Pointer to structure containing the property of camera
                                user need to malloc the buffer

return:
SVB_SUCCESS: Operation is successful
SVB_ERROR_INVALID_INDEX  :no camera connected or index value out of boundary

************************************/
pub fn _get_camera_prop(camera_id: i32) -> Result<SVB_CAMERA_PROPERTY, SVBError> {
    let mut camera_prop = SVB_CAMERA_PROPERTY::new();
    match unsafe { SVBGetCameraProperty(camera_id, &mut camera_prop) } {
        0 => Ok(camera_prop),
        code => Err(convert_err_code(code)),
    }
}

/***************************************************************************
Descriptions:
get the information of the connected cameras, you can do this without open the camera.
here is the sample code:

int iNumofConnectCameras = SVBGetNumOfConnectedCameras();
SVB_CAMERA_INFO **ppSVBCameraInfo = (SVB_CAMERA_INFO **)malloc(sizeof(SVB_CAMERA_INFO *)*iNumofConnectCameras);
for(int i = 0; i < iNumofConnectCameras; i++)
{
ppSVBCameraInfo[i] = (SVB_CAMERA_INFO *)malloc(sizeof(SVB_CAMERA_INFO ));
SVBGetCameraInfo(ppSVBCameraInfo[i], i);
}

Paras:
    SVB_CAMERA_INFO *pSVBCameraInfo: Pointer to structure containing the information of camera
                                    user need to malloc the buffer
    int iCameraIndex: 0 means the first connect camera, 1 means the second connect camera

return:
    SVB_SUCCESS: Operation is successful
    SVB_ERROR_INVALID_INDEX  :no camera connected or index value out of boundary

************************************************/
pub fn _get_camera_info(camera_idx: i32) -> Result<SVB_CAMERA_INFO, SVBError> {
    let mut camera_info = SVB_CAMERA_INFO::new();
    match unsafe { SVBGetCameraInfo(&mut camera_info, camera_idx) } {
        0 => Ok(camera_info),
        e => Err(convert_err_code(e)),
    }
}

/***************************************************************************
Descriptions:
Get the current ROI area setting .

Paras:
int CameraID: this is get from the camera property use the API SVBGetCameraInfo
int *piWidth,  pointer to the width of the ROI area
int *piHeight, pointer to the height of the ROI area.
int *piBin,   pointer to binning method. bin1=1, bin2=2

return:
SVB_SUCCESS : Operation is successful
SVB_ERROR_CAMERA_CLOSED : camera didn't open
SVB_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary

********************************************************************/
pub fn _get_roi_format(
    camera_id: i32,
    startx: &mut i32,
    starty: &mut i32,
    width: &mut i32,
    height: &mut i32,
    bin: &mut i32,
) -> SVBError {
    convert_err_code(unsafe { SVBGetROIFormat(camera_id, startx, starty, width, height, bin) })
}

/***************************************************************************
Descriptions:
set the ROI area before capture.
you must stop capture before call it.
the width and height is the value after binning.
ie. you need to set width to 640 and height to 480 if you want to run at 640X480@BIN2
SVB120's data size must be times of 1024 which means width*height%1024=0SVBSetStartPos
Paras:
int CameraID: this is get from the camera property use the API SVBGetCameraInfo
int iWidth,  the width of the ROI area. Make sure iWidth%8 = 0.
int iHeight,  the height of the ROI area. Make sure iHeight%2 = 0,
further, for USB2.0 camera SVB120, please make sure that iWidth*iHeight%1024=0.
int iBin,   binning method. bin1=1, bin2=2

return:
SVB_SUCCESS : Operation is successful
SVB_ERROR_CAMERA_CLOSED : camera didn't open
SVB_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary
SVB_ERROR_INVALID_SIZE, //wrong video format size
SVB_ERROR_INVALID_IMGTYPE, //unsupported image format, make sure iWidth and iHeight and binning is set correct
*************************************************************************/
pub fn _set_roi_format(
    camera_id: i32,
    startx: i32,
    starty: i32,
    width: i32,
    height: i32,
    bin: i32,
) -> SVBError {
    convert_err_code(unsafe { SVBSetROIFormat(camera_id, startx, starty, width, height, bin) })
}

/***************************************************************************
Descriptions:
Start video capture
then you can get the data from the API SVBGetVideoData


Paras:
int CameraID: this is get from the camera property use the API SVBGetCameraInfo

return:
SVB_SUCCESS : Operation is successful, it will return success if already started
SVB_ERROR_CAMERA_CLOSED : camera didn't open
SVB_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary
SVB_ERROR_EXPOSURE_IN_PROGRESS: snap mode is working, you need to stop snap first
***********************************************************************/

pub fn _start_video_capture(camera_id: i32) -> SVBError {
    convert_err_code(unsafe { SVBStartVideoCapture(camera_id) })
}

/***************************************************************************
Descriptions:
get data from the video buffer.the buffer is very small
you need to call this API as fast as possible, otherwise frame will be discarded
so the best way is maintain one buffer loop and call this API in a loop
please make sure the buffer size is biger enough to hold one image
otherwise the this API will crash


Paras:
int CameraID: this is get from the camera property use the API SVBGetCameraInfo
unsigned char* pBuffer, caller need to malloc the buffer, make sure the size is big enough
        the size in byte:
        8bit mono:width*height
        16bit mono:width*height*2
        RGB24:width*height*3

int iWaitms, this API will block and wait iWaitms to get one image. the unit is ms
        -1 means wait forever. this value is recommend set to exposure*2+500ms

return:
SVB_SUCCESS : Operation is successful
SVB_ERROR_CAMERA_CLOSED : camera didn't open
SVB_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary
SVB_ERROR_TIMEOUT: no image get and timeout
*********************************************************/

pub fn _get_video_data(camera_id: i32, pbuf: *mut u8, buf_size: i64, wait_ms: i32) -> SVBError {
    convert_err_code(unsafe { SVBGetVideoData(camera_id, pbuf, buf_size, wait_ms) })
}

pub fn _stop_video_capture(camera_id: i32) -> SVBError {
    convert_err_code(unsafe { SVBStopVideoCapture(camera_id) })
}

/***************************************************************************
Descriptions:
Set the output image type, The value set must be the type supported by the SVBGetCameraProperty function.

Paras:
int CameraID: this is get from the camera property use the API SVBGetCameraInfo
SVB_IMG_TYPE *pImageType: pointer to current image type.

return:
SVB_SUCCESS : Operation is successful
SVB_ERROR_CAMERA_CLOSED : camera didn't open
SVB_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary
SVB_ERROR_INVALID_IMGTYPE, //invalid image type
SVB_ERROR_GENERAL_ERROR,//general error, eg: value is out of valid range; operate to camera hareware failed
********************************************************/
pub fn _set_img_type(camera_id: i32, img_type: SVB_IMG_TYPE) -> SVBError {
    convert_err_code(unsafe { SVBSetOutputImageType(camera_id, img_type) })
}

/***************************************************************************
Descriptions:
Get the output image type.

Paras:
int CameraID: this is get from the camera property use the API SVBGetCameraInfo
SVB_IMG_TYPE *pImageType: pointer to current image type.

return:
SVB_SUCCESS : Operation is successful
SVB_ERROR_CAMERA_CLOSED : camera didn't open
SVB_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary
SVB_ERROR_GENERAL_ERROR,//general error, eg: value is out of valid range; operate to camera hareware failed
***************************************************************/
pub fn _get_img_type(camera_id: i32, img_type: &mut SVB_IMG_TYPE) -> SVBError {
    convert_err_code(unsafe { SVBGetOutputImageType(camera_id, img_type) })
}

/***************************************************************************
Descriptions:
Get controls property value and auto value
note:the value of the temperature is the float value * 10 to convert it to long type, control name is "Temperature"
because long is the only type for control(except cooler's target temperature, because it is an integer)

Paras:
int CameraID: this is get from the camera property use the API SVBGetCameraInfo
int ControlType: this is get from control property use the API SVBGetControlCaps
long *plValue: pointer to the value you want to save the value get from control
SVB_BOOL *pbAuto: pointer to the SVB_BOOL type

return:
SVB_SUCCESS : Operation is successful
SVB_ERROR_CAMERA_CLOSED : camera didn't open
SVB_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary
SVB_ERROR_INVALID_CONTROL_TYPE, //invalid Control type
***************************************************************/
pub fn _get_ctl_value(
    camera_id: i32,
    ctl_type: SVB_CONTROL_TYPE,
    value: &mut SVBControlValue,
    is_auto: &mut i32,
) -> SVBError {
    convert_err_code(unsafe { SVBGetControlValue(camera_id, ctl_type as i32, value, is_auto) })
}

/***************************************************************************
Descriptions:
Set controls property value and auto value
it will return success and set the max value or min value if the value is beyond the boundary


Paras:
int CameraID: this is get from the camera property use the API SVBGetCameraInfo
int ControlType: this is get from control property use the API SVBGetControlCaps
long lValue: the value set to the control
SVB_BOOL bAuto: set the control auto

return:
SVB_SUCCESS : Operation is successful
SVB_ERROR_CAMERA_CLOSED : camera didn't open
SVB_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary
SVB_ERROR_INVALID_CONTROL_TYPE, //invalid Control type
SVB_ERROR_GENERAL_ERROR,//general error, eg: value is out of valid range; operate to camera hareware failed
******************************************************************/

pub fn _set_ctl_value(
    camera_id: i32,
    ctl_type: SVB_CONTROL_TYPE,
    value: SVBControlValue,
    is_auto: SVB_BOOL,
) -> SVBError {
    convert_err_code(unsafe {
        SVBSetControlValue(camera_id, ctl_type as i32, value, is_auto as i32)
    })
}

pub fn _get_serial_number(camera_id: i32, sn: *mut SVB_SN) -> SVBError {
    convert_err_code(unsafe { SVBGetSerialNumber(camera_id, sn) })
}

pub fn _get_droped_frame(camera_id: i32, num_droppd_frame: &mut i32) -> SVBError {
    convert_err_code(unsafe { SVBGetDroppedFrames(camera_id, num_droppd_frame) })
}



pub fn _adjust_white_balance(
    camera_id: i32,
) -> SVBError {
    convert_err_code(unsafe { SVBWhiteBalanceOnce(camera_id ) })
}