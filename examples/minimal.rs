use oculussdk_sys::*;
use std::ffi::CString;
use std::os::raw::c_void;

unsafe extern  fn log(
    user_data: usize,
    level: ::std::os::raw::c_int,
    message: *const ::std::os::raw::c_char,
){
    use std::ffi::CStr;
    let message = CStr::from_ptr(message);
    println!("User data: {}, level: {}, message: {:?}", user_data, level, message);
}




fn main(){
    let params: ovrInitParams = ovrInitParams {
        Flags: 0,
        RequestedMinorVersion: 0,
        LogCallback: Some(log),
        UserData: 101,
        ConnectionTimeoutMS: 0,
        pad0: [0,0,0,0]
    };
    let res = unsafe{
        ovr_Initialize(&params)
    };
    if res != 0 {
        println!("Error: {:?}", res);
        return;
    }

    let mut session = unsafe{ std::mem::zeroed() };
    let mut luid = unsafe{ std::mem::zeroed() };

    let res = unsafe{
        ovr_Create(&mut session, &mut luid)
    };
    if res != 0 {
        println!("Error: {:?}", res);
        return;
    }
    println!("{:?}", luid);


    let desc = unsafe{
        ovr_GetHmdDesc(session)
    };
    println!("{:?}", desc.Resolution);
    println!("{:?}", desc.DisplayRefreshRate);
    println!("{:?}", desc.DefaultEyeFov);

    let tracker_desc = unsafe{
        ovr_GetTrackerDesc(session, 0)
    };
    println!("{:?}", tracker_desc);
    let mut len =1024;
    let mut exts = [0i8; 1024];


    let c = unsafe{
        ovr_GetInstanceExtensionsVk(luid,  exts.as_mut_ptr(), &mut len)
    };




    use std::ffi::CStr;
    let m = unsafe{
        CStr::from_ptr(exts.as_ptr())
    };
    println!("{:?}", m);
    let c = unsafe{
        ovr_GetDeviceExtensionsVk(luid,  exts.as_mut_ptr(), &mut len)
    };
    let m = unsafe{
        CStr::from_ptr(exts.as_ptr())
    };
    println!("{:?}", m);

    let mut state = unsafe{
        std::mem::uninitialized()
    };
    let samples = (0..256).map(|x| {
        if x > 256 / 2 && x % 2 == 1 {
            0
        }else {
            255 *( (x as f32 * std::f32::consts::PI).sin() / 256.0) as u8
        }
    }).collect::<Vec<_>>();



    let haptic = ovrHapticsBuffer_{
        Samples: samples.as_ptr() as *const c_void,
        SamplesCount: 256,
        SubmitMode: ovrHapticsBufferSubmitMode__ovrHapticsBufferSubmit_Enqueue
    };

    unsafe{
        let er = ovr_SetControllerVibration(session, ovrControllerType__ovrControllerType_LTouch, 320.0, 90.0);
        if er != 0 {
            println!("Error set vibe: {:?}", er);
            return
        }
    }




    unsafe{
        let haptic_desc = ovr_GetTouchHapticsDesc(session,ovrControllerType__ovrControllerType_LTouch );

        println!("{:?}", haptic_desc);

    }
    let mut frame = 0;
    loop{
        unsafe{
            let dms = ovr_GetPredictedDisplayTime(session, frame);
            frame += 1;
            let track = ovr_GetTrackingState(
                session,
                dms,
                1
            );

            println!("State: {:#?}", track.HandPoses[0].ThePose);

            let er = ovr_GetInputState(
                session,
                1,
                &mut state
            );
            if er != 0 {
                println!("Error: {:?}", er);
                break;
            }

            println!("State: {:?}", state);


            {
                let mut play_state = std::mem::uninitialized();
                let er = ovr_GetControllerVibrationState(session, 0x0001, &mut play_state);
                ovr_SubmitControllerVibration(session, 0x0001, &haptic);
                // if er == 0 {
                    if play_state.RemainingQueueSpace >= 256 {
                        println!("Submiting");

                    }else{
                        println!("Cannot submit haptic");
                    }
               // }

            }

        }
        std::thread::sleep(std::time::Duration::from_secs(2));
    }

















    unsafe{
        ovr_Destroy(session);
        ovr_Shutdown();
    }


}