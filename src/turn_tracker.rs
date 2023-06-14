use crate::gametraits::User;
use itertools::enumerate;
use itertools::Itertools;
use log::debug;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TurnTracker {
    players: Vec<User>,
    next_player_index: usize,
}

impl TurnTracker {
    fn player_string(&self) -> String {
        let mut players: String = String::new();
        for (i, User { name, .. }) in enumerate(&self.players) {
            if i == self.next_player_index {
                players += format!("*{name}").as_str();
            } else {
                players += name.to_string().as_str();
            }
        }
        players
    }

    pub fn new(players: Vec<User>) -> Self {
        debug!("Creating turn tracker, with users {players:?}");
        Self {
            players,
            next_player_index: 0,
        }
    }

    pub fn remove_player(&mut self, username: &str) {
        let p_str = self.player_string();
        debug!("Removing player {username}, left: {p_str}");
        let (i, _) = self
            .players
            .iter()
            .find_position(|u| u.name == username)
            .unwrap();

        if i <= self.next_player_index {
            // Remove player earlier in the list
            if i < self.next_player_index {
                self.next_player_index -= 1;
            } else if self.next_player_index == self.players.len() - 1 {
                self.next_player_index = 0;
            }
        }
        self.players = self
            .players
            .iter()
            .filter(|u| u.name != username)
            .map(Clone::clone)
            .collect();
    }

    pub fn add_player(&mut self, user: User) {
        let p_str = self.player_string();
        debug!("Adding player {user:?}, new: {p_str}");
        self.players.push(user);
    }

    pub fn advance_player(&mut self) -> Option<User> {
        let p_str = self.player_string();
        debug!("Advancing player, new: {p_str}");
        if self.players.is_empty() {
            return None;
        }
        let current_index = self.next_player_index;
        self.next_player_index = (self.next_player_index + 1) % self.players.len();
        self.players.get(current_index).map(Clone::clone)
    }

    pub fn num_players(&self) -> usize {
        self.players.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn make_player(name: &str) -> User {
        User {
            name: name.to_string(),
            color: druid::piet::Color::BLUE,
        }
    }

    #[test]
    fn construct_and_loop() {
        let p1 = make_player("p1");
        let p2 = make_player("p2");
        let p3 = make_player("p3");
        let mut t = TurnTracker::new(vec![p1.clone(), p2.clone(), p3.clone()]);

        for _ in 1..50 {
            assert_eq!(t.advance_player(), Some(p1.clone()));
            assert_eq!(t.advance_player(), Some(p2.clone()));
            assert_eq!(t.advance_player(), Some(p3.clone()));
        }
    }

    #[test]
    fn add_one() {
        let p1 = make_player("p1");
        let p2 = make_player("p2");
        let mut t = TurnTracker::new(vec![p1.clone(), p2.clone()]);

        assert_eq!(t.advance_player(), Some(p1.clone()));
        let p3 = make_player("p3");
        t.add_player(p3.clone());
        for _ in 1..10 {
            assert_eq!(t.advance_player(), Some(p2.clone()));
            assert_eq!(t.advance_player(), Some(p3.clone()));
            assert_eq!(t.advance_player(), Some(p1.clone()));
        }
    }

    #[test]
    fn add_two() {
        let p1 = make_player("p1");
        let mut t = TurnTracker::new(vec![p1.clone()]);

        assert_eq!(t.advance_player(), Some(p1.clone()));
        let p2 = make_player("p2");
        let p3 = make_player("p3");
        t.add_player(p2.clone());
        t.add_player(p3.clone());
        for _ in 1..10 {
            assert_eq!(t.advance_player(), Some(p1.clone()));
            assert_eq!(t.advance_player(), Some(p2.clone()));
            assert_eq!(t.advance_player(), Some(p3.clone()));
        }
    }
    #[test]
    fn remove_last() {
        let p1 = make_player("p1");
        let p2 = make_player("p2");
        let mut t = TurnTracker::new(vec![p1.clone(), p2.clone()]);

        assert_eq!(t.advance_player(), Some(p1.clone()));
        t.remove_player("p1");
        for _ in 1..10 {
            assert_eq!(t.advance_player(), Some(p2.clone()));
        }
    }

    #[test]
    fn remove_next() {
        let p1 = make_player("p1");
        let p2 = make_player("p2");
        let mut t = TurnTracker::new(vec![p1.clone(), p2.clone()]);

        assert_eq!(t.advance_player(), Some(p1.clone()));
        t.remove_player("p2");
        for _ in 1..10 {
            assert_eq!(t.advance_player(), Some(p1.clone()));
        }
    }

    #[test]
    fn remove_all() {
        let p1 = make_player("p1");
        let p2 = make_player("p2");
        let mut t = TurnTracker::new(vec![p1.clone(), p2.clone()]);

        assert_eq!(t.advance_player(), Some(p1.clone()));
        t.remove_player("p2");
        t.remove_player("p1");
        for _ in 1..10 {
            assert_eq!(t.advance_player(), None);
        }
    }

    #[test]
    fn start_empty() {
        let p1 = make_player("p1");
        let p2 = make_player("p2");
        let mut t = TurnTracker::new(vec![]);

        assert_eq!(t.advance_player(), None);
        t.add_player(p1.clone());
        t.add_player(p2.clone());
        for _ in 1..10 {
            assert_eq!(t.advance_player(), Some(p1.clone()));
            assert_eq!(t.advance_player(), Some(p2.clone()));
        }
    }

    #[test]
    fn single_player() {
        let p1 = make_player("p1");
        let mut t = TurnTracker::new(vec![p1.clone()]);
        for _ in 1..10 {
            assert_eq!(t.advance_player(), Some(p1.clone()));
        }
    }
}
