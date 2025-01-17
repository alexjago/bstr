// DO NOT EDIT THIS FILE. IT WAS AUTOMATICALLY GENERATED BY:
//
//   ucd-generate dfa --name GRAPHEME_BREAK_REV --reverse --longest --sparse --minimize --anchored --state-size 2 src/unicode/fsm/ [snip (arg too long)]
//
// ucd-generate 0.2.12 is available on crates.io.

#[cfg(target_endian = "big")]
lazy_static::lazy_static! {
  pub static ref GRAPHEME_BREAK_REV: ::regex_automata::SparseDFA<&'static [u8], u16> = {
    #[repr(C)]
    struct Aligned<B: ?Sized> {
        _align: [u8; 0],
        bytes: B,
    }

    static ALIGNED: &'static Aligned<[u8]> = &Aligned {
        _align: [],
        bytes: *include_bytes!("grapheme_break_rev.bigendian.dfa"),
    };

    unsafe {
      ::regex_automata::SparseDFA::from_bytes(&ALIGNED.bytes)
    }
  };
}

#[cfg(target_endian = "little")]
lazy_static::lazy_static! {
  pub static ref GRAPHEME_BREAK_REV: ::regex_automata::SparseDFA<&'static [u8], u16> = {
    #[repr(C)]
    struct Aligned<B: ?Sized> {
        _align: [u8; 0],
        bytes: B,
    }

    static ALIGNED: &'static Aligned<[u8]> = &Aligned {
        _align: [],
        bytes: *include_bytes!("grapheme_break_rev.littleendian.dfa"),
    };

    unsafe {
      ::regex_automata::SparseDFA::from_bytes(&ALIGNED.bytes)
    }
  };
}
