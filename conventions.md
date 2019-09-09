This document descripes some conventions,
  in particular those about the programming style,
to have some uniformity for this repository.
Meaningful alignment, order, consistency, a good appearance and a cup of tea shall always be considered.

Naming Things
=============

Identifiers should be in British English without contractions.
Variables, constants, functions and other actual objects shall be in lower camel case,
  but structures, enumeration types, traits, and other abstract things as well as members of enumerated types should usually be in upper camel case.
If due to reasonable reasons abstract things are to be in lower camel case,
  it should be preceded with:

```{.rust .numberLines}
  #[allow(non_camel_case_types)]
```  

Write the following lines in the top the central file to prevent warnings:

```{.rust .numberLines}
  #![allow(non_snake_case)]
  #![allow(non_upper_case_globals)]
```

Indentation, Alignment and Spacing
==================================

Two spaces per indentation shall be used.
Never use tabs (ASCII-Character 0x09/›\\t‹)!
The only two whitespace-characters,
  that could be used,
are spaces (ASCII 0x20/› ‹) and line feeds (ASCII 0x0a/›\\n‹).

Brackets of block structures shall be both on
  their own lines and,
    if present,
  the same indentation-level with the control statements:

```{.rust .numberLines}
  fn foobar
  (
    x:                                  usize,
    y:                                  usize,
  )
  ->  Result
      <
        usize,
        String,
      >
  {
    if …
    {
      …
    }
    else
    {
      …
    }
  }
```

The
  assignment-operator,
  assignemnts to structure members,
  in `use`-statements,
  the `in` of `for`-loops and
  types of
    structure and
    enumerated type members as well as
    arguments
shall be on column 40 (first column is 0) and
  the value of assignments and the range of `for`-loops shall be on column 44,
The assignment-operator, types of structure and enumeration type member can be placed on a new line too.
When types of structure and enumerated type members or assignments to structure members are placed on a new line,
  the rules for brackets mentioned above apply.

```{.rust .numberLines}
  pub enum Foo
  {
    x,
    y                                   ( u64 ),
    z
    {
      a:                                i128,
      b:                                i128,
    }
  }

  for a                                 in  0 .. 42
  {
    b                                   +=  a;
  }
```

The `=>`-operators of a `match`-block shall be
  either on the same column
  or    each on a new line.
When the any operator,
  whether it is an assignment or `=>`-operator,
the value or the opening bracket of the block shall be placed on the same line but four columns deeper than the indentation-level.
Brackets shall be on the same column:

```{.rust .numberLines}
  match a
  {
    foo::x
    =>  foobar      ( 0,  0,  ),
    foo::y  ( b       )
    =>  foobar      ( 0,  b,  ),
    foo::z  { a,  b,  }
    =>  {
          if  a > b
          {
            foobar  ( b,  a,  );
          }
          else
          {
            c
            =   b;
            c
            +=  (
                  23
                );
            foobar  ( a,  c,  );
          }
        },
  }
```

In declaration of imutable variables with `let`,
  there should be 5 spaced between `let` and the identifier,
    so adding a `mut` does not change the column of the identifier.

```{.rust .numberLines}
  let mut aFoo                          =   23;
  let     aBar                          =   42;
```

This alignment to column 40 is a general rule.
There should be some space arround operators, brackets, keywords, identifiers and other tokens,
  but not in front of commas, semicolons etc.,
and they shall be on even columns.

Neither shall be blank characters on empty lines nor at the end of lines in apart from a line feed.
Lines shall exceed 200 characters.
Wrap lines, if possible.
The last line of a file shall end with a line feed.

Comments, Documentation and Error Messages
==========================================

Comments and error messages shall be written in British English without contractions or the oxford comma..
They should be understood as titles,
  therefore the rules of capitalisation applies,
    but they should end with a full stop or in some rare cases with an exclamation mark or question mark.
Ellipses shall be written with the intended unicode character instead of three full stops.
The meaning of values in error messages shall be clear without looking into the code.
Do not just drop some values in the logs.

Comments should regardless of whether they are single or multi-line, always be started with ›//‹ on each line.
They shall be on the line before the line to which it is referring to and on the same indentation-level and
  there the text of the comment shall start on even columns.
Short comments on
  multiple declarations of variables,
  declaration of constants or
  structure and enumeration type members
can follow on the same line,
  but the ›//‹ of each comment shall be on the same column:

```{.rust .numberLines}
  struct  Rofl
  {
    a:                                  usize,  //  this is a.
    b:                                  u64,    //  this is b
  }
```

Long sentences should be wrapped in appropriate places,
  such as commas or conjugations, and
  the lines should be well indented,
    like this text.
There shall not be more than one sentence on a line.
An empty line precede the comment,
  but there shall not be an empty line after,
    unless the comment is a placeholder,
      then both in front of and behind the comment shall be an empty line.
Do not be frugal with words, there are plenty of them.

If you have idea for a feature, todo-thing/issue, concerns or other useful change to the code,
  then write it down right away!

Comments for documentation shall be in front of the object to which they are referring to:

