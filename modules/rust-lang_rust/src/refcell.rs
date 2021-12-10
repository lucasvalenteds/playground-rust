use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Invitation {
    pub guest: &'static str,
    pub subscription: i32,
}

pub trait Event {
    fn new() -> Self;
    fn invite(&self, guest: &'static str) -> Result<Invitation, String>;
    fn invitations_list(&self) -> Vec<Invitation>;
}

struct GameNight {
    invitations: RefCell<HashMap<&'static str, i32>>,
}

impl Event for GameNight {
    fn new() -> Self {
        GameNight {
            invitations: RefCell::new(HashMap::new()),
        }
    }

    fn invite(&self, guest: &'static str) -> Result<Invitation, String> {
        let mut table = self.invitations.take();

        if table.contains_key(guest) {
            return Err("Guest already invited".to_string());
        }

        let subscription = table.len() as i32 + 1;
        table.insert(guest, subscription);
        self.invitations.replace(table);

        Ok(Invitation {
            guest,
            subscription,
        })
    }

    fn invitations_list(&self) -> Vec<Invitation> {
        self.invitations
            .take()
            .iter()
            .map(|(guest, subscription)| Invitation {
                guest,
                subscription: *subscription,
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {

    use super::{Event, GameNight};

    #[test]
    fn can_invitation_a_guest_to_event() {
        // Arrange
        let game_night = GameNight::new();
        assert_eq!(0, game_night.invitations_list().len());

        // Act
        let invitation_1 = game_night.invite("John Smith").unwrap();
        let invitation_2 = game_night.invite("Mary Jane").unwrap();

        // Assert
        assert_eq!(2, game_night.invitations_list().len());

        assert_eq!("John Smith", invitation_1.guest);
        assert_eq!(1, invitation_1.subscription);

        assert_eq!("Mary Jane", invitation_2.guest);
        assert_eq!(2, invitation_2.subscription);
    }

    #[test]
    fn cannot_invitation_the_same_guest_twice() {
        // Arrange
        let game_night = GameNight::new();
        assert_eq!(0, game_night.invitations_list().len());

        // Act
        let guest_name = "Edgar Brooks";
        let invitation_1 = game_night.invite(&guest_name);
        let invitation_2 = game_night.invite(&guest_name);

        // Assert
        assert_eq!(true, invitation_1.is_ok());
        assert_eq!("Guest already invited", invitation_2.unwrap_err());
    }
}
