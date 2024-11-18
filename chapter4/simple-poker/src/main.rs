#[derive(Debug, Clone, Copy, PartialEq)]
enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Card {
    suit: Suit,
    rank: i32,
}

use rand::seq::SliceRandom;
fn main() {
    let mut deck: Vec<Card> = Vec::new();
    let suits = [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade];

    for suit in suits {
        for rank in 1..=13 {
            deck.push(Card { suit, rank });
        }
    }
    let mut rng = rand::thread_rng();
    deck.shuffle(&mut rng);

    // 手札
    let mut hand: Vec<Card> = Vec::new();
    // 5枚引く
    for _ in 0..5 {
        hand.push(deck.pop().unwrap());
    }
    // 手札をソート
    hand.sort_by(|a, b| a.rank.cmp(&b.rank));
    // 手札を表示
    println!("---hand---");
    for (i, card) in hand.iter().enumerate() {
        println!("{:}: {:?} {:}", i + 1, card.suit, card.rank)
    }

    // 手札交換
    println!("入れ替えたいカードの番号を入力してください(例: 1 2 3)");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let numbers: Vec<usize> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<usize>>();
    for number in numbers {
        hand[number - 1] = deck.pop().unwrap();
    }
    hand.sort_by(|a, b| a.rank.cmp(&b.rank));
    println!("---hand---");
    for (i, card) in hand.iter().enumerate() {
        println!("{:}: {:?} {:}", i + 1, card.suit, card.rank)
    }

    // 役判定
    let suit = hand.first().unwrap().suit;
    let flash = hand.iter().all(|c| c.suit == suit);
    let mut count = 0;
    for i in 0..hand.len() - 1 {
        for j in i + 1..hand.len() {
            if hand[i].rank == hand[j].rank {
                count += 1;
            }
        }
    }

    if flash {
        println!("フラッシュ!");
    } else if count >= 3 {
        println!("スリーカード!")
    } else if count == 2 {
        println!("2ペア!");
    } else if count == 1 {
        println!("1ペア!");
    } else {
        println!("役なし...")
    }
}
