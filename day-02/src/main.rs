use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(Path::new(&filename)).map_err(|e| e.to_string())?;

    let reports = parse(&content)?;

    let safe_reports = count_safe(&reports);
    println!("{safe_reports} are safe");

    Ok(())
}

fn count_safe(reports: &[Box<[u32]>]) -> usize {
    reports.iter().filter(|report| is_safe(report)).count()
}

fn is_safe(report: &[u32]) -> bool {
    (report.windows(2).all(|pair| pair[0] < pair[1])
        || report.windows(2).all(|pair| pair[1] < pair[0]))
        && report.windows(2).all(|pair| {
            let diff = pair[0].max(pair[1]) - pair[0].min(pair[1]);
            diff > 0 && diff < 4
        })
}

fn parse(input: &str) -> Result<Box<[Box<[u32]>]>, String> {
    input.lines().map(parse_report).collect()
}

fn parse_report(line: &str) -> Result<Box<[u32]>, String> {
    line.split_whitespace()
        .map(|s| {
            s.parse::<u32>()
                .map_err(|e| format!("unable to parse level '{s}': {e}"))
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    static input: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"#;

    #[test]
    fn count_safe_counts_correctly() {
        // given
        let reports = parse(input).expect("expected successful parsing");

        // when
        let count = count_safe(&reports);

        // then
        assert_eq!(count, 2);
    }
}
