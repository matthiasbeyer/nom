#[macro_use]
extern crate nom;

use nom::{IResult,Producer,FileProducer,Move,Consumer,ConsumerState,not_line_ending};

use std::str;
use std::fmt::Debug;
use nom::HexDisplay;

#[test]
#[allow(unused_must_use)]
fn tag() {
  FileProducer::new("links.txt", 20).map(|producer: FileProducer| {
    let mut p = producer;
    p.refill();

    consumer_from_parser!(PrintConsumer<()>, flat_map!(map_res!(tag!("https!"), str::from_utf8), print));
    let mut cs = PrintConsumer { state: ConsumerState::Continue(Move::Consume(0)) };
    for _ in 1..4 {
      p.apply(&mut cs);
    }
    //assert!(false);
  });
}

pub fn print<T: Debug>(input: T) -> IResult<T,()> {
  println!("{:?}", input);
  IResult::Done(input, ())
}


#[test]
fn is_not() {
  //is_not!(foo b"\r\n");
  named!(foo<&[u8],&[u8]>, is_not!(&b"\r\n"[..]));
  let a = &b"ab12cd\nefgh"[..];
  assert_eq!(foo(a), IResult::Done(&b"\nefgh"[..], &b"ab12cd"[..]));
}

#[test]
fn exported_public_method_defined_by_macro() {
  let a = &b"ab12cd\nefgh"[..];
  assert_eq!(not_line_ending(a), IResult::Done(&b"\nefgh"[..], &b"ab12cd"[..]));
}
