use std::collections::HashSet;
use std::collections::VecDeque;
use std::hash::Hash;
use std::hash::Hasher;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("failed to parse card value")]
    CardParse {
        #[from]
        source: std::num::ParseIntError,
    },

    #[error("got player count {0}; want 2 players")]
    PlayerCount(usize),
}

type Result<T> = std::result::Result<T, Error>;

type Card = usize;

pub enum Player {
    One,
    Two,
}

enum WinType {
    None,
    Normal(Player),
    Recursive,
}

#[derive(Clone, Hash)]
struct Deck {
    cards: VecDeque<Card>,
}

#[derive(Hash)]
pub struct Combat {
    player_1: Deck,
    player_2: Deck,
}

impl std::str::FromStr for Deck {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let cards = s
            .trim()
            .lines()
            .map(|l| l.parse::<Card>())
            .collect::<std::result::Result<VecDeque<_>, _>>()?;
        Ok(Deck { cards })
    }
}

impl std::str::FromStr for Combat {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let players = s
            .trim()
            .split("\n\n")
            .map(|player| {
                player
                    .lines()
                    .skip(1)
                    .collect::<Vec<_>>()
                    .join("\n")
                    .parse::<Deck>()
            })
            .collect::<Result<Vec<_>>>()?;
        if players.len() != 2 {
            Err(Error::PlayerCount(players.len()))
        } else {
            Ok(Combat {
                player_1: players[0].clone(),
                player_2: players[1].clone(),
            })
        }
    }
}

impl Deck {
    fn score(&self) -> usize {
        let mut sum = 0;
        for (i, card) in self.cards.iter().enumerate() {
            sum += (self.cards.len() - i) * card;
        }
        sum
    }
}

impl Combat {
    fn get_hash(&self) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }

    fn step(&mut self) -> Option<Player> {
        let card_1 = self.player_1.cards.pop_front().unwrap();
        let card_2 = self.player_2.cards.pop_front().unwrap();
        if card_1 > card_2 {
            self.player_1.cards.push_back(card_1);
            self.player_1.cards.push_back(card_2);
        } else {
            self.player_2.cards.push_back(card_2);
            self.player_2.cards.push_back(card_1);
        }
        if self.player_1.cards.is_empty() {
            Some(Player::Two)
        } else if self.player_2.cards.is_empty() {
            Some(Player::One)
        } else {
            None
        }
    }

    fn step_recursive(&mut self, game_state_seen: &mut HashSet<u64>) -> WinType {
        match game_state_seen.insert(self.get_hash()) {
            true => (),
            false => return WinType::Recursive,
        };
        let card_1 = self.player_1.cards.pop_front().unwrap();
        let card_2 = self.player_2.cards.pop_front().unwrap();
        if card_1 <= self.player_1.cards.len() && card_2 <= self.player_2.cards.len() {
            let mut new_game = Combat {
                player_1: Deck {
                    cards: self
                        .player_1
                        .cards
                        .iter()
                        .cloned()
                        .take(card_1)
                        .collect::<VecDeque<_>>(),
                },
                player_2: Deck {
                    cards: self
                        .player_2
                        .cards
                        .iter()
                        .cloned()
                        .take(card_2)
                        .collect::<VecDeque<_>>(),
                },
            };
            let (winner, _score) = new_game.play_recursive();
            match winner {
                Player::One => {
                    self.player_1.cards.push_back(card_1);
                    self.player_1.cards.push_back(card_2);
                }
                Player::Two => {
                    self.player_2.cards.push_back(card_2);
                    self.player_2.cards.push_back(card_1);
                }
            }
        } else if card_1 > card_2 {
            self.player_1.cards.push_back(card_1);
            self.player_1.cards.push_back(card_2);
        } else {
            self.player_2.cards.push_back(card_2);
            self.player_2.cards.push_back(card_1);
        }
        if self.player_1.cards.is_empty() {
            WinType::Normal(Player::Two)
        } else if self.player_2.cards.is_empty() {
            WinType::Normal(Player::One)
        } else {
            WinType::None
        }
    }

    pub fn play(&mut self) -> (Player, usize) {
        loop {
            match self.step() {
                None => (),
                Some(Player::One) => return (Player::One, self.player_1.score()),
                Some(Player::Two) => return (Player::Two, self.player_2.score()),
            }
        }
    }

    pub fn play_recursive(&mut self) -> (Player, usize) {
        let mut game_state_seen = HashSet::new();
        loop {
            match self.step_recursive(&mut game_state_seen) {
                WinType::None => (),
                WinType::Normal(Player::One) => return (Player::One, self.player_1.score()),
                WinType::Normal(Player::Two) => return (Player::Two, self.player_2.score()),
                WinType::Recursive => return (Player::One, self.player_1.score()),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_GAME: &str = r"Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";

    #[test]
    fn test_play_score() {
        let mut game = TEST_GAME.parse::<Combat>().unwrap();
        assert_eq!(306, game.play().1);
    }

    #[test]
    fn test_play_recursive() {
        let mut game = TEST_GAME.parse::<Combat>().unwrap();
        assert_eq!(291, game.play_recursive().1);
    }
}
