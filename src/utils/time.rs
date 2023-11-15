use chrono::{DateTime, Utc, TimeZone,FixedOffset};

pub fn current_time() -> String {
    let current_utc_time: DateTime<Utc> = Utc::now();
    
    // Specify the fixed offset for Bangkok time (UTC+7)
    let bangkok_offset = TimeZone::from_offset(&FixedOffset::east_opt(7*3600).unwrap()); // 7 hours * 3600 seconds per hour

    // Convert the UTC time to Bangkok time
    let bangkok_time: DateTime<FixedOffset> = current_utc_time.with_timezone(&bangkok_offset);

    // Format the time for display
    let formatted_time = bangkok_time.format("%Y-%m-%d %H:%M:%S %z");

    // Return the formatted time as a string
    formatted_time.to_string()
}