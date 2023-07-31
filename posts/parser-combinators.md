---
title: A gentle introduction to parser-combinators
tags: [parsers, combinators, parsing, rust, chumsky, AST, programming]
date: 2023-07-31
blurb: "Discover the fundamentals of parser-combinators, offering a gentle introduction to the art of parsing."
---

# A gentle introduction to parser-combinators

Hang around programmers long enough, and you’ll often hear of *parsers* or *parsing*. Let’s find out what it is and what we use it for.

Parsers are programs that can analyze and process input, typically text, according to a set of rules. They are commonly used for tasks such as validating user input, interpreting programming languages, or extracting data from structured text. 

Naturally, parsing is the act of running a parser over input. For example, a JSON parser will accept text and output an object.

With that out of the way, let’s design a parser.

> ********Note********: All code will be written in Rust for the purposes of this article.
> 

## Parser Combinators

Parser combinators are a way of building parsers by combining smaller parsers into larger ones. Each parser combinator takes one or more parsers as input and returns a new parser as output. By chaining together parser combinators, we can create more complex parsers that can handle a wider range of inputs. This approach is often used in functional programming languages, such as Haskell, where parsers are represented as functions that can be composed together.

You might ask; but why use Parser Combinators? Surely you can write a parser by hand instead of chaining multiple parsers together?

## Why use Parser Combinators?

Parser combinators offer several advantages over hand-written parsers.

They are often easier to read and understand. By breaking down the parser into smaller, composable parts, it becomes easier to reason about each part of the parsing process.

Parser combinators are often more modular and reusable than hand-written parsers. Once a set of parser combinators has been defined, it can be used to parse many different inputs.

Parser combinators often result in shorter, more concise code. This is because they allow complex parsing logic to be expressed in a more declarative style, rather than a procedural style.

It is also difficult and time consuming to write parsers with good error recovery mechanisms. It requires understanding the intricacies of the recursive descent algorithm, and then implement recovery strategies on top of them.

If you’re writing a programming language, there will almost certainly be changes in the syntax and grammar in the process, which can cause time-consuming and painful refactoring. Parser combinators solve both error recovery and refactoring by providing an easy to use API that allows for rapid prototyping.

Parser combinators are an excellent choice for domain-specific languages that lack an existing parser. With a reliable and fault-tolerant parser combinator library, what would have taken several days to accomplish can be achieved in as little as half an hour.

## Abstract Syntax Trees

Abstract Syntax Trees (ASTs) are a way of representing the structure of code or data in a program. They are used by compilers and interpreters to transform source code into executable code, or to analyze data structures. 

An AST is a tree-like data structure that represents the syntactic structure of the code or data, without including information about the specific characters used to write the code. Instead, it captures the essential structure of the code, including the relationships between different parts of the code and the order in which they appear. ASTs are often used in conjunction with parser combinators to build parsers for programming languages or other structured data formats.

An example AST for a parsed JSON file is shown here:

```rust
Some(
    Object(
        {
            "acres": Str(
                "home",
            ),
            "leaving": Object(
                {
                    "front": Str(
                        "college",
                    ),
                    "origin": Num(
                        981339097.0,
                    ),
                    "cowboy": Num(
                        -355139449.0,
                    ),
                    "fed": Num(
                        -283765067.9149623,
                    ),
                    "tail": Array(
                        [
                            Num(
                                -2063823378.8597813,
                            ),
                            Bool(
                                true,
                            ),
                            Null,
                            Num(
                                -153646.6402,
                            ),
                            Str(
                                "board",
                            ),
                        ],
                    ),
                    "although": Num(
                        -794127593.3922591,
                    ),
                },
            ),
            "activity": Str(
                "value",
            ),
            "noise": Bool(
                false,
            ),
            "office": Num(
                -342325541.1937506,
            ),
        },
    ),
)
```

## Our first parser

We’re going to write a simple CLI tool to parse JSON and generate an AST (Abstract Syntax Tree) called `bourne`. We will write it in Rust, and use the `chumsky` parser-combinator library. 

Note that our parser will not handle the full JSON specification, such as escape characters or exponential numbers, but it should handle most common JSON, such as nested arrays, objects, numbers, strings, booleans and null values.

### Creating the project

