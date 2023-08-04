use std::{error::Error, fmt::Debug, fmt::Display};

/*
    Parsing is a process of deriving structure from a stream of data.
    A parser is something which teases out that structure.

    A parser, in its simplest form, is a function which takes some input
    and returns either the parsed output along with the remainder of the input,
    or an error saying "I couldn't parse this."

    Identifier

    one alphabetical character, followed by zero or more of either an alphabetical
    character, a number, or a dash -

    examples:

    <name></name>
    <name-></name->
    <n1ame-></n1ame->
    <na-2me-></na-2me->
*/

struct Element {
    name: String,
    attributes: Vec<(String, String)>,
    children: Vec<Element>,
}


// lifetime 'a refers specifically to the lifetime of the input
type ParserOutput<'a, Output> = Result<(&'a str, Output), &'a str>;

trait Parser<'a, Output> {
    fn parse(&self, input: &'a str) -> ParserOutput<'a, Output>;
}

/*
    implementing Parser trait for any function that matches it's signature
    (any function that takes a string slice and returns a ParseResult is considered a Parser), 
    but eventually it doesn't mean there won't be implementations for another structs or types

    This way we also open up the possibility to use other kinds of types as parsers.
    But, more importantly, it saves us from having to type out function signatures all the time.
*/
impl <'a, F, Output> Parser<'a, Output> for F 
where
    F: Fn(&'a str) -> ParserOutput<Output>,
{
    fn parse(&self, input: &'a str) -> ParserOutput<'a, Output> {
        self(input)
    }
}

// fn identity_combinator<'a, P, I, O>(parser: P, identity: I) -> impl Fn(&'a str) -> ParserOutput<O>
// where
//     P: Parser<'a, O>,
//     I: Fn(ParserOutput<O>) -> ParserOutput<O>
// {
//     move |input| {
//         identity(parser.parse(input))
//     }
// }

// impl Fn(&'a str) -> ParserOutput<O>
// impl Parser<'a, O>


// короче идея impl Parser<'a, O>, чтобы одни комбинаторы могли принимать результаты
// других, создавая композицию
fn identity_combinator<'a, P, O>(parser: P) -> impl Parser<'a, O> 
where
    P: Parser<'a, O>,
{
    move |input| {
        parser.parse(input)
    }
}

fn identity_combinator_2<'a, P, O>(parser: P) -> impl Fn(&'a str) -> ParserOutput<O> 
where
    P: Parser<'a, O>,
{
    move |input| {
        parser.parse(input)
    }
}

fn left<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, R1>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    map(pair(parser1, parser2), |(left, _right)| left)
}


fn main() -> Result<(), Box<dyn Error>> {
    // let mapper = map(take_first_char, |input| input);
    // println!("{:?}", mapper.parse("hello"));

    // let id_cb = identity_combinator(take_first_char);

    let left_c = left(take_first_char, take_first_char);

    println!("{:?}", left_c.parse("hello"));

    // println!("{:?}", id_cb.parse("hello"));

    Ok(())
}


/* -- -- -- -- -- -- -- -- Parsers -- -- -- -- -- -- -- -- -- */

/*
    parser by it's own
*/
fn take_first_char(input: &str) -> ParserOutput<char> {
    let first_char = input.chars().next();

    match first_char {
        // utf8 char could take from 1 to 4 bytes 
        // and string slice operates bytes at [..] operation
        Some(c) => Ok((&input[c.len_utf8()..], c)),
        _ => Err(input),
    }
}

/*
    check whether a given input string begins with a specific match_literal string,
    but no need to return this literal as a part of a result. will be used for example
    to check tags opening/closing

    convinient with partial application -> returns closure

    returns parser
*/
fn match_literal(expected: &'static str) -> impl Fn(&str) -> ParserOutput<()> {
    move |input| {
        let expected_input_slice = input.get(0..expected.len());

        match expected_input_slice {
            Some(next) if next == expected => Ok((&input[expected.len()..], ())),
            _ => Err(input),
        }
    }
}