```{.rust .numberLines}
  /// This function does something.
  ///
  /// `a`   – This is a.
  /// `boo` – This is boo.
  /// `far` – This is far.
  pub fn  something
  (
    a:                                  usize,
    boo:                                String,
    far:                                isize,
  )
  ->  u16
  {
    0x1337
  }
```

Block comments should be used to comment out code,
  single-line comments can be used to comment out a single line.
Space between ›//‹ and the code is not necessary.

Types
=====

Types should not be aliased with `type`,
  because it does not have type checking.
Tuple-like structures could be used instead,
  where the value in the original type can be assigned to `bar.0`,
or enumeration structures:

```{.rust .numberLines}
  #[derive(PartialEq)]
  struct  StructBar                     ( usize );
  bar.0                                 =   23;
  baz.0                                 =   42;

  if  bar ==  baz
  {
    println!  ( "yay" );
  }
  else
  {
    println!  ( "oh?" );
  }
```

C-like enumeration structures,
  but with aliases for variants could be achieved with the `Const!`-macro:

```{.rust .numberLines}
  #[macro_use]
  extern crate const_type;
  Const!
  {
    pub Bar:                            u8
    {
      A                                 =   1,
      B                                 =   2,
      C                                 =   2,
      D                                 =   Bar::A.0,
    }
  }

```

Object Oriented Programming
---------------------------

Classes shall be implemented with a `struct` and an `impl`.
The constructor shall be named the same as the class and
  the definition of the constructor shall be between the structure and the implemenation of methods.
The class-name shall be on the same column,
  unless it is the return-type.

```{.rust .numberLines}
  pub struct  Class
  {
    …
  }

  pub fn      Class
  (
    …
  )
  ->  Class
  {
    …
  }

  impl        Class
  {
    …
  }
```

Trait-implementations,
  unless it is for the destructor,
shall be placed behind.
The destructor definition of the destructor could be placed behind the definition of the constructor.

Values
======

Hexadecimal numbers shall
  start with `0x`,
  have only decimals and lower case letters (0–9, a–f),
  have an even length and
  should be as wide as the corresponding type,
    e.g. `0x00000001` for a 32-bit-value.
  

Lists
=====

All members or entries of lists and list-like things like
    lists of
      arguments,
      array, vector, structure, enumeration type and tuple members,
      types of a specific form of a generic type,
      …,
    pattern matching arms,
    etc.
  shall end with a comma.
If the list has only one member or entry and
  is written on one line,
the comma could be ommited,
  unless it is a single argument of a method or enumeration type member,
    where similar methods or enumeration type members are used elsewhere.
In the latter case,
  the arguments and even the parenthesis shall be on the same columns,
    unless a function has a very long argument,
      e.g. an error message.

```{.rust .numberLines}
  let     aList                         =   vec!  ( 23, 42, );
  fn aFunc
  (
    a:                                  usize,
  )
  ->  Result
      <
        Fuu,
        String,
      >
  {
    match a
    {
      1 =>  Ok
            (
              Fuu::A  ( 1,  2,                )
            ),
      2 =>  Ok
            (
              Fuu::B  { 1,  2,  4,            )
            ),
      3 =>  Err
            (
              format! ( "Something is wrong." ),
            ),
      _ =>  fuubar    ( 23,                   ),
    }
  }
```

but:

```{.rust .numberLines}
  let     abc                           =   ( 123 );
```

Crate, Mod and Use
==================

`extern crate`s, `crate`s, `mod`s (public or not) and `use`s shall be in this order and
  each block seperated by an empty line.
The items of these ›lists‹ shall be in alphabetic order,
  but the `use`-declaration of items shall precede the `use`-declaration of modules and
  the `use`-declaration of `crate`, `self` and `super` shall always precede the other `use`-declarations and in this order.
As much as possible shall be declared in a single `use`-block,
  but
    neither the wildcard `*` shall
    nor unused imports
  shall be used.
`use`-declarations of modules shall always in square brackets.
The rules for brackets as well as for list members mentioned above apply.
Relative imports shall be preferred over absolute paths:

```{.rust .numberLines}
  extern  crate bar;
  extern  crate foo;

  mod bazfaz;
  mod boofar;
  mod didum;

  use crate
  {
    bazfaz::
    {
      A,
      B,
      ccc::
      {
        C,
        D,
      },
    },
    boofar::
    {
      P,
      Q,
      R,
    },
    didum::
    {
      X,
      Y,
      Z,
    },
  };
```

Errors and Warnings
===================

It is good practise to reduce the amount of warnings to zero.
Changes should not be commited,
  if the compilation throws warnings.

Warnings about the non-snake-case and non-upper-case-globals (see naming conventions) shall be suppressed with:

```{.rust .numberLines}
  #![allow(non_snake_case)]
  #![allow(non_upper_case_globals)]
```

If the final pattern of an `match` is unreachable,
  it could be commented out,
    but shall not be just removed,
      because perhaps the enumeration type will be extended.

Panics and crashed shall be avoided as much as possible.
