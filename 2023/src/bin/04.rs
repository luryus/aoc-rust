use std::{collections::HashMap, collections::HashSet, io};

struct Card {
    id: usize,
    winning: HashSet<u16>,
    card_nums: HashSet<u16>,
}

fn part1(input: &[Card]) -> usize {
    input
        .iter()
        .map(|c| match c.card_nums.intersection(&c.winning).count() > 0 {
            true => 2usize.pow((c.card_nums.intersection(&c.winning).count() - 1) as u32),
            false => 0,
        })
        .sum()
}

fn part2(input: &Vec<Card>) -> usize {
    let mut card_counts: HashMap<usize, usize> = input.iter().map(|c| (c.id, 1)).collect();
    for i in 1..=input.len() {
        let this_card_count = card_counts[&i];
        let this_card = &input[i - 1];
        let matches = this_card.card_nums.intersection(&this_card.winning).count();
        if matches > 0 {
            for j in (i + 1)..=(i + matches) {
                *card_counts.get_mut(&j).unwrap() += this_card_count;
            }
        }
    }

    card_counts.values().sum()
}

fn parse_input(input: Vec<String>) -> Vec<Card> {
    input
        .into_iter()
        .map(|l| {
            let (head, tail) = aoclib::split_to_tuple2(&l, ": ").unwrap();
            let (win_str, num_str) = aoclib::split_to_tuple2(tail, "|").unwrap();

            let id = *aoclib::read_ints_from_string(head, false).first().unwrap();
            let winning = aoclib::read_ints_from_string(win_str, false)
                .into_iter()
                .collect();
            let card_nums = aoclib::read_ints_from_string(num_str, false)
                .into_iter()
                .collect();

            Card {
                id,
                winning,
                card_nums,
            }
        })
        .collect()
}

fn main() -> io::Result<()> {
    let input = aoclib::read_input_lines()?;
    let input = parse_input(input);

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
        let input = aoclib::read_file_lines(aoclib::get_test_input_file!(4)).unwrap();
        let input = parse_input(input);

        let p1 = part1(&input);
        assert_eq!(p1, 23750);

        let p2 = part2(&input);
        assert_eq!(p2, 13261850);
    }
}
