use crate::register::gtp::GTP_STATE;

pub fn update_gtp() {
    let mut state = GTP_STATE.lock().unwrap();
    state.refresh = true;
}
pub fn set_csv_path(path: String) {
    let mut state = GTP_STATE.lock().unwrap();
    state.csv_path = path;
}