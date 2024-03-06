use crate::sync::up::UPSafeCell;
use core::slice::from_raw_parts;

const MAX_APP_NUM: usize = 16;

struct AppManager {
    num_app: usize,
    current_app: usize,
    app_start: [usize; MAX_APP_NUM + 1],
}

fn app_data_init() -> AppManager {
    extern "C" {
        fn _num_app();
    }
    unsafe {
        let num_app_ptr = _num_app as *const usize;
        let num_app = num_app_ptr.read_volatile();
        let mut app_start: [usize; MAX_APP_NUM + 1] = [0; MAX_APP_NUM + 1];
        let app_start_raw: &[usize] = from_raw_parts(num_app_ptr.add(1), num_app + 1);
        app_start[..=num_app].copy_from_slice(app_start_raw);
        AppManager {
            num_app,
            current_app: 0,
            app_start,
        }
    }
}

impl AppManager {
    pub fn print_app_info(&self) {
        println!("[kernel] num_app = {}", self.num_app);
        for i in 0..self.num_app {
            println!(
                "[kernel] app_{} [{:#x}, {:#x})",
                i,
                self.app_start[i],
                self.app_start[i + 1]
            );
        }
    }

    pub fn move_to_next_app(&mut self) {
        self.current_app += 1;
    }

    pub fn get_current_app(&self) -> usize {
        self.current_app
    }

    fn load_app(&self, app_id: usize) {
        //todo
    }
}

lazy_static! {
    static ref APP_MANAGER: UPSafeCell<AppManager> = UPSafeCell::new(app_data_init());
}

pub fn init() {
    APP_MANAGER.exclusive_access().print_app_info();
}

pub fn run_next_app() -> ! {
    //todo
}
