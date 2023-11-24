use crate::rdev::{Event, EventType, GrabError};
use crate::windows::common::{convert, set_key_hook, set_mouse_hook, HookError, HOOK, KEYBOARD};
use std::ptr::null_mut;
use std::time::SystemTime;
use winapi::um::winuser::{CallNextHookEx, GetMessageA, HC_ACTION};

static mut GLOBAL_CALLBACK: Option<Box<dyn FnMut(Event) -> Option<Event>>> = None;

unsafe extern "system" fn raw_callback(code: i32, param: usize, lpdata: isize) -> isize {
    if code == HC_ACTION {
        let opt = convert(param, lpdata);
        if let Some(event_type) = opt {
            let name = match &event_type {
                EventType::KeyPress(_key) => match (*KEYBOARD).lock() {
                    Ok(mut keyboard) => keyboard.get_name(lpdata),
                    Err(_) => None,
                },
                _ => None,
            };
            let event = Event {
                event_type,
                time: SystemTime::now(),
                name,
            };
            if let Some(callback) = &mut GLOBAL_CALLBACK {
                if callback(event).is_none() {
                    // https://stackoverflow.com/questions/42756284/blocking-windows-mouse-click-using-setwindowshookex
                    // https://android.developreference.com/article/14560004/Blocking+windows+mouse+click+using+SetWindowsHookEx()
                    // https://cboard.cprogramming.com/windows-programming/99678-setwindowshookex-wm_keyboard_ll.html
                    // let _result = CallNextHookEx(HOOK, code, param, lpdata);
                    return 1;
                }
            }
        }
    }
    CallNextHookEx(HOOK, code, param, lpdata)
}
impl From<HookError> for GrabError {
    fn from(error: HookError) -> Self {
        match error {
            HookError::Mouse(code) => GrabError::MouseHookError(code),
            HookError::Key(code) => GrabError::KeyHookError(code),
        }
    }
}

pub fn grab<T>(callback: T) -> Result<(), GrabError>
where
    T: FnMut(Event) -> Option<Event> + 'static,
{
    unsafe {
        GLOBAL_CALLBACK = Some(Box::new(callback));
        set_key_hook(raw_callback)?;
        set_mouse_hook(raw_callback)?;

        GetMessageA(null_mut(), null_mut(), 0, 0);
    }
    Ok(())
}

pub fn grab_t<T>(callback: T) -> Result<(), GrabError>
where
    T: FnMut(Event) -> Option<Event> + 'static,
{
    if callback(rdev_event).is_some() {
        match event.event_code {
            EventCode::EV_KEY(ref key) => match key {
                EV_KEY::KEY_ESC => (None, GrabStatus::Stop),
                _ => unsafe {
                GLOBAL_CALLBACK = Some(Box::new(callback));
                set_key_hook(raw_callback)?;
                set_mouse_hook(raw_callback)?;

                GetMessageA(null_mut(), null_mut(), 0, 0);
            },
            },
            _ => unsafe {
                GLOBAL_CALLBACK = Some(Box::new(callback));
                set_key_hook(raw_callback)?;
                set_mouse_hook(raw_callback)?;

                GetMessageA(null_mut(), null_mut(), 0, 0);
            },
        }
    } else {
        // callback returns None, swallow the event
        println!("swallowing event");
        (None, GrabStatus::Continue)
    }

    Ok(())
}