Create a simple rust binary project using `cargo`:

```bash
cargo new bourne
cd bourne/
```

Add the `chumsky` and `ariadne` libraries to `Cargo.toml`

We’ll be using `ariadne` for effective error recovery and reporting.

```toml
[package]
name = "bourne"
version = "0.1.0"
edition = "2021"

[dependencies]
ariadne = "0.3.0"
chumsky = {git = "https://github.com/zesterer/chumsky", rev = "3b1ab31"}
```

### Read a JSON file from stdin

To parse some JSON, we first have to read it into memory. For this purpose, we’ll take a file path through stdin and read it.

```rust
use std::{env, fs};

fn main() {
	let src = fs::read_to_string(env::args().nth(1).expect("Expected file argument"))
        .expect("Failed to read file");
}
```

### Define the AST model

We will define a recursive AST based on the JSON value type as a Rust enum.

```rust
use std::{collections::HashMap, env, fs};

#[derive(Clone, Debug)]
enum Json {
    Invalid, // invalid node
    Null, // null node
    Bool(bool), // boolean node with its value
    Str(String), // string node with value
    Num(f64), // number node with value
    Array(Vec<Json>), // array node with values
    Object(HashMap<String, Json>), // nested object with child key value pairs
}

fn main() ...
```

### Build a parser function

Finally, let’s build a `parser()` function that will return a recursive parser that can take in string input and return the built AST and any errors that might be encountered.

```rust
use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::prelude::*;

#[derive(Clone, Debug)]
enum Json {...

fn parser<'a>() -> impl Parser<'a, &'a str, Json, extra::Err<Rich<'a, char>>> {
	recursive(|value| {
	})
}

fn main() {
	...
	let (json, errs) = parser().parse(src.trim()).into_output_errors();
    println!("{:#?}", json);
    errs.into_iter().for_each(|e| {
        Report::build(ReportKind::Error, (), e.span().start)
            .with_message(e.to_string())
            .with_label(
                Label::new(e.span().into_range())
                    .with_message(e.reason().to_string())
                    .with_color(Color::Red),
            )
            .finish()
            .print(Source::from(&src))
            .unwrap()
    });
}
```

Let’s break down all the changes here.

1. First, we defined a function called `parser` that returns an implementation of the `Parser` trait defined by `chumsky`. This trait takes in a few type parameters, which are described below:
    1. Parsers are parameterised over the lifetime of their inputs. Because we don't yet know what input our parser will be used to parse, we declare a generic lifetime, `'a`, to allow the parser to work with whatever input lifetime it needs to work with.
    2. The first type parameter (i.e: ignoring the lifetime parameter) of the [`Parser`] trait is the input type. Inputs must implement the [`Input`] trait. Examples of inputs include strings, slices, arrays, [`Stream`]s, and much more. For now we are specifying that this parser can only operate upon string slices: but it is also possible to introduce the input type as a generic type parameter like `I: Input<'src>` instead if you want your parser to be generic across more than just string slices.
    3. The second type parameter of the [`Parser`] trait is the output type. This is the type of the value that the parser will eventually give you, assuming that parsing was successful. Here, we use the output type of [`Json`], i.e: the enum we just defined to represent our AST.
2. Next, we returned the value returned by the `recursive` function, which takes in a callback where we will actually construct the parser. There are a few things worth paying attention to here:
    1. `recursive` allows us to define a parser recursively in terms of itself by giving us a copy of it within the closure's scope. 
    2. You might ask, why do we need a recursive parser? It’s because we want to be able to handle nested JSON nodes, such as arrays and objects.
3. Finally, we call `Parser::parse()` on the parser returned by the `parser()` function we just defined. This takes in the input and returns the output and errors encountered during parsing. We then print the returned output and errors. 
    
    Note that since we’re using a `Rich` error, we’ll get a lot of helpful information about what went wrong for free from chumsky!
    

### Building the parser

Let’s get down to the meat and potatoes, actually building the parser! Parser combinators allow us to declaratively build parsers and combine them to build large parsers. 

