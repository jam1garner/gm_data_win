#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PosSlice<'a>(usize, pub(crate) &'a [u8]);

impl<'a> PosSlice<'a> {
    pub fn from_slice(slice: &'a [u8]) -> Self {
        PosSlice::<'a>(
            0usize,
            slice
        )
    }

    pub fn new(pos: usize, slice: &'a [u8]) -> Self {
        PosSlice::<'a>(
            pos,
            slice
        )
    }

    pub fn slice(&self, start: usize, end: usize) -> Self {
        PosSlice::<'a>(
            self.0 + start,
            &self.1[start..end]
        )
    }

    pub fn pos(&self) -> usize {
        self.0
    }

    pub fn offset(&self, off: usize) -> Self {
        self.slice(off, self.1.len())
    }

    pub fn len(&self) -> usize {
        self.1.len()
    }
}

use std::iter::{
    Enumerate,
    Map,
};

use nom::{
    InputTake,
    InputLength,
};

impl<'a> nom::InputIter for PosSlice<'a> {
    type Item = u8;
    type Iter = Enumerate<Self::IterElem>;
    type IterElem = Map<std::slice::Iter<'a, Self::Item>, fn(&u8) -> u8>;

    #[inline]
    fn iter_indices(&self) -> Self::Iter {
        self.iter_elements().enumerate()
    }

    #[inline]
    fn iter_elements(&self) -> Self::IterElem {
        self.1.iter().map(|a| *a)
    }

    #[inline]
    fn position<P>(&self, predicate: P) -> Option<usize>
        where
            P: Fn(Self::Item) -> bool,
    {
        self.1.iter().position(|b| predicate(*b))
    }

    #[inline]
    fn slice_index(&self, count: usize) -> Option<usize> {
        if self.1.len() >= count {
            Some(count)
        } else {
            None
        }
    }
}

impl<'a> nom::InputLength for PosSlice<'a> {
    fn input_len(&self) -> usize {
        self.1.len()
    }
}

impl<'a> nom::InputTake for PosSlice<'a> {
    fn take(&self, amount: usize) -> Self {
        self.slice(0, amount)
    }
    
    fn take_split(&self, amount: usize) -> (Self, Self) {
        let (prefix, suffix) = self.1.split_at(amount);
        (
            Self::new(self.0 + amount, suffix),
            Self::new(self.0, prefix)
        )
    }
}

use nom::{
    Err,
    Needed,
    IResult,
    error::{
        ParseError,
        ErrorKind
    }
};

impl<'a> nom::InputTakeAtPosition for PosSlice<'a> {
    type Item = u8;

    fn split_at_position<P, E: ParseError<Self>>(&self, predicate: P) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match (0..self.1.len()).find(|b| predicate(self.1[*b])) {
            Some(i) => Ok((
                      Self::new(self.0 + i, &self.1[i..]), 
                      Self::new(self.0, &self.1[..i])
                    )),
            None => Err(Err::Incomplete(Needed::Size(1))),
        }
    }

    fn split_at_position1<P, E: ParseError<Self>>(&self, predicate: P, e: ErrorKind) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match (0..self.1.len()).find(|b| predicate(self.1[*b])) {
            Some(0) => Err(Err::Error(E::from_error_kind(*self, e))),
            Some(i) => Ok((
                        Self::new(self.0 + i, &self.1[i..]), 
                        Self::new(self.0, &self.1[..i])
                    )),
            None => Err(Err::Incomplete(Needed::Size(1))),
        }
    }

    fn split_at_position_complete<P, E: ParseError<Self>>(&self, predicate: P) -> IResult<Self, Self, E>
        where P: Fn(Self::Item) -> bool
    {
        match (0..self.1.len()).find(|b| predicate(self.1[*b])) {
            Some(i) => Ok((
                        Self::new(self.0 + i, &self.1[i..]), 
                        Self::new(self.0, &self.1[..i])
                    )),
            None => Ok(self.take_split(self.input_len())),
        }
    }

    fn split_at_position1_complete<P, E: ParseError<Self>>(&self, predicate: P, e: ErrorKind) -> IResult<Self, Self, E>
        where P: Fn(Self::Item) -> bool
    {
        match (0..self.1.len()).find(|b| predicate(self.1[*b])) {
            Some(0) => Err(Err::Error(E::from_error_kind(*self, e))),
            Some(i) => Ok((
                        Self::new(self.0 + i, &self.1[i..]), 
                        Self::new(self.0, &self.1[..i])
                    )),
            None => {
                if self.1.len() == 0 {
                    Err(Err::Error(E::from_error_kind(*self, e)))
                } else {
                    Ok(self.take_split(self.input_len()))
                }
            },
        }
    }
}

impl<'a> nom::Compare<&[u8]> for PosSlice<'a> {
    fn compare(&self, to: &[u8]) -> nom::CompareResult {
        nom::Compare::compare(&self.1, to)
    }

    fn compare_no_case(&self, to: &[u8]) -> nom::CompareResult {
        nom::Compare::compare_no_case(&self.1, to)
    }
}

impl<'a> nom::Compare<&[u8; 4]> for PosSlice<'a> {
    fn compare(&self, to: &[u8; 4]) -> nom::CompareResult {
        nom::Compare::compare(&self.1, to)
    }

    fn compare_no_case(&self, to: &[u8; 4]) -> nom::CompareResult {
        nom::Compare::compare_no_case(&self.1, to)
    }
}
