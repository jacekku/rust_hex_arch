use crate::domain::create_pokemon::*;
use crate::domain::entities::*;

#[cfg(test)]
mod tests {
    use std::{sync::Arc, vec};

    use crate::repositories::pokemon::{InMemoryRepository, Repository};

    use super::*;

    #[test]
    fn it_should_return_the_pokemon_number_otherwise() {
        let repo = Arc::new(InMemoryRepository::new());

        let req = Request::new(
            PokemonNumber::pikachu(),
            PokemonName::pikachu(),
            PokemonTypes::pikachu(),
        );

        let res = execute(repo, req);

        match res {
            Ok(res) => {
                assert_eq!(res.number, u16::from(PokemonNumber::pikachu()));
                assert_eq!(res.name, String::from(PokemonName::pikachu()));
                assert_eq!(res.types, Vec::<String>::from(PokemonTypes::pikachu()));
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_should_return_a_bad_request_error_when_request_is_invalid() {
        let repo = Arc::new(InMemoryRepository::new());
        let req = Request::new(
            PokemonNumber::pikachu(),
            PokemonName::bad(),
            PokemonTypes::pikachu(),
        );

        let res = execute(repo, req);

        match res {
            Err(Error::BadRequest) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_should_return_a_conflict_error_when_pokemon_number_already_exists() {
        let repo = Arc::new(InMemoryRepository::new());

        repo.insert(
            PokemonNumber::pikachu(),
            PokemonName::pikachu(),
            PokemonTypes::pikachu(),
        )
        .ok();

        let req = Request::new(
            PokemonNumber::pikachu(),
            PokemonName::charmander(),
            PokemonTypes::charmander(),
        );

        let res = execute(repo, req);

        match res {
            Err(Error::Conflict) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_should_return_an_error_when_an_unexpected_error_happens() {
        let repo = Arc::new(InMemoryRepository::new().with_error());

        let req = Request::new(
            PokemonNumber::pikachu(),
            PokemonName::pikachu(),
            PokemonTypes::pikachu(),
        );

        let res = execute(repo, req);

        match res {
            Err(Error::Unknown) => {}
            _ => unreachable!(),
        }
    }

    impl Request {
        fn new(number: PokemonNumber, name: PokemonName, types: PokemonTypes) -> Self {
            Self {
                number: u16::from(number),
                name: String::from(name),
                types: Vec::<String>::from(types),
            }
        }
    }
}