- Let’s start off by extracting all numbers.
    
    ```rust
    recursive(|value| {
    	let number = just('-')
                .or_not()
                .then(text::int(10))
                .map_slice(|s: &str| s.parse::<f64>().unwrap());
    })
    ```
    
    Here, we define a parser for numbers which is composed of smaller parsers. Let’s break it down:
    
    - The `just()` primitive parser recognises a specific token or an exact ordered sequence of tokens. Here, we combine it with the `or_not()` combinator, which attempts to parse a pattern, always succeeding with either [`Some(...)`] or [`None`] depending on whether parsing was successful. We use it here to optionally parse a negative number beginning with `-`
    - The `then()` combinator parses one pattern and then another, producing a tuple of the two parsers outputs as an output. We use it to chain the actual number parser, which is `text::int()`. It is provided by `chumsky`, and lets us parse integers with any radix. Here we specify radix `10` .
- Now, let’s build a parser for strings.
    
    ```rust
    recursive(|value| {
    	...
    
    	let string = none_of("\\\"")
                .repeated()
                .collect::<String>()
                .delimited_by(just('"'), just('"'));
    })
    ```
    
    Let’s break this down as well.
    
    - First, we use the `none_of()` parser, which parses a single token that is *not* part of a given sequence of tokens. Here, we use it to parse all tokens except `\"`, which is the opening and closing double quotes character.
    - Next, we call `repeated()` to repeatedly parse the `none_of()` pattern.
    - Then, we collect the result of the repeated parsing into a `String`. Note that this is possible because each token parsed by `repeated()` is a `char`.
    - Finally, we apply a combinator `delimited_by()`, which parses the above pattern, delimited by two other patterns on either side. Most often used to parse parenthesiesed expressions, blocks, or arrays.
        
        You might wonder why we need this combinator when we’re already ignoring `"`. The reason is, the parser will not match the first token of the string, `"` unless we specify something to consume it. There are other ways to consume this character, for example, we could  consume it using `just('"')`, and then chain another `just('"')` after the `none_of()` pattern, like the following, but using the `delimited_by()` combinator just looks cleaner. 
        
        ```rust
        just('"')
          .ignore_then(none_of("\\\"").repeated().collect::<String>())
          .then_ignore(just('"'));
        ```
        
- Now, let’s build a parser for members, i.e anything after the `:` in JSON
    
    ```rust
    recursive(|value| {
    	...
    
    	let member = string.clone()
    							.then_ignore(just(':').padded()).then(value.clone());
    })
    ```
    
    There are several interesting things going on here, let’s break them down.
    
    - First, notice that we call `clone()` on the `string` parser. That’s because the logic for parsing a JSON key is the same as a string, since a JSON key is indeed, a string. Thus, we can reuse the string parser for the first part of a member.
    - Next, we read and ignore the `:`, it doesn’t give us any useful values so we just consume it and ignore the token. Note that the parser for this has a `.padded()` call. This also consumes any whitespace around the `:`.
    - Finally, we chain it to `value`. This can seem a little confusing. What exactly is value here? We know it’s the argument provided to us by the `recursive()` callback. That is a hint as to its function.
        
        `value` is a reference to the top level parser, which allows us to recursively call our top level parser on child values. In this case, we chain it to the member because the “value” part of JSON can be a number, string, object or array, so we have to call the top level parser again to parse those. This is what we mean by a recursive parser.
        
- Next, let’s write a parser for arrays.
    
    ```rust
    recursive(|value| {
    	...
    
    	let array = value
                .clone()
                .separated_by(just(',').padded().recover_with(skip_then_retry_until(
                    any().ignored(),
                    one_of(",]").ignored(),
                )))
                .allow_trailing()
                .collect::<Vec<Json>>()
                .padded()
                .delimited_by(
                    just('['),
                    just(']')
                        .ignored()
                        .recover_with(via_parser(end()))
                        .recover_with(skip_then_retry_until(any().ignored(), end())),
                );
    })
    ```
    
    Whoa, there’s a lot going on here, let’s break it down as usual.
    
    - First, you’ll notice that we start off with a clone of `value`. This is because array’s children can be any of the valid JSON types, namely numbers, strings, objects, and yes, more arrays.
    - Next, we chain it to `separated_by()`. This behaves just like `repeated()`, except it requires the values to be separated by a sequence. Here, we separate by a comma `,` (along with whitespace).
    - You’ll notice a curious new function, `recover_with()` being used here. This is one of `chumsky`'s powerful error recovery combinators. Here, our strategy is to skip any erroring elements (i.e, elements that do not successfully match any of the defined parsers) until we get either a `,` or a closing `]`, which naturally corresponds to the next item, or the end of the array.
        
        In simple terms, skip any erroring tokens until the next item or end of array.
        
    - Next, we chain `allow_trailing()`. This is a useful combinator that allows a trailing separator for the `separated_by()` combinator. In simple terms, this will allow trailing commas in arrays.
    - Next, we `collect()` the parsed elements into a `Vec<Json>`. Naturally, this corresponds to an array of JSON nodes. Also we append `padded()` for good measure to gobble up any trailing whitespace.
    - Finally, we chain it to `delimited_by()`, which we use to mark the array as being surrounded by `[]`. Note that we also perform error recovery here. The exact strategy for error recovery here is left as an exercise for the reader 😇
