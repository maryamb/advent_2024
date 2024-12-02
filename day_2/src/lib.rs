use std::fs::File;
use std::io::{self, BufRead};

type AllReports = Vec<Vec<i32>>;

fn read_file_and_store_reports(file_path: &str) -> io::Result<AllReports> {
    // Open the file
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    // Create the output vector
    let output: AllReports = reader.lines().map(|line| {
        let line: String = line.expect("");
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        numbers
    }).collect();
    Ok(output)
}

fn is_report_safe(report: &Vec<i32>) -> bool {
    let deltas: Vec<i32> = report
    .windows(2)
    .map(|pair| pair[1] - pair[0])
    .collect();
    deltas.iter().all(|element| {
        let element = element.abs();
        element >= 1 && element <= 3}) && (
            deltas.iter().all(|&a| a <= 0) || deltas.iter().all(|&a| a >= 0))
}

fn could_report_be_safe(report: &Vec<i32>) -> bool {
    // If the report is already safe, return true
    if is_report_safe(report) {
        return true;
    }
    
    // Try removing each level and check if the resulting report is safe
    for i in 0..report.len() {
        // Create a new report with the i-th element removed
        let mut modified_report: Vec<i32> = report.clone();
        modified_report.remove(i);
        
        // If the modified report is safe, return true
        if is_report_safe(&modified_report) {
            return true;
        }
    }
    
    // If no modification makes the report safe, return false
    false
}

pub fn count_safe_reports_in_file(file_path: &str) -> usize {
    let reports: AllReports = read_file_and_store_reports(file_path).expect("");
    reports.iter().filter(|&report| is_report_safe(report)).count()
}

pub fn count_potentially_safe_reports_in_file(file_path: &str) -> usize {
    let reports: AllReports = read_file_and_store_reports(file_path).expect("");
    reports.iter().filter(|&report| could_report_be_safe(report)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(count_safe_reports_in_file("data/example_1.txt"), 2);
        assert_eq!(count_safe_reports_in_file("data/input_1.txt"), 224);
        assert_eq!(count_potentially_safe_reports_in_file("data/example_1.txt"), 4);
        assert_eq!(count_potentially_safe_reports_in_file("data/input_1.txt"), 293);
    }
}
