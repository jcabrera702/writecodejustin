fn  total_hours(values: &[i32]) -> i32 {
    values.iter().sum()
}
fn average_hours(values: &[i32]) -> f32{
    if values.is_empty() {
        return 0.0;
    }
    let total: i32 = values.iter().sum();
    total as f32 / values.len() as f32
}
fn peak_hours(values: &[i32]) -> i32 {
    *values.iter().max().unwrap_or(&0)  
}
fn main() {
    let weekly_schedule = [3,6,2,7,4,1,5];
    // println!("Total hours: {}", total_hours(&weekly_schedule));
    // println!("Average hours: {:.2}", average_hours(&weekly_schedule));
    // println!("Peak hours: {}", peak_hours(&weekly_schedule));
    print!("Weekday hours Mon-Fri: ");
    for i in &weekly_schedule[0..5]{
        print!("{i} ");
    }
}
