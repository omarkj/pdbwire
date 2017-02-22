#[macro_use]
extern crate nom;

use nom::{be_u8, be_u16, be_u32, IResult};

named!(mint,
    do_parse!(
        peek!(one_of!(&[0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8])) >>
        byte: take!(1) >>
        (byte)
    ));

named!(word,
    do_parse!(
        length: do_parse!(
                tag: take!(1) >>
                l: cond_reduce!(tag[0] & 128 == 128, be_u16) >>
                (l - 128)
            ) >>
        bytes: take!(length) >>
        (bytes)
    ));

named!(internal_word,
    do_parse!(
        length: do_parse!(
                tag!(&[128u8][..]) >>
                length: be_u8 >>
                (length)
            ) >>
        bytes: take!(length - 128) >>
        (bytes)
    ));

named!(small,
    do_parse!(
        tag!(&[111u8][..]) >>
        length: be_u8 >>
        bytes: take!(length) >>
        (bytes)
    ));

named!(medium,
    do_parse!(
        tag!(&[112u8][..]) >>
        length: be_u16 >>
        bytes: take!(length) >>
        (bytes)
    ));

named!(large,
    do_parse!(
        tag!(&[113u8][..]) >>
        length: be_u32 >>
        bytes: take!(length) >>
        (bytes)
    ));

named!(program<Vec<u8>>,
    do_parse!(
        program: many0!(alt!(mint | small | medium | large | internal_word | word)) >>
        ({
            let mut vec = Vec::new();
            for item in program {
                vec.extend_from_slice(item);
            }
            vec
        })
    ));

#[cfg(test)]
mod tests {
    use super::{mint, small, medium, large};

    use nom::IResult;

    #[test]
    fn test_mints() {
        let d = vec![0u8];
        let rest: &[u8] = &[];
        let success: &[u8] = &[0u8];
        assert_eq!(mint(&d), IResult::Done(rest, success));
    }

    #[test]
    fn test_small() {
        let d = vec![111u8, 1u8, 101u8];
        let rest: &[u8] = &[];
        let success: &[u8] = &[101u8];
        assert_eq!(small(&d), IResult::Done(rest, success));
    }

    #[test]
    fn test_medium() {
        let d = vec![112u8, 0u8, 1u8, 101u8];
        let rest: &[u8] = &[];
        let success: &[u8] = &[101u8];
        assert_eq!(medium(&d), IResult::Done(rest, success));
    }

    #[test]
    fn test_large() {
        let d = vec![113u8, 0u8, 0u8, 0u8, 1u8, 101u8];
        let rest: &[u8] = &[];
        let success: &[u8] = &[101u8];
        assert_eq!(large(&d), IResult::Done(rest, success));
    }
}
