use crate::gametraits::User;
use itertools::enumerate;
use itertools::Itertools;
use log::debug;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TurnTracker {
    players: Vec<User>,
    next_player_index: usize,
    single_player_mode_started: bool,
}

impl TurnTracker {
    fn player_string(&self) -> String {
        let mut players: String = String::new();
        for (i, User { name, .. }) in enumerate(&self.players) {
            if i == self.next_player_index {
                players += format!(", *{name}").as_str();
            } else {
                players += format!(", {name}").as_str();
            }
        }
        players
    }

    pub fn new(players: Vec<User>) -> Self {
        debug!("Creating turn tracker, with users {players:?}");
        Self {
            players,
            next_player_index: 0,
            single_player_mode_started: false,
        }
    }

    pub fn is_playing(&self, username: &str) -> bool {
        self.players.iter().any(|p| p.name == username)
    }

    pub fn remove_player(&mut self, username: &str) {
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
        let p_str = self.player_string();
        debug!("Removing player {username}, left: {p_str}");
    }

    pub fn add_player(&mut self, user: User) {
        if self.players.iter().any(|p| p.name == user.name) {
            panic!("Player with identical name added twice");
        }
        let p_name = user.name.clone();
        self.players.push(user);
        if self.players.len() == 2 && self.single_player_mode_started {
            self.next_player_index = 1;
        }
        let p_str = self.player_string();
        debug!("Adding player {p_name}, new: {p_str}");
    }

    pub fn advance_player(&mut self) -> Option<User> {
        if self.players.is_empty() {
            return None;
        }
        self.single_player_mode_started = self.players.len() == 1;

        let current_index = self.next_player_index;
        self.next_player_index = (self.next_player_index + 1) % self.players.len();
        let p_str = self.player_string();
        debug!("Advancing player, new: {p_str}");
        self.players.get(current_index).map(Clone::clone)
    }

    pub fn num_players(&self) -> usize {
        self.players.len()
    }

    pub fn is_first_player(&self, name: &str) -> bool {
        if let Some(first_player) = self.players.first() {
            name == first_player.name
        } else {
            false
        }
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
            assert_eq!(t.advance_player(), Some(p2.clone()));
            assert_eq!(t.advance_player(), Some(p3.clone()));
            assert_eq!(t.advance_player(), Some(p1.clone()));
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
    fn add_one_advance_add_one_then_rinse_repeat() {
        let p1 = make_player("p1");
        let p2 = make_player("p2");
        let mut t = TurnTracker::new(vec![]);

        assert_eq!(t.advance_player(), None);
        t.add_player(p1.clone());
        assert_eq!(t.advance_player(), Some(p1.clone()));
        t.add_player(p2.clone());
        assert_eq!(t.advance_player(), Some(p2.clone()));

        t.remove_player("p1");
        t.remove_player("p2");

        assert_eq!(t.advance_player(), None);
        t.add_player(p1.clone());
        assert_eq!(t.advance_player(), Some(p1.clone()));
        t.add_player(p2.clone());
        assert_eq!(t.advance_player(), Some(p2.clone()));
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
