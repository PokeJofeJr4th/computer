use std::{fmt::Debug, hash::{Hash, Hasher}};

pub fn print_and_ret<T: Debug>(t: T) -> T {
    println!("{t:?}");
    t
}

pub fn get_hash<T: Hash>(t: T) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    t.hash(&mut hasher);
    hasher.finish()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Either<A, B> {
    Left(A),
    Right(B),
}

impl<A, B> Either<A, B> {
    #[allow(clippy::missing_const_for_fn)]
    pub fn right(self) -> Option<B> {
        match self {
            Self::Left(_) => None,
            Self::Right(b) => Some(b),
        }
    }

    #[allow(clippy::missing_const_for_fn)]
    pub fn left(self) -> Option<A> {
        match self {
            Self::Left(a) => Some(a),
            Self::Right(_) => None,
        }
    }
}

impl<A, B> From<Either<A, B>> for Result<A, B> {
    fn from(value: Either<A, B>) -> Self {
        match value {
            Either::Left(a) => Self::Ok(a),
            Either::Right(b) => Self::Err(b),
        }
    }
}
