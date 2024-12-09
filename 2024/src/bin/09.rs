use std::{collections::VecDeque, io};

type DiskMap = VecDeque<(u8, Option<usize>)>;

fn checksum(disk: &DiskMap) -> usize {
    disk.iter()
        .scan(0_usize, |acc, (len, id)| {
            let len = *len as usize;
            let first = *acc;
            *acc = first + len;
            let last = first + len - 1;
            let sum = len * (first + last) / 2;
            Some(sum * id.unwrap_or_default())
        })
        .sum()
}

fn parse_diskmap(input: &str) -> DiskMap {
    let input: Vec<_> = input
        .chars()
        .filter_map(|c| c.to_digit(10).map(|n| n as u8))
        .collect();

    input
        .into_iter()
        .scan((true, 0usize), |acc: &mut (bool, usize), n| {
            let res = Some(if acc.0 { (n, Some(acc.1)) } else { (n, None) });
            *acc = (!acc.0, acc.1 + acc.0 as usize);
            res
        })
        .collect()
}

fn part1(input: &str) -> usize {
    let mut disk_map = parse_diskmap(input);
    let mut tail_pos = disk_map.len() - 1;
    let mut head_pos = 0;

    while head_pos < tail_pos {
        let head = disk_map[head_pos];
        if head.1.is_some() {
            head_pos += 1;
            continue;
        }

        let tail = disk_map[tail_pos];
        if head.0 <= tail.0 {
            let new_tail_len = tail.0 - head.0;
            if new_tail_len == 0 {
                disk_map[tail_pos] = (new_tail_len, None);
                while disk_map[tail_pos].1.is_none() {
                    tail_pos -= 1;
                }
            } else {
                disk_map[tail_pos] = (new_tail_len, tail.1)
            }
            disk_map[head_pos] = (head.0, tail.1);
        } else {
            let head_rem = head.0 - tail.0;
            disk_map.insert(head_pos + 1, (head_rem, None));
            tail_pos += 1;
            disk_map[head_pos] = (tail.0, tail.1);
            disk_map[tail_pos] = (0, None);
            while disk_map[tail_pos].1.is_none() {
                tail_pos -= 1;
            }
        }
        head_pos += 1;
    }

    checksum(&disk_map)
}

fn part2(input: &str) -> usize {
    let mut disk_map = parse_diskmap(input);

    let mut tail_pos = disk_map.len() - 1;

    while tail_pos > 0 {
        let (len, id) = disk_map[tail_pos];
        if id.is_none() {
            tail_pos -= 1;
            continue;
        }

        let Some((slot_pos, &(slot_len, _))) = disk_map
            .iter()
            .take(tail_pos)
            .enumerate()
            .find(|(_, (slot_len, id))| id.is_none() && *slot_len >= len)
        else {
            tail_pos -= 1;
            continue;
        };

        disk_map[tail_pos] = (len, None);
        if slot_len == len {
            disk_map[slot_pos] = (len, id);
            tail_pos -= 1;
        } else {
            let rem = slot_len - len;
            disk_map.insert(slot_pos + 1, (rem, None));
            disk_map[slot_pos] = (len, id);
        }
    }

    checksum(&disk_map)
}

fn main() -> io::Result<()> {
    let input = aoclib::read_input_string()?;

    let p1 = part1(&input);
    println!("Part 1: {}", p1);

    let p2 = part2(&input);
    println!("Part 2: {}", p2);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_real_input() {
        let input = std::fs::read_to_string(aoclib::get_test_input_file!(9)).unwrap();

        let p1 = part1(&input);
        assert_eq!(p1, 6399153661894);

        let p2 = part2(&input);
        assert_eq!(p2, 6421724645083);
    }
}
