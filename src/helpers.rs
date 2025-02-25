use std::fmt::{Display, Formatter};

use crate::report::{tips, warn, ErrorKey};
use crate::token::Token;

/// Warns about a redefinition of a database item
pub fn dup_error(key: &Token, other: &Token, id: &str) {
    warn(ErrorKey::DuplicateItem)
        .msg(format!("{id} is redefined by another {id}"))
        .loc(other)
        .loc(key, format!("the other {id} is here"))
        .push();
}

/// Warns about an exact redefinition of a database item
pub fn exact_dup_error(key: &Token, other: &Token, id: &str) {
    warn(ErrorKey::ExactDuplicateItem)
        .msg(format!("{id} is redefined by an identical {id}"))
        .loc(other)
        .loc(key, format!("the other {id} is here"))
        .push();
}

/// Warns about a redefinition of a database item, but only at "advice" level
pub fn exact_dup_advice(key: &Token, other: &Token, id: &str) {
    tips(ErrorKey::DuplicateItem)
        .msg(format!("{id} is redefined by an identical {id}, which may cause problems if one of them is later changed"))
        .loc(other)
        .loc(key, format!("the other {id} is here"))
        .push();
}

/// Warns about a duplicate `key = value` in a database item
pub fn dup_assign_error(key: &Token, other: &Token) {
    // Don't trace back macro invocations for duplicate field errors,
    // because they're just confusing.
    let mut key = key.clone();
    key.loc.link = None;
    let mut other = other.clone();
    other.loc.link = None;

    warn(ErrorKey::DuplicateField)
        .msg(format!("`{other}` is redefined in a following line").as_str())
        .loc(other.loc)
        .loc(key.loc, "the other one is here")
        .push();
}

pub fn display_choices(f: &mut Formatter, v: &[&str], joiner: &str) -> Result<(), std::fmt::Error> {
    for i in 0..v.len() {
        write!(f, "{}", v[i])?;
        if i + 1 == v.len() {
        } else if i + 2 == v.len() {
            write!(f, " {joiner} ")?;
        } else {
            write!(f, ", ")?;
        }
    }
    Ok(())
}

/// The Choices enum exists to hook into the Display logic of printing to a string
enum Choices<'a> {
    OrChoices(&'a [&'a str]),
    AndChoices(&'a [&'a str]),
}

impl<'a> Display for Choices<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Choices::OrChoices(cs) => display_choices(f, cs, "or"),
            Choices::AndChoices(cs) => display_choices(f, cs, "and"),
        }
    }
}

pub fn stringify_choices(v: &[&str]) -> String {
    format!("{}", Choices::OrChoices(v))
}

pub fn stringify_list(v: &[&str]) -> String {
    format!("{}", Choices::AndChoices(v))
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TriBool {
    True,
    False,
    Maybe,
}

/// A trait for taking a `String` or `&str` and then either re-using the `String` or cloning the `&str`.
/// It seems like this should already exist in the standard library but I can't find it.
/// (As far as I can tell, `ToOwned` isn't it -- using that will clone the `String`.)
pub trait Own<T> {
    fn own(self) -> T;
}

impl<T> Own<T> for T {
    fn own(self) -> T {
        self
    }
}

impl<T: ToOwned<Owned = T>> Own<T> for &T {
    fn own(self) -> T {
        self.to_owned()
    }
}

impl Own<String> for &str {
    fn own(self) -> String {
        self.to_owned()
    }
}
