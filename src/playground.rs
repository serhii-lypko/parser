fn main() -> Result<(), Box<dyn Error>> {
  // let (reminder1, char) = take_first_char("abc")?;
  // println!("reminder is: {:?}, chars is: {}", reminder1, char);

  // let parse_semicolon = match_literal(";");
  // let semicolon_res = parse_semicolon("abc");
  // println!("{:?}", semicolon_res);

  // let parse = identifier("name∆");

  let tag_opener = match_literal("<");
  let combinator = pair(tag_opener, identifier);
  // let combinator_res = combinator("<!hello/>");
  // println!("{:?}", combinator_res);

  fn mapper<A>(arg: A)
  where
      A: Debug,
  {
      println!("{:?}", arg);
  }

  let functor = map(combinator, mapper);

  let functor_res = functor("name");

  // println!("{:?}", functor_res);

  let foo_identifier = identifier("name");
  let foo_mapped = foo_identifier.map(|(reminder, result)| {
      println!("{:?}", result);
      //
      (reminder, result)
  });

  // println!("{:?}", foo_mapped);

  Ok(())
}