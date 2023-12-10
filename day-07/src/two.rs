use std::{cmp::Ordering, mem};

pub fn run(input: &str) {
    let mut parser = utils::Parser::new(input);

    let mut hands = Vec::new();
    parser.sep_by("\n", |parser| {
        let hand = parser
            .inner
            .by_ref()
            .map(|(_, c)| c)
            .take(5)
            .collect::<Vec<_>>();
        parser.expect(" ");
        let bid = parser.int();

        hands.push((hand, bid));
    });

    let mut sort_buffer = Vec::new();
    let mut cmp_buffer = Vec::new();
    merge_sort(&mut hands, &mut sort_buffer, &mut cmp_buffer);

    let mut sum = 0;
    for (rank, (_, bid)) in hands.into_iter().enumerate() {
        sum += bid * (rank as u64 + 1);
    }

    println!("Two: {sum}");
}

type Hand = (Vec<char>, u64);

fn merge_sort(slice: &mut [Hand], sort_buf: &mut Vec<Hand>, cmp_buf: &mut Vec<(char, u64)>) {
    match slice.len() {
        0 | 1 => (),
        2 => {
            if let Ordering::Greater = compare(&slice[0], &slice[1], cmp_buf) {
                slice.swap(0, 1);
            }
        }
        len => {
            let pivot = len / 2;
            let (left, right) = slice.split_at_mut(pivot);
            merge_sort(left, sort_buf, cmp_buf);
            merge_sort(right, sort_buf, cmp_buf);

            sort_buf.clear();
            recurse(left, right, sort_buf, cmp_buf);

            fn recurse(
                left: &mut [Hand],
                right: &mut [Hand],
                sort_buf: &mut Vec<Hand>,
                cmp_buf: &mut Vec<(char, u64)>,
            ) {
                let ((cards, bid), (left, right)) =
                    match (left.split_first_mut(), right.split_first_mut()) {
                        (Some((l, l_rest)), Some((r, r_rest))) => match compare(l, r, cmp_buf) {
                            Ordering::Less | Ordering::Equal => (l, (l_rest, right)),
                            Ordering::Greater => (r, (left, r_rest)),
                        },
                        (Some((l, l_rest)), None) => (l, (l_rest, right)),
                        (None, Some((r, r_rest))) => (r, (left, r_rest)),
                        (None, None) => return,
                    };

                sort_buf.push((mem::take(cards), *bid));
                recurse(left, right, sort_buf, cmp_buf);
            }

            for ((cards, bid), dest) in sort_buf.iter_mut().zip(slice.iter_mut()) {
                *dest = (mem::take(cards), *bid);
            }
        }
    }
}

fn compare(lhs: &Hand, rhs: &Hand, buffer: &mut Vec<(char, u64)>) -> Ordering {
    let (l_val, r_val) = (identify(lhs, buffer), identify(rhs, buffer));

    match l_val.cmp(&r_val) {
        Ordering::Equal => {
            static VALUES: [char; 13] = [
                'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
            ];

            for (l, r) in lhs.0.iter().zip(rhs.0.iter()) {
                let (l, r) = VALUES
                    .iter()
                    .position(|c| c == l)
                    .zip(VALUES.iter().position(|c| c == r))
                    .unwrap();

                match l.cmp(&r) {
                    Ordering::Equal => continue,
                    ord => return ord,
                }
            }

            Ordering::Equal
        }
        ord => ord,
    }
}

fn identify(hand: &Hand, buffer: &mut Vec<(char, u64)>) -> u64 {
    buffer.clear();

    for card in hand.0.iter().copied() {
        if card == 'J' {
            continue;
        } else if let Some((_, count)) = buffer.iter_mut().find(|(c, _)| *c == card) {
            *count += 1;
        } else {
            buffer.push((card, 1));
        }
    }

    match buffer.as_slice() {
        [_] | [] => 7, // Five of a kind
        [a, b] => match (a.1, b.1) {
            (1, _) | (_, 1) => 6, // Four of a kind
            (2, _) | (_, 2) => 5, // Full house
            _ => panic!(),
        },
        [a, b, c] => match (a.1, b.1, c.1) {
            (_, 1, 1) | (1, _, 1) | (1, 1, _) => 4, // Three of a kind
            (2, 2, 1) | (1, 2, 2) | (2, 1, 2) => 3, // Two pair
            _ => panic!(),
        },
        cards if cards.len() == 4 => 2, // One pair
        cards if cards.len() == 5 => 1, // High card
        _ => panic!(),
    }
}
