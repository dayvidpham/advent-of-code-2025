# Day 2 Notes

## Work Session 3

Okay. Back at it again.
Just woke up, haven't gotten so much sleep the last few days but that's my own fault xdddd.
I'm going to start just documenting my thoughts here just to keep track of stuff.

Understanding how `iter()` interacts with `Option` and `Result` was this weird sharp edge that I didn't understand for a while until Work Session 2.
Maybe it was just my LSP confusing me, where I would stare at its list of autocompletions to try and figure out what methods were available on which types ... but because of Rust's Trait system, each Type has like a bajillion methods from each Trait that it implements, and that shit clogs my screen every time the autocompletion menu shows up.
Then I'd get a little lost + curious just staring at all the shiny objects in there.
I see some people like Tsoding who I think don't even have autocomplete, only the syntax highlighting.
What a baller honestly.

Might be a workflow I might try getting into.
It would force me to stay more focused, and I would have to train myself to hold bigger contexts and usage patterns in my head.
This would definitely make unintuitive parts of code stick out a lot more.
I wonder if it would also help me write better code for myself, because I'd be forced to work within the bounds of my own memory.
Maybe?
That would probably make Rust at least a little bit easier.
I'm constantly trying to juggle between the 10+ different methods available on each type, there's not much learning happening.
Need more reps, need more sleep, but also *fuck* this is confusing.

Anyways.
For a long time I didn't understand why you can call `iter()` on `Option` and `Result`, or what `iter().next()` *really* did.
The main motivation: how did `flat_map()` use this to return only the `Some(x)` values of a stream (informal) that yielded `Option` values?

The core barrier here was that I didn't understand exactly what or how the `Iterator` Trait worked, or what it was expected to implement, but I looked it up and it kind of makes sense now?

Let's say we're talking about a type `Vec<T>`.

* editors note: I looked even more into it, and no ... it has gone back to not making sense. but that's good! that means I'm learning.
* I used my powerful LSP powers to jump to the definition of `Vec<T>::iter` ... and it was not what I expected. O_O. Despite more confusion, it was deeply insightful, and I was able to even further develop my understanding. PROGRESS.

Which Trait does the `iter()` method come from?
Does it even come from a trait, or is it just some bespoke convention that these iterable types implement?

* On my jump-to-definition LSP journey, I learned that .... the `Vec<T>` type does not exactly define an `iter()` method; the purpose of a `IntoIterator` trait (and its corresponding `FromIterator`); and also I'm not sure where `Vec` defines `iter()`? Is `Vec` actually a `VecDeque` under the hood??? well, that would probably answer the question of WHERE is it defined.

### On the Uselessness of the Rust Book

Section on [Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html), from The Rust Programming Language.

The book only discusses Iterators in some shallowness all the in the 13th breaking chapter, when they are such a core part of how the language builds up its semantics, inspired by functional programming.

I'm learning that the Rust book is kind of useless, and the best way to understand how these fundamental Traits (1) are expected to be composed, (2) actually work, and (3) are motivated ... is to look at the freaking Rust standard library code itself!

