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
    println!("The compacted hard drive's checksum is {fragmented_checksum}");

    let defragmented_checksum = defragment_disk(&original_disk);
    println!("The defragmented hard drive's checksum is {defragmented_checksum}");

    Ok(())
}

fn defragment_disk(original_disk: &[u8]) -> u64 {
    /*let mut compressed_disk: Box<[(u8, Option<u16>)]> = original_disk
    .iter()
    .enumerate()
    .map(|(i, size)| {
        (
            *size,
            if i % 2 == 0 {
                Some((i / 2) as u16)
            } else {
                None
            },
        )
    })
    .collect();*/
    let mut disk = decompress_disk(original_disk);
    let mut u: usize = disk.len() - 1;
    let mut max_id: u16 = (original_disk.len() / 2) as u16;
    while u > 0 {
        while disk[u].map(|id| id > max_id).unwrap_or(true) {
            u -= 1;
        }
        let mut ul = u;
        while ul > 0 && disk[ul] == disk[u] {
            ul -= 1;
        }
        let size = u - ul;
        if let Some(target_offset) = find_space(&disk[0..=ul], size) {
            for i in 0..size {
                disk[target_offset + i] = disk[ul + 1 + i];
                disk[ul + 1 + i] = None;
            }
        }
        max_id = max_id.max(1) - 1;
        u = ul;
    }

    disk_checksum(&disk)
}

// return offset of first space available that is large enough to fit len, or None if there is no
// such space
fn find_space(disk: &[Option<u16>], len: usize) -> Option<usize> {
    if len == 0 {
        return None;
    }
    let mut offset = 0;
    while offset + len <= disk.len() {
        if disk[offset].is_some() {
            offset += 1;
        } else {
            if (offset..(offset + len)).all(|i| disk[i].is_none()) {
                return Some(offset);
            }
            offset += 1;
        }
    }
    None
}

fn fragment_disk(original_disk: &[u8]) -> u64 {
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

    #[test]
    fn defragment_disk_works_for_example() {
        // given
        let original_disk = parse(EXAMPLE).expect("expected example input to parse");

        // when
        let checksum = defragment_disk(&original_disk);

        // then
        assert_eq!(checksum, 2858);
    }
}
