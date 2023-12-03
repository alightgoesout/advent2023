use std::fmt::Debug;
use std::io::{BufRead, BufReader, Read};
use std::iter::Filter;
use std::marker::PhantomData;
use std::str::FromStr;

pub trait FilterNotEmpty: Iterator + Sized {
    fn filter_not_empty(self) -> Filter<Self, fn(&String) -> bool>;
}

impl<I> FilterNotEmpty for I
where
    I: Iterator<Item = String>,
{
    fn filter_not_empty(self) -> Filter<Self, fn(&String) -> bool> {
        self.filter(|s| !s.is_empty())
    }
}

pub struct Parse<I, T>(I, PhantomData<T>);

impl<I, U, T> Iterator for Parse<I, T>
where
    I: Iterator<Item = U>,
    U: ToString,
    T: FromStr,
    T::Err: Debug,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|item| item.to_string().parse().unwrap())
    }
}

pub trait ParseExt<I> {
    fn parse<T>(self) -> Parse<I, T>;
}

impl<I: Iterator> ParseExt<I> for I {
    fn parse<T>(self) -> Parse<I, T> {
        Parse(self, PhantomData::default())
    }
}

pub fn read_lines<R: Read>(reader: R) -> impl Iterator<Item = String> {
    let buf_reader = BufReader::new(reader);
    buf_reader
        .lines()
        .filter(Result::is_ok)
        .map(|line| line.unwrap())
}
