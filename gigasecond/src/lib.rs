use time::ext::NumericalDuration;
use time::PrimitiveDateTime as DateTime;
use time::{convert, Duration};

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let gigasecond = Duration::seconds(1_000_000_000);
    let seconds = gigasecond.as_seconds_f64().seconds().whole_seconds();
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let seconds = (seconds % 3600) % 60;
    let end = start + Duration::hours(hours) + Duration::minutes(minutes) + Duration::seconds(seconds);
    end
}