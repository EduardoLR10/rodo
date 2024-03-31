use nom::{*, bytes::complete::{take_until, take_till}, character::{is_newline}, error::*, sequence::*};

pub fn todo() -> impl Fn(&[u8]) -> IResult<&[u8], &[u8], Error<&[u8]>> {
    move |i: &[u8]| {
	let header = take_until("TODO:");
	let content = take_till(is_newline);
	preceded(header, content)(i)
    }
}

#[derive(Debug, PartialEq)]
pub enum PResult<'a> {
    Todo {
        text: &'a str
    },
    NoTodo
}

pub fn parse_todo<'a>(todo_tag: &'a str, input: &'a str) -> PResult<'a> {
    preceded(
        take_until(todo_tag),
        rest::<&str, ()>
    )(input).map_or(PResult::NoTodo,
                    |(_, result)| PResult::Todo {text: result} )
}

#[test]
fn todo_clean() {
    let input = "TODO: test todo";
    assert_eq!(parse_todo("TODO:", input), PResult::Todo{text: "TODO: test todo"});
}

#[test]
fn todo_in_comment() {
    let input = "// TODO: test todo";
    assert_eq!(parse_todo("TODO:", input), PResult::Todo{text: "TODO: test todo"});
}

#[test]
fn todo_different_tag() {
    let input = "FIXME: different tag";
    assert_eq!(parse_todo("TODO:", input), PResult::NoTodo);
}



pub fn parse_todo1<'a>(todo_tag: &'a str, input: &'a str) -> IResult<&'a str, &'a str> {
    preceded(
        take_until(todo_tag),
        rest
    )(input)
}

#[test]
fn todo_clean1() {
    let input = "TODO: test todo";
    assert_eq!(parse_todo1("TODO:", input), Ok(("", "TODO: test todo")));
}

#[test]
fn todo_in_comment1() {
    let input = "// TODO: test todo";
    assert_eq!(parse_todo1("TODO:", input), Ok(("", "TODO: test todo")));
}

#[test]
fn todo_different_tag1() {
    let input = "FIXME: different tag";
    match parse_todo1("TODO:", input) {
        Ok(_) => assert!(false),
        Err(_) => assert!(true)
    }
}
