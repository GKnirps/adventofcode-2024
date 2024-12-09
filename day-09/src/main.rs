use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(Path::new(&filename)).map_err(|e| e.to_string())?;
    let original_disk = parse(&content)?;

    let fragmented_checksum = fragment_disk(&original_disk);
    println!("The compacted hard drive's checksum os {fragmented_checksum}");

    Ok(())
}

fn fragment_disk(original_disk: &[u8]) -> u64 {
    // let's do the brute force again. This will probably byte me in the ass in part 2, but
    // whatever
    let mut disk = decompress_disk(original_disk);
    let mut l: usize = 0;
    let mut u: usize = disk.len() - 1;

    while l < u {
        if disk[u].is_some() {
            if disk[l].is_none() {
                disk[l] = disk[u];
                disk[u] = None;
                u -= 1;
                l += 1;
            } else {
                l += 1;
            }
        } else {
            u -= 1;
        }
    }
    disk_checksum(&disk)
}

fn disk_checksum(disk: &[Option<u16>]) -> u64 {
    disk.iter()
        .enumerate()
        .filter_map(|(i, id)| {
            let id = (*id)?;
            Some((i, id))
        })
        .map(|(i, id)| i as u64 * id as u64)
        .sum()
}

fn decompress_disk(compressed: &[u8]) -> Box<[Option<u16>]> {
    let mut decompressed: Vec<Option<u16>> = Vec::with_capacity(10 * compressed.len());
    let mut id_count: u16 = 0;
    let mut empty_section = false;

    for section_length in compressed {
        if empty_section {
            for _ in 0..*section_length {
                decompressed.push(None);
            }
            empty_section = false;
        } else {
            for _ in 0..*section_length {
                decompressed.push(Some(id_count));
            }
            id_count += 1;
            empty_section = true;
        }
    }

    decompressed.into_boxed_slice()
}

fn parse(input: &str) -> Result<Box<[u8]>, String> {
    input
        .trim()
        .chars()
        .map(|c| {
            c.to_digit(10)
                .map(|d| d as u8)
                .ok_or_else(|| format!("unable to parse digit '{c}'"))
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = "2333133121414131402\n";

    #[test]
    fn fragment_disk_works_for_example() {
        // given
        let original_disk = parse(EXAMPLE).expect("expected example input to parse");

        // then
        let checksum = fragment_disk(&original_disk);

        // then
        assert_eq!(checksum, 1928);
    }
}
