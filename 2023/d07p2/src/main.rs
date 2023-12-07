#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: String,
    bid: usize,
    tipe: usize,
}

impl Hand {
    fn new(cards: String, bid: usize) -> Self {
        let tipe = type_of_hand(cards.as_str());
        Hand { cards, bid, tipe }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.tipe > other.tipe {
            std::cmp::Ordering::Greater
        } else if self.tipe < other.tipe {
            std::cmp::Ordering::Less
        } else if self.cards > other.cards {
            std::cmp::Ordering::Greater
        } else if self.cards < other.cards {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Equal
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn type_of_hand(cards: &str) -> usize {
    let mut counts = [0; 13];
    let mut jokers = 0;
    for c in cards.chars() {
        match c {
            '2'..='9' => counts[c as usize - '2' as usize] += 1,
            'a'..='e' => counts[c as usize - 'a' as usize + 8] += 1,
            '1' => jokers += 1,
            _ => {}
        }
    }
    let pairs = counts.iter().filter(|&c| *c == 2).count();
    let max_count = *counts.iter().max().unwrap() as usize;
    
    if max_count + jokers == 5 {
        6
    } else if max_count + jokers == 4 {
        5
    } else if max_count == 3 && pairs == 1 || pairs == 2 && jokers == 1 {
        4
    } else if max_count + jokers == 3 {
        3
    } else if pairs == 2 {
        2
    } else if max_count + jokers == 2 {
        1
    } else {
        0
    }
}

pub fn main() {
    let mut hands: Vec<_> = include_str!("../input").trim().lines().map(|line| {
        let (hand, bid) = line.split_once(" ").unwrap();
        let hand = hand
            .replace("T", "a")
            .replace("J", "1")
            .replace("Q", "c")
            .replace("K", "d")
            .replace("A", "e");
        let bid = bid.parse::<usize>().unwrap();
        Hand::new(hand, bid)
    }).collect();

    hands.sort();

    let mut res = 0;
    for (i, hand) in hands.iter().enumerate() {
        res += (i+1) * hand.bid;
    }

    println!("res: {res}");
}