A snippet taken from the [std::iter](https://doc.rust-lang.org/std/iter/index.html) crate documentation, the crate that defines the Iterator trait:

---

# Iterator

The heart and soul of this module is the [`Iterator`] trait. The core of
[`Iterator`] looks like this:

```
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

An iterator has a method, [`next`], which when called, returns an
<code>[Option]\<Item></code>. Calling [`next`] will return [`Some(Item)`] as long as there
are elements, and once they've all been exhausted, will return `None` to
indicate that iteration is finished. Individual iterators may choose to
resume iteration, and so calling [`next`] again may or may not eventually
start returning [`Some(Item)`] again at some point (for example, see [`TryIter`]).

[`Iterator`]'s full definition includes a number of other methods as well,
but they are default methods, built on top of [`next`], and so you get
them for free.

Iterators are also composable, and it's common to chain them together to do
more complex forms of processing. See the [Adapters](#adapters) section
below for more details.

[`Some(Item)`]: Some
[`next`]: Iterator::next
[`TryIter`]: ../../std/sync/mpsc/struct.TryIter.html

# The three forms of iteration

There are three common methods which can create iterators from a collection:

* `iter()`, which iterates over `&T`.
* `iter_mut()`, which iterates over `&mut T`.
* `into_iter()`, which iterates over `T`.

Various things in the standard library may implement one or more of the
three, where appropriate.

# Implementing Iterator

Creating an iterator of your own involves two steps: creating a `struct` to
hold the iterator's state, and then implementing [`Iterator`] for that `struct`.
This is why there are so many `struct`s in this module: there is one for
each iterator and iterator adapter.

---

And there you have it!

The `iter()` function is a **convention**.
It's defined to return a custom `some::crate::thing::Iter<'_, T>` type that **implements the `Iterator<'a, T>` trait** by defining its method `next() -> Option<T>`!

Thus, **by definition**, `Iterator`s return `Option`s!
WHAT THE FUCK!
I thought this was just because I kept working with collections of Options.
But the `next()` method is DEFINED TO RETURN AN OPTION. OHHHH!

Once I realized this, I understood how `flat_map()` was motivated, how it was supposed to compose things together, and how it got out the underlying `Some`s.

`Iterator`s usually stop freaking iterating when they encounter a `None`.

So if I have

```rust
let xs: Vec<Option<i64>> = vec![Some(5), Some(6), None, Some(9)];
xs.iter()
    .flat_map(|ox| ox)
    .for_each(|x| println!("Found an {x:?}"));

// same output as
let ys: Vec<Option<i64>> = vec![Some(5), Some(6), None, Some(9)];
ys.iter()
    .flat_map(|oy| oy.iter().next())
    .for_each(|y| println!("Found an {x:?}"));
```

very coolio.
brain understandingington.

ok.

also interestingly, `Vec<T>::iter(&self) -> Iter<'_, &[T]>`.
that is, `iter()` returns a slice.
where is this even defined?

i looked around for a long time until I finally asked Gemini 3 Pro.
turns out ... it's because `Vec<T>` implements the `Deref` trait?

what the fuck.
that is fucking insanity.
what the fuck.

so when I try to call methods on `Vec<T>`, I may instead be calling methods on a `&[T]`.
and there wasn't really any great documentation on this. other than the fact taht i would have had to know that `Vec` implements `Deref`. what the fuck?

Okay okay okay.
Wow. Doing a deeper dive into the deep dive.
Things are slowly coming together on the story of `Vec<T>`.
I'm also understanding why it implements `IntoIterator` in the way that it does. 

This morning ... oh boy.
What a morning.
Really though, it's been fun.

I've spent a good 2 hours jumpng around in the `std` and `core` crates trying to figure out where `Vec<T>` was defining  `Vec<T>::iter()` and why in the bloody hell it returns a slice. turns out it implements the `Deref` trait with `type Target = [T]`.

This `Deref<T>` trait implements the dereference operator `*vec` you'd typically see in C, but it's also got some magic Trait behaviour that allows users of `Vec<T>` to unknowingly call the associated function (read: method) of the `Target` type, same way as they would call any other method, i.e.

```rust
let vec: Vec<i64> = vec![1, 6, 7, 10];
vec.iter() // actually dispatches to [T]::iter()
           // and there is no explicitly defined Vec<T>::iter()
```

I guess I was surprised because I didn't expect this form of non-obvious type-fuckery from the Rust world given they complain so much about C++ meta-programming, but in the end, I see they just prefer a more diffuse, loosely-coupled flavour of type fuckery.

```cpp
// In C would probably look like
SomeType* vec = ...;
vec->iter();

// or if C++ where reference
SomeType& vec = ...;
vec.iter()
// I guess it kind of mimics the "syntactic sugar" provided by C++ references?
```

Props to the Rust devs but also what in the fuck how do I know where anything I ever use is coming from :skeletonunderneath: 

