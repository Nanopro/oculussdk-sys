use oculussdk_sys::*;

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































    unsafe{
        ovr_Destroy(session);
        ovr_Shutdown();
    }


}