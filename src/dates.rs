//get time
use chrono::prelude::*;

pub fn calculate_dates(
    output: String,
    start_str: &str,
    end_str: &str,
) -> (String, chrono::Duration) {
    //get build date
    let start_bd = output.find(start_str).unwrap_or(0);
    let end_bd = output.find(end_str).unwrap_or(output.len());
    let build_date = &output[start_bd + 14..end_bd].trim();

    //convert str to Datetime (Naive means no particular timezone)
    let dt = NaiveDateTime::parse_from_str(build_date, "%a %d %b %Y %r").unwrap();
    /*
    let dt:NaiveDateTime = match dt {
        Ok(v) => v,
        Err(e) => e,
    };*/
    //https://stackoverflow.com/questions/49487722/how-do-i-get-duration-from-two-datetime-naivedatetime
    let other_dt = DateTime::<Utc>::from_utc(dt, Utc);
    let now = Local::now();
    let diff = now.signed_duration_since(other_dt);

    (build_date.to_string(), diff)
}
pub fn calc_pkg_dates(build_date: &str, original_repo_date: &str) -> chrono::Duration {
    let build_date = NaiveDate::parse_from_str(build_date, "%a %d %b %Y %r").unwrap();
    let ord = NaiveDate::parse_from_str(original_repo_date, "%b %d, %Y").unwrap();

    //https://stackoverflow.com/questions/49487722/how-do-i-get-duration-from-two-datetime-naivedatetime
    //let other_dt = DateTime::<Utc>::from_utc(dt, Utc);
    //let now = Local::now();
    let diff = build_date.signed_duration_since(ord);

    diff
}