- We’re in the home stretch here! We just need to write the parser for objects and then we’re done! Let’s get to it.
    
    ```rust
    recursive(|value| {
    	...
    
    	let object = member
        .clone()
        .separated_by(just(',').padded().recover_with(skip_then_retry_until(
            any().ignored(),
            one_of(",}").ignored(),
        )))
        .collect()
        .padded()
        .delimited_by(
            just('{'),
            just('}')
                .ignored()
                .recover_with(via_parser(end()))
                .recover_with(skip_then_retry_until(any().ignored(), end())),
        );
    })
    ```
    
    After breaking down the array parser, this should be straightforward to understand. Let’s understand what’s happening here.
    
    - First, we chain to the `member` parser, since JSON objects can be at the root, or as members.
    - Next, we again chain the `separated_by()` combinator, just like arrays, but with slightly different error recovery (`}` instead of `]`). Note that we don’t accept a trailing separator here because the JSON specification does not allow trailing commas after values.
    - We `collect()` all these values into a `HashMap<String, Json>`. You might be wondering how we can get both the JSON `key` and `value` from this parser; this is due to the `.then()` combinator on the `member` parser. `.then()` combinators return a tuple of the parsed data, of which the first element here is the JSON `key`, and the chained value is the JSON `value`.
    - Finally, we chain it to `delimited_by()` again, with similar error recovery as the `array` parser.
- Now, you might be wondering, we haven’t actually returned any of these values in the parser. Good catch! We will now combine all these parsers into a final one.
    
    ```rust
    choice((
            just("null").to(Json::Null),
            just("true").to(Json::Bool(true)),
            just("false").to(Json::Bool(false)),
            number.map(Json::Num),
            string.map(Json::Str),
            array.map(Json::Array),
            object.map(Json::Object),
        ))
      .padded()
    ```
    
    Here, we use another useful combinator called `choice`. It simply takes in a tuple of parsers, and returns the output generated by the first one to match. We chain it with `padded()` for good measure to gobble up any whitespace.
    

And that’s it! Our parser is ready.

Create a file called `sample.json` with some test JSON data, and run the parser like below:

```rust
cargo run -- sample.json
```

You should be able to see the output AST similar to this:

```rust
Some(
    Object(
        {
            "batters": Object(
                {
                    "batter": Array(
                        [
                            Object(
                                {
                                    "type": Str(
                                        "Regular",
                                    ),
                                    "id": Str(
                                        "1001",
                                    ),
                                },
                            ),
                            Object(
                                {
                                    "id": Str(
                                        "1002",
                                    ),
                                    "type": Str(
                                        "Chocolate",
                                    ),
                                },
                            ),
                        ],
                    ),
                },
            ),
            "type": Str(
                "donut",
            ),
            "id": Str(
                "0001",
            ),
            "name": Str(
                "Cake",
            ),
            "ppu": Num(
                55.0,
            ),
        },
    ),
)
```

## Conclusion

I hope this article has demystified parser combinators and given you a taste of how powerful they can be. If you’re interested in learning more, I encourage you to check out [chumsky](https://github.com/zesterer/chumsky), the parser combinator library used in this article, as well as other parser combinator libraries like [nom](https://github.com/rust-bakery/nom) and [combine](https://github.com/Marwes/combine).

That’s all for now, happy learning!