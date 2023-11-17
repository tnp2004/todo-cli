use crate::utils::time;

pub fn get_file_name() -> String {
    let file_name = format!("Export-{}.csv", time::get_time_name());

    file_name.to_string()
}
