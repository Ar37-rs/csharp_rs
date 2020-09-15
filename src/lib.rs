use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::{sync::mpsc::channel, sync::Mutex, thread};
use num_cpus;
use ryu;

pub struct Strs {
    str_ptr: CString,
}

impl<'a> Strs {
    pub fn from_ptr(ptr: *mut c_char) -> Self {
        Self {
            str_ptr: unsafe { CString::from_raw(ptr) },
        }
    }

    pub fn from_str(str: &str) -> Self {
        Self {
            str_ptr: CString::new(str).unwrap_or_default(),
        }
    }

    pub fn from_string(string: String) -> Self {
        Self {
            str_ptr: CString::new(string).unwrap_or_default(),
        }
    }

    pub fn to_str(&self) -> &'a str {
        unsafe {
            CStr::from_ptr(self.str_ptr.clone().into_raw())
                .to_str()
                .unwrap_or_default()
        }
    }

    pub fn to_string(self) -> String {
        self.str_ptr.to_str().unwrap_or_default().to_owned()
    }

    pub fn as_ptr(self) -> *mut c_char {
        self.str_ptr.into_raw()
    }
}

pub fn vec_f64_to_string() -> String {
    let string: String;
    {
        // // multithread mode 

        let (s, r) = channel();
        let cpus: usize = match num_cpus::get() {
            cpu if cpu == 2 => cpu,
            cpu if cpu > 2 => cpu / 2,
            _ => 1
        };

        for _ in 0..cpus {
            let sc = s.clone();
            thread::spawn(move || {
                let mut string = String::new();
                let mtx = Mutex::new(&mut string);
                for i in 0..30 {
                    mtx.lock().unwrap().push_str(&[(ryu::Buffer::new().format_finite(std::f64::consts::PI*(i as f64))).to_owned() + "`"].concat().replace(".", ","));
                }
                sc.send(string).ok()
            });
        }
        string = r.recv().expect("Error`4044,0404")

        // // single thread mode, faster on low capacity vec

        // let mut strings = String::new();
        // for i in 0..30000 {
        //     strings.push_str(&[(ryu::Buffer::new().format_finite(std::f64::consts::PI*(i as f64))).to_owned() + "`"].concat().replace(".", ","));
        // }
        // string = strings;

    }
    string
}

#[no_mangle]
pub fn rs_entry_point<'a>(ptr: *mut c_char) -> *mut c_char {
    let dotnet_str = Strs::from_ptr(ptr).to_str();
    println!("{}, crate num_cpus got {} physical cpu/s in your machine.", dotnet_str, num_cpus::get());
    println!("here are some numbers form rust: ");
    Strs::from_string(vec_f64_to_string()).as_ptr()
}
