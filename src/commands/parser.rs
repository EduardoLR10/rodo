use nom::{*, bytes::complete::{take_until, take_till}, character::{is_newline}, error::*, sequence::*};

pub fn todo() -> impl Fn(&[u8]) -> IResult<&[u8], &[u8], Error<&[u8]>> {
    move |i: &[u8]| {
	let header = take_until("TODO:");
	let content = take_till(is_newline);
	preceded(header, content)(i)
    }
}