/*
    will be used for example to check tags names (identifier),
    since xml-identifier string is alphanumeric + '-'

    parser by it's own
*/
fn identifier(input: &str) -> ParserOutput<String> {
    let mut matched = String::new();
    let mut chars = input.chars();

    // identifier has to start from alphabetic char only
    match chars.next() {
        Some(next) if next.is_alphabetic() => matched.push(next),
        _ => return Err(input),
    }

    while let Some(next) = chars.next() {
        if next.is_alphanumeric() || next == '-' {
            matched.push(next);
        } else {
            break;
        }
    }

    let next_index = matched.len();

    Ok((&input[next_index..], matched))
}

/* -- -- -- -- -- -- -- -- Combinators -- -- -- -- -- -- -- -- -- */

fn pair<'a, P1, P2, R1, R2>(
    parser1: P1,
    parser2: P2,
) -> impl Parser<'a, (R1, R2)>
where
    P1: Parser<'a, R1>, // could be shortened with trait
    P2: Parser<'a, R2>, // could be shortened with trait
{
    move |input| match parser1.parse(input) {
        Ok((next_input, result1)) => match parser2.parse(next_input) {
            Ok((final_input, result2)) => Ok((final_input, (result1, result2))),
            Err(err) => Err(err),
        },
        Err(err) => Err(err),
    }
}



// Result<(&str, A), &str> -> Result<(&str, B), &str>
// fn map<P, F, A, B>(parser: P, map_fn: F) -> impl Fn(&str) -> Result<(&str, B), &str>
// where
//     P: Fn(&str) -> Result<(&str, A), &str>,
//     F: Fn(A) -> B,
// {
//     move |input| match parser(input) {
//         Ok((next_input, result)) => Ok((next_input, map_fn(result))),
//         Err(err) => Err(err),
//     }
// }

// fn map<'a, P, F, A, B>(parser: P, map_fn: F) -> impl Fn(&str) -> ParserOutput<B>
// where
//     P: Fn(&str) -> ParserOutput<A>, // could be shortened with trait
//     F: Fn(A) -> B,
// {
//     move |input| {
//         parser(input)
//             // Result - is a functor, so we can map over it
//             .map(|(next_input, result)| (next_input, map_fn(result)))
//     }
// }

fn map<'a, P, F, A, B>(parser: P, map_fn: F) -> impl Parser<'a, B>
where
    P: Parser<'a, A>,
    F: Fn(A) -> B,
{
    move |input| {
        parser.parse(input)
            // Result - is a functor, so we can map over it
            .map(|(next_input, result)| (next_input, map_fn(result)))
    }
}

// fn identifier_with_rule<R>(input: &str, verifier: R) -> ParserOutput<String> 
// where 
//     R: Fn(char) -> bool    
// {
//     let mut matched = String::new();
//     let mut chars = input.chars();

//     while let Some(next) = chars.next() {
//         if verifier(next) {
//             matched.push(next)
//         } else {
//             break;
//         }
//     }

//     let next_index = matched.len();

//     Ok((&input[next_index..], matched))
// }

/* -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- */

// fn main() -> Result<(), Box<dyn Error>> {
//     Ok(())
// }

/* -- -- -- -- -- -- -- -- Tests -- -- -- -- -- -- -- -- -- */

#[test]
fn test_match_literal() {
    let parse_opening_angle_bracket = match_literal("<");

    assert_eq!(Ok(("", ())), parse_opening_angle_bracket("<"));
    assert_eq!(Ok(("name>", ())), parse_opening_angle_bracket("<name>"));
    assert_eq!(Err("foo"), parse_opening_angle_bracket("foo"));
}

#[test]
fn test_identifier() {
    assert_eq!(Err(""), identifier(""));
    assert_eq!(Err("!not-identifier"), identifier("!not-identifier"));
    assert_eq!(
        Ok(("", String::from("is-identifier"))),
        identifier("is-identifier")
    );
    assert_eq!(Ok(("😎", String::from("name-"))), identifier("name-😎"));
}

// #[test]
// fn test_pair() {
//     let tag_opener = pair(match_literal("<"), identifier);

//     assert_eq!(
//         Ok(("/>", ((), "hello".to_string()))),
//         tag_opener("<hello/>")
//     );

//     assert_eq!(Ok(("", ((), "id-1".to_string()))), tag_opener("<id-1"));

//     assert_eq!(Err("+hello/>"), tag_opener("+hello/>"));

//     assert_eq!(Err("!hello/>"), tag_opener("<!hello/>"));
// }
