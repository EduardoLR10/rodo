use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{anychar, char, newline, not_line_ending, none_of},
    combinator::{fail, opt},
    multi::{many0, many_till},
    sequence::tuple,
    *,
};

// TODO: change String to &str in the Todo type \
// Making this will break the list_todos_path() function because of ownership of
// the file contents.
#[derive(Debug, PartialEq, Clone)]
pub struct Todo {
    pub tag: String,
    pub text: String,
}

fn parse_header<'a>(todo_tag: &'a str, input: &'a str) -> IResult<&'a str, ()> {
    // remove everything prior to the tag
    let (input, _) = take_until(todo_tag)(input)?;
    // detect todo_tag
    let (input, _) = tag(todo_tag)(input)?;
    Ok((input, ()))
}

fn parse_todo<'a>(todo_tag: &'a str, input: &'a str) -> IResult<&'a str, Todo> {
    let (input, _) = parse_header(todo_tag, input)?;

    let (input, text) = not_line_ending(input)?;
    // discard optional line ending
    let (input, _) = opt(newline)(input)?;

    Ok((
        input,
        Todo {
            tag: todo_tag.to_owned(),
            text: text.to_owned(),
        },
    ))
}

fn parse_todo_multiline<'a>(todo_tag: &'a str, input: &'a str) -> IResult<&'a str, Todo> {
    let (input, _) = parse_header(todo_tag, input)?;

    println!("{:?}", input);

    //let parse_line = many_till(anychar::<T, E>, tuple((char('\\'), newline)));

    let (input, (vector, _)) = many_till(
        many_till(anychar, tuple((char('\\'), newline))),
        many_till(anychar, tuple((none_of("\\"), newline))),
    )(input)?;

    println!("{:?}", vector);

    let result: String = vector.into_iter().map(| (line, _) | line.iter().collect::<String>()).collect();

    // for (v, _) in vector {
    //     let line:String = v.iter().collect();

    // }

    // let (input, text) = not_line_ending(input)?;

    // match text.chars().last() {
    //     Some('\\') => todo!(),
    //     _ => fail::<&str, Todo, nom::error::Error<&str>>("")?,
    // };

    // discard optional line ending
    // let (input, _) = opt(newline)(input)?;

    Ok((
        input,
        Todo {
            tag: todo_tag.to_owned(),
            text: result,
        },
    ))
}


fn parse_todos(file_content: &str) -> IResult<&str, Vec<Todo>> {
    let line_parser = |file_content| parse_todo("TODO:", file_content);
    let (input, todos) = many0(line_parser)(file_content)?;
    Ok((input, todos))
}

pub fn parse_file(file_content: &str) -> Vec<Todo> {
    match parse_todos(file_content) {
        Ok((_, todos)) => todos,
        Err(_) => vec![],
    }
}

#[cfg(test)]
mod test {
    use crate::commands::parser::{parse_todo, parse_todos, parse_todo_multiline, Todo};

    #[test]
    fn multiline_todos() {
        let input =
            "TODO: todo1 \\n\
             todo2\n\
             line1";

        assert_eq!(
            parse_todo_multiline("TODO:", input),
            Ok((
                "line1",
                Todo {
                    tag: "TODO:".to_owned(),
                    text: " todo1 todo".to_owned()
                },
            ))
        );
    }

    fn todos() {
        let input = "line 1\n\
             TODO: todo1\n\
             line 2\n\
             line 3 -- TODO: todo2\n";

        assert_eq!(
            parse_todos(input),
            Ok((
                "",
                vec![
                    Todo {
                        tag: "TODO:".to_owned(),
                        text: " todo1".to_owned()
                    },
                    Todo {
                        tag: "TODO:".to_owned(),
                        text: " todo2".to_owned()
                    },
                ]
            ))
        );

        let input = "line 1\n";
        assert_eq!(parse_todos(input), Ok(("line 1\n", vec![])));

        let input = "line1 TODO: todo1\n";
        assert_eq!(
            parse_todos(input),
            Ok((
                "",
                vec![Todo {
                    tag: "TODO:".to_owned(),
                    text: " todo1".to_owned()
                }]
            ))
        );

        let input = "line1\n\
             line2\n\
             line3\n\
             TODO: todo1\n\
             line4\n";
        assert_eq!(
            parse_todos(input),
            Ok((
                "line4\n",
                vec![Todo {
                    tag: "TODO:".to_owned(),
                    text: " todo1".to_owned()
                }]
            ))
        );

        let input = "line1\n\
             line2\n\
             line3\n\
             FIXME: todo1\n\
             line4\n";
        assert_eq!(parse_todos(input), Ok((input, vec![])));
    }

    #[test]
    fn todo_clean() {
        let input = "TODO: test todo";
        assert_eq!(
            parse_todo("TODO:", input),
            Ok((
                "",
                Todo {
                    tag: "TODO:".to_owned(),
                    text: " test todo".to_owned()
                }
            ))
        );
    }

    #[test]
    fn todo_tag_only() {
        let input = "TODO:\nline1";
        assert_eq!(
            parse_todo("TODO:", input),
            Ok((
                "line1",
                Todo {
                    tag: "TODO:".to_owned(),
                    text: "".to_owned()
                }
            ))
        );

        let input = "TODO:";
        assert_eq!(
            parse_todo("TODO:", input),
            Ok((
                "",
                Todo {
                    tag: "TODO:".to_owned(),
                    text: "".to_owned()
                }
            ))
        );

        let input = "TODO:\n";
        assert_eq!(
            parse_todo("TODO:", input),
            Ok((
                "",
                Todo {
                    tag: "TODO:".to_owned(),
                    text: "".to_owned()
                }
            ))
        );
    }

    #[test]
    fn todo_clean_newline() {
        let input = "TODO: test todo\nline1";
        assert_eq!(
            parse_todo("TODO:", input),
            Ok((
                "line1",
                Todo {
                    tag: "TODO:".to_owned(),
                    text: " test todo".to_owned()
                }
            ))
        );
    }

    #[test]
    fn todo_in_comment() {
        let input = "// TODO: test todo";
        assert_eq!(
            parse_todo("TODO:", input),
            Ok((
                "",
                Todo {
                    tag: "TODO:".to_owned(),
                    text: " test todo".to_owned()
                }
            ))
        );
    }

    #[test]
    fn todo_different_tag() {
        let input = "FIXME: different tag";
        assert_eq!(parse_todo("TODO:", input).ok(), None);
    }
}
