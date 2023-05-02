// extern crate protocol;
// extern crate server;
// extern crate client;
// extern crate middleware;

// use pulse::context::pa_context_state_t::{
//     Authorizing, Connecting, Failed, Ready, SettingName, Terminated, Unconnected,
// };
use pulse::{context, mainloop};
use std::ffi::CString;
use std::os::raw::c_void;
use std::ptr::null;

fn pulse_context_state(pulse_context: *mut context::pa_context, user_data: *mut c_void) {
    unsafe {
        let pulse_state = context::pa_context_get_state(pulse_context);

        match pulse_state {
            Unconnected => println!("[Server]: Unconnected"),
            Connecting => println!("[Server]: Connecting"),
            Authorizing => println!("[Server]: Authorizing"),
            SettingName => println!("[Server]: Settings Name"),
            Ready => {
                println!("[Server]: Ready");
                // some more
            }
            Failed => {
                eprintln!("[Server]: Failed");
                std::process::exit(0);
            }
            Terminated => {
                println!("[Server]: Terminated");
                // close the main loop
            }
        }
    }
}

fn main() {
    // middleware::test::gen_sin(2.0, 440.0);
    let ret: *mut i32 = std::ptr::null_mut();

    let proc_name = CString::new("Device List:")
        .expect("CString Failed")
        .as_ptr();

    unsafe {
        let pulse_mainloop = mainloop::standard::pa_mainloop_new();
        let mainloop_api = mainloop::standard::pa_mainloop_get_api(pulse_mainloop);

        let pulse_context = context::pa_context_new(mainloop_api, proc_name);

        context::pa_context_connect(
            pulse_context,
            null(),
            context::flags::PA_CONTEXT_NOFLAGS,
            null(),
        );

        // TODO: Implement set callback
        // context::pa_context_set_state_callback(
        //     pulse_context,
        //     Some(pulse_callback),
        //     std::ptr::null_mut(),
        // );

        let status = mainloop::standard::pa_mainloop_iterate(pulse_mainloop, 0, ret);

        if status == -1 {
            eprintln!("Failed to open mainloop");
            std::process::exit(0);
        }

        mainloop::standard::pa_mainloop_run(pulse_mainloop, ret);
    }
}
