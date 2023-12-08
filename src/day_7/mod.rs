use std::collections::HashMap;

fn score_map(score: char) -> i64 {
    let sm: HashMap<char, i64> = HashMap::from([
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('T', 10),
        ('J', 11),
        ('Q', 12),
        ('K', 13),
        ('A', 14),
    ]);
    sm[&score]
}

fn joker_scores(base_score: &(i64, i64, i64, i64, i64)) -> (i64, i64, i64, i64, i64) {
    let j = score_map('J');
    (
        if base_score.0 == j {1} else {base_score.0},
        if base_score.1 == j {1} else {base_score.1},
        if base_score.2 == j {1} else {base_score.2},
        if base_score.3 == j {1} else {base_score.3},
        if base_score.4 == j {1} else {base_score.4},
    )
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone, Copy)]
enum HandScore {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn get_score(cards: &Vec<i64>) -> HandScore {
    let mut card_map = HashMap::new();
    for card in cards {
        if card_map.contains_key(card) {
            card_map.insert(card, card_map[card] + 1);
        } else {
            card_map.insert(card, 1);
        }
    }
    let map_len = card_map.len();
    if map_len == 1 {
        return HandScore::FiveOfAKind;
    }
    if map_len == 5 {
        return HandScore::HighCard;
    }
    if map_len == 4 {
        return HandScore::OnePair;
    }
    if map_len == 3 {
        if *card_map.values().max().unwrap() == 2 {
            return HandScore::TwoPair;
        }
        return HandScore::ThreeOfAKind;
    }
    // map_len == 2
    if *card_map.values().max().unwrap() == 4 {
        return HandScore::FourOfAKind;
    }
    HandScore::FullHouse
}

fn get_joker_score(cards: &Vec<i64>) -> HandScore {
    let mut card_map = HashMap::new();
    for card in cards {
        if card_map.contains_key(card) {
            card_map.insert(card, card_map[card] + 1);
        } else {
            card_map.insert(card, 1);
        }
    }
    let map_len = card_map.len();
    let joker_key = score_map('J');
    let contains_joker = card_map.contains_key(&joker_key);
    let joker_value = if contains_joker {Some(card_map[&joker_key])} else {None};
    if map_len == 1 {
        return HandScore::FiveOfAKind;
    }
    if map_len == 5 {
        if contains_joker {
            return HandScore::OnePair;
        }
        return HandScore::HighCard;
    }
    if map_len == 4 {
        if contains_joker {
            // If the pair is a joker, then match anything to make it 3 of a kind, same if the pair is something else
            // Always better than two pair
            return HandScore::ThreeOfAKind;
        }
        // No joker, so just a pair
        return HandScore::OnePair;
    }
    if map_len == 3 {
        if *card_map.values().max().unwrap() == 2 {
            if contains_joker {
                if joker_value.unwrap() == 2 {
                    // Merge with the other pair to get 4
                    return HandScore::FourOfAKind;
                }
                // Just a single, so merge with a pair to get full house
                return HandScore::FullHouse;
            }
            // No joker, so just two pairs
            return HandScore::TwoPair;
        }
        if contains_joker {
            // Either is the 3 of a kind and can join, or is a single and can join the three, either is 4 of a kind
            return HandScore::FourOfAKind;
        }
        // No joker, so must be a three
        return HandScore::ThreeOfAKind;
    }
    // map_len == 2
    if contains_joker {
        // Two card types and one is joker, so must be five of a kind
        return HandScore::FiveOfAKind;
    }
    if *card_map.values().max().unwrap() == 4 {
        return HandScore::FourOfAKind;
    }
    HandScore::FullHouse
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Copy)]
struct Hand {
     outcome: HandScore,
     cards: (i64, i64, i64, i64, i64),
     bid: i64,
}

fn parse_line(line: String) -> (Hand, Hand) {
    let trimmed = line.trim();
    let split: Vec<&str> = trimmed.split_whitespace().collect();
    let card_string = split[0];
    let cards: Vec<i64> = card_string.chars().map(|x| score_map(x)).collect();
    if cards.len() != 5 {
        panic!("");
    }
    let cards_tuple = (cards[0], cards[1], cards[2], cards[3], cards[4]);
    let outcome = get_score(&cards);
    let joker_outcome = get_joker_score(&cards);
    let bid = split[1].parse::<i64>().unwrap();
    let hand = Hand { outcome, cards: cards_tuple, bid };
    let joker_hand = Hand { outcome: joker_outcome, cards: joker_scores(&cards_tuple), bid };
    (hand, joker_hand)
}

fn ranking_score(hands: &Vec<Hand>) -> i64 {
    let mut result = 0;
    for (i, hand) in hands.iter().enumerate() {
        let rank = 1 + (i as i64);
        result += rank * hand.bid;
    }
    result
}

pub fn solution() -> String {
    let contents = include_str!("input.txt");
    let mut card_sets = vec![];
    for line in contents.lines() {
        card_sets.push(parse_line(line.to_string()));
    }
    let (mut hands, mut joker_hands): (Vec<Hand>, Vec<Hand>) = card_sets.iter().cloned().unzip();
    hands.sort();
    let result_1 = ranking_score(&hands);
    joker_hands.sort();
    let result_2 = ranking_score(&joker_hands);
    format!("Problem 1: {result_1}\nProblem 2: {result_2}")
}
