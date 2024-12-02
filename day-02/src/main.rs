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

    let dampened_safe_reports = count_dampened_safe(&reports);
    println!("With the problem dampener {dampened_safe_reports} reports are safe.");

    Ok(())
}

fn count_safe(reports: &[Box<[u32]>]) -> usize {
    reports.iter().filter(|report| is_safe(report)).count()
}

fn is_safe(report: &[u32]) -> bool {
    let (c1, c2) = dampener_candidates(report);
    c1.is_none() || c2.is_none()
}

fn dampener_candidates(report: &[u32]) -> (Option<usize>, Option<usize>) {
    let candidate_1 = report
        .windows(2)
        .enumerate()
        .filter(|(_, pair)| pair[0] >= pair[1] || pair[1] - pair[0] > 3)
        .map(|(i, _)| i)
        .next();
    let candidate_2 = report
        .windows(2)
        .enumerate()
        .filter(|(_, pair)| pair[1] >= pair[0] || pair[0] - pair[1] > 3)
        .map(|(i, _)| i)
        .next();
    (candidate_1, candidate_2)
}

fn count_dampened_safe(reports: &[Box<[u32]>]) -> usize {
    reports
        .iter()
        .filter(|report| is_dampened_safe(report))
        .count()
}

fn is_dampened_safe(report: &[u32]) -> bool {
    let (c1, c2) = dampener_candidates(report);
    if c1.is_none() || c2.is_none() {
        return true;
    }
    c1.iter()
        .flat_map(|i| [*i, i + 1])
        .chain(c2.iter().flat_map(|i| [*i, i + 1]))
        .any(|unsafe_i| {
            let dampened_report: Box<[u32]> = report
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != unsafe_i)
                .map(|(_, level)| *level)
                .collect();
            is_safe(&dampened_report)
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

    static INPUT: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"#;

    #[test]
    fn count_safe_counts_correctly() {
        // given
        let reports = parse(INPUT).expect("expected successful parsing");

        // when
        let count = count_safe(&reports);

        // then
        assert_eq!(count, 2);
    }

    #[test]
    fn count_dampened_safe_counts_correctly() {
        // given
        let reports = parse(INPUT).expect("expected successful parsing");

        // when
        let count = count_dampened_safe(&reports);

        // then
        assert_eq!(count, 4);
    }

    #[test]
    fn is_dampened_safe_works_if_first_index_needs_to_be_removed() {
        // given
        let report = &[1, 2, 4, 3, 4];

        // when
        let safe = is_dampened_safe(report);

        // then
        assert!(safe);
    }

    #[test]
    fn is_dampened_safe_works_if_first_report_is_in_wrong_order() {
        // given
        let report = &[2, 1, 2, 3, 4];

        // when
        let safe = is_dampened_safe(report);

        // then
        assert!(safe);
    }

    #[test]
    fn is_dampened_safe_works_if_first_report_is_too_far_away() {
        // given
        let report = &[1, 8, 9, 10];

        // when
        let safe = is_dampened_safe(report);

        // then
        assert!(safe);
    }

    #[test]
    fn is_dampened_safe_works_if_last_report_is_in_wrong_order() {
        // given
        let report = &[1, 2, 3, 4, 3];

        // when
        let safe = is_dampened_safe(report);

        // then
        assert!(safe);
    }

    #[test]
    fn is_dampened_safe_works_if_last_report_is_too_far_away() {
        // given
        let report = &[1, 2, 3, 4, 8];

        // when
        let safe = is_dampened_safe(report);

        // then
        assert!(safe);
    }
}
