use nom::{
    bytes::complete::{take_till, take_until, tag},
    character::{is_newline, complete::{newline, not_line_ending}},
    error::*,
    sequence::preceded,
    combinator::opt,
    multi::many0,
    *,
};

pub fn todo() -> impl Fn(&[u8]) -> IResult<&[u8], &[u8], Error<&[u8]>> {
    move |i: &[u8]| {
        let header = take_until("TODO:");
        let content = take_till(is_newline);
        preceded(header, content)(i)
    }
}

#[derive(Debug, PartialEq)]
pub struct Todo<'a> {
    tag: &'a str,
    text: &'a str,
}

pub fn parse_todo<'a>(todo_tag: &'a str, input: &'a str) -> IResult<&'a str, Todo<'a>> {
    // remove everything prior to the tag
    let (input, _) = take_until(todo_tag)(input)?;

    let (input, _) = tag(todo_tag)(input)?;
    let (input, text) = not_line_ending(input)?;

    // discard optional line ending
    let (input, _) = opt(newline)(input)?;

    Ok((input, Todo {
        tag: todo_tag,
        text
    }))
}

pub fn parse_todos<'a>(file_content: &'a str) -> IResult<&'a str, Vec<Todo<'a>>> {
    let line_parser = | file_content | parse_todo("TODO:", file_content);
    let (input, todos) = many0(line_parser)(file_content)?;
    Ok((input, todos))
}


#[test]
fn todos() {
    let input =
        "line 1\n\
         TODO: todo1\n\
         line 2\n\
         line 3 -- TODO: todo2\n";

    assert_eq!(parse_todos(input),
               Ok(("",
                   vec![
                       Todo{ tag: "TODO:",
                             text: " todo1"},
                       Todo{ tag: "TODO:",
                             text: " todo2"},
                   ])
               )
    );

    let input = "line 1\n";
    assert_eq!(parse_todos(input),
               Ok(("line 1\n",
                   vec![])
               )
    );

    let input = "line1 TODO: todo1\n";
    assert_eq!(parse_todos(input),
               Ok(("",
                   vec![
                       Todo { tag: "TODO:",
                              text: " todo1"}])
               )
    );

    let input =
        "line1\n\
         line2\n\
         line3\n\
         TODO: todo1\n\
         line4\n";
    assert_eq!(parse_todos(input),
               Ok(("line4\n",
                   vec![
                       Todo { tag: "TODO:",
                              text: " todo1"}])
               )
    );

    let input =
        "line1\n\
         line2\n\
         line3\n\
         FIXME: todo1\n\
         line4\n";
    assert_eq!(parse_todos(input),
               Ok((input,
                   vec![])
               )
    );

}


#[test]
fn todo_clean() {
    let input = "TODO: test todo";
    assert_eq!(
        parse_todo("TODO:", input),
        Ok(("",
            Todo {
                tag: "TODO:",
                text: " test todo"
            })
        )
    );
}

#[test]
fn todo_tag_only() {
    let input = "TODO:\nline1";
    assert_eq!(
        parse_todo("TODO:", input),
        Ok(("line1",
            Todo {
                tag: "TODO:",
                text: ""
            })
        )
    );

    let input = "TODO:";
    assert_eq!(
        parse_todo("TODO:", input),
        Ok(("",
            Todo {
                tag: "TODO:",
                text: ""
            })
        )
    );

    let input = "TODO:\n";
    assert_eq!(
        parse_todo("TODO:", input),
        Ok(("",
            Todo {
                tag: "TODO:",
                text: ""
            })
        )
    );
}

#[test]
fn todo_clean_newline() {
    let input = "TODO: test todo\nline1";
    assert_eq!(
        parse_todo("TODO:", input),
        Ok(("line1",
            Todo {
                tag: "TODO:",
                text: " test todo"
            })
        )
    );
}

#[test]
fn todo_in_comment() {
    let input = "// TODO: test todo";
    assert_eq!(
        parse_todo("TODO:", input),
        Ok(("",
            Todo {
                tag: "TODO:",
                text: " test todo"
            })
        )
    );
}

#[test]
fn todo_different_tag() {
    let input = "FIXME: different tag";
    assert_eq!(parse_todo("TODO:", input).ok(), None);
}
