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
    let mut pairs = 0;
    let mut three_of_a_kind = false;
    let mut four_of_a_kind = false;
    let mut five_of_a_kind = false;
    for c in cards.chars() {
        match c {
            '2'..='9' => counts[c as usize - '2' as usize] += 1,
            'a'..='e' => counts[c as usize - 'a' as usize + 8] += 1,
            _ => {}
        }
    }
    for &count in counts.iter() {
        match count {
            2 => pairs += 1,
            3 => three_of_a_kind = true,
            4 => four_of_a_kind = true,
            5 => five_of_a_kind = true,
            _ => {}
        }
    }
    if five_of_a_kind {
        6
    } else if four_of_a_kind {
        5
    } else if three_of_a_kind && pairs == 1 {
        4
    } else if three_of_a_kind {
        3
    } else if pairs == 2 {
        2
    } else if pairs == 1 {
        1
    } else {
        0
    }
}

fn main() {
    let mut hands: Vec<_> = include_str!("../input").trim().lines().map(|line| {
        let (hand, bid) = line.split_once(" ").unwrap();
        let hand = hand
            .replace("T", "a")
            .replace("J", "b")
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
