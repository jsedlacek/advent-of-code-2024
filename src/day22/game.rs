use std::collections::HashMap;

pub fn part1(numbers: &[u64]) -> u64 {
    numbers
        .iter()
        .flat_map(|&number| Sequence::new(number).skip(2000).take(1))
        .sum()
}

pub fn part2(numbers: &[u64]) -> Option<u64> {
    let mut map = HashMap::new();

    let sequences = numbers
        .iter()
        .map(|&number| {
            let sequence = Sequence::new(number)
                .map(|number| (number % 10) as u8)
                .take(2001)
                .collect::<Vec<_>>();

            let diffs = sequence
                .windows(2)
                .map(|pair| pair[1] as i8 - pair[0] as i8)
                .collect::<Vec<_>>();

            (sequence, diffs)
        })
        .collect::<Vec<_>>();

    for (sequence, diffs) in sequences.iter() {
        let mut number_map = HashMap::new();

        for (i, number) in sequence.iter().enumerate().skip(4) {
            let prev_diffs = &diffs[i - 4..i];

            number_map.entry(prev_diffs).or_insert(*number as u64);
        }

        for (&prefix, &price) in &number_map {
            *map.entry(prefix).or_insert_with(|| 0) += price;
        }
    }

    map.iter().map(|(_, number)| *number).max()
}

fn next_secret_number(secret_number: u64) -> u64 {
    let secret_number = mix(secret_number, secret_number * 64);
    let secret_number = prune(secret_number);

    let secret_number = mix(secret_number, secret_number / 32);
    let secret_number = prune(secret_number);

    let secret_number = mix(secret_number, secret_number * 2048);
    let secret_number = prune(secret_number);

    secret_number
}

fn mix(a: u64, b: u64) -> u64 {
    a ^ b
}

fn prune(a: u64) -> u64 {
    a % 16777216
}

struct Sequence {
    secret_number: u64,
}

impl Sequence {
    fn new(secret_number: u64) -> Self {
        Self { secret_number }
    }
}

impl Iterator for Sequence {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let secret_number = self.secret_number;
        self.secret_number = next_secret_number(secret_number);
        Some(secret_number)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_secret_number() {
        assert_eq!(next_secret_number(123), 15887950);
    }

    #[test]
    fn test_sequence() {
        assert_eq!(Sequence::new(1).skip(2000).take(1).sum::<u64>(), 8685429);
        assert_eq!(Sequence::new(10).skip(2000).take(1).sum::<u64>(), 4700978);
        assert_eq!(Sequence::new(100).skip(2000).take(1).sum::<u64>(), 15273692);
        assert_eq!(Sequence::new(2024).skip(2000).take(1).sum::<u64>(), 8667524);
    }

    #[test]
    fn test_mix() {
        assert_eq!(mix(42, 15), 37);
    }

    #[test]
    fn test_prune() {
        assert_eq!(prune(100000000), 16113920);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&[1, 10, 100, 2024]), 37327623);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&[1, 2, 3, 2024]).unwrap(), 23);
    }
}
