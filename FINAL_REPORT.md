% Memory Safety and the `rust` Language
% Ryan Hedgecock - CS166 - Instructor: Chao-Li Tarng
% Submitted: November 28, 2022

\newpage

# Overview
We live in a world that continues to be dominated by technology; and that is showing no signs of slowing down. Computers keep getting faster and cheaper, while being relied upon more heavily for increasingly important things. Rightfully so too. Computers have enabled us to catapult ourselves into the future at rates that would have traditionally been very alarming.

A problem that it has never solved however, is *human error*. And we are ripe with human error. Computers, of course, automate a lot of processes in many different ways. This inevitably reduces the ability for human error to happen. But not until very recently have we started to really concern ourselves with the human error of the programmers in charge of these computers.

A systems language is a programming language that runs on the metal of a system. These are traditionally `C` and `C++`, among a vast assortment of less popular ones. Their staple is *manual memory management* because when you are running on the metal, this is how you get performance and efficiency on low-spec machines. Or rather, thats how it has been for the longest time. The issue with manual memory management, is that it is incredibly prone to *memory safety* errors. These are often quiet, insidious errors that if you are lucky will just rear their head and **`SEGFAULT`**. But a lot of the time they just lurk in the shadows committing the sins of *undefined behavior*. These are *often* exploited by nefarious actors to perform all sorts of computer attacks, such as buffer overflow attacks. These are especially nefarious because it allows for complete arbitrary code execution on the subjects system.

In the modern era however, we have learned a bit about how to stay safe when writing software. We will feel the repercussions of our ancient past for a long time, but its about time we start making changes and learning from our mistakes. The chief mistake being that software engineers had too much trust in themselves. No human can write perfect code, and computers cant write code themselves. This is the idea behind the `rust` programming language. Rust has a system of data ownership built into its compiler, that can *provably* keep a program memory safe. While it may seem to get in the way of the engineer sometimes, what its really doing is walling off the areas where we so boldly went before, and are now paying the price.

\newpage

# Objectives

We will deep dive into the `rust` programming language to explore the true depth of the safety that it provides. We will also assess a buffer overflow attack and how it can occur in `C`, and how it may be achieved/prevented in `rust`.

# Architecture

We will use `TCC` for compiling our `C` code, and `cargo` (`rust`'s package manager) to build and manage our rust code.

# Results and Analysis

As it turns out, I was not prepared for the depth of the memory safety that `rust` provides. Keep in mind, this is also happening while rust is equaling `C` even in its ability to execute things quickly. In fact, for simple programs (thus easy to compare the assembly), `rust` compiled to almost identical assembly that the `C` code compiled to.

While `C` and `C++` have manual memory management, it leads to ways of *EASILY* causing memory safety bugs. But rust to my surprise NEVER makes you manage your own memory, *AND* theres no garbage collector either. It uses this fancy idea in the compiler called an data ownership model.

`rust` uses a data ownership model, but what does that mean? I didn't quite understand originally, which led me to believe that, "well, it cant be perfect... right? Theres ALWAYS a way to wiggle in and mess something up in a programming language. Humans are great at messing things up". Boy was I wrong, maybe leading to what some may think is a less fun paper. Well, yeah I didn't get to break anything, but it was not for a lack of trying. `rust`'s ownership model is *incredibly* robust and works as follows[^1]:

1. All data is "owned" by something. Usually it is some variable or some `struct` that is the owner. The compiler keeps track of the owner at all times. Ownership may be transferred, and the data may no longer be read or written to at its previous location.

2. Data may be "borrowed" immutably as many times as you like. The compiler keeps track of every borrow.

3. Data may only be "borrowed" mutably, if it is not already borrowed somewhere else. The compiler also keeps track of this. This prevents ANY AND ALL race conditions. It is impossible to have two mutable accesses to the same data. It is impossible to have read access to data that is currently mutably borrowed.

4. All data structures are explicitly bounds checked for every necessary insertion/read. Arrays have their size baked in as a part of its type. However, the performance overhead is slim to none, since the bounds are able to be checked by the compiler most of the time. This is why arrays have bounds in their type.

5. Use of pointers is strictly prohibited without an `unsafe` block. And they aren't needed. Pointers used to be the necessary way to pass data around without having to `memcpy` large structs. But with the borrow system, its no problem. Pointers are also useful when iterating through data quickly, but that is no longer necessary with `rust`'s robust iteration system.

There are MANY more things that will address any specific ideas. But those cover pretty much all memory safety issues in modern programming. In fact, these make it so airtight, that the only additions to the language is to cover concerns about verbosity and to add convenient features to make the programmers life easier.

#### Github Link: [https://github.com/sjsu-students/cs166-final](https://github.com/sjsu-students/cs166-final)

# Future Work Suggestion

In the future, I will continue to work with rust regardless of writing papers. The memory model is very compelling. It would be fun to do some performance tests to see how far `rust` may stray from peak speed compared to similar programs written in other languages like `C` or `C++`.

# Conclusion

While I thought I would go into this and exploit a few small things in the new and fancy `rust` programming language, I was surprised to run into such an airtight system. I guess it should be obvious to me now, as this was the design goal of `rust` in the first place. *Fearless memory safety*. Its not constructed around some complex crazy system to that does its best to keep you safe; Its designed around a small number of *CORE* principals in its ownership system that cover EVERY case that could occur at scale. This has always been the best way to design anything. The smaller number of moving parts, the more confidence you will have in the system. And `rust`'s parts lay a solid foundation for any structure to be built, while avoiding the unstable raw memory land below.

\newpage

# References

Klabnik, S., & Nichols, C. (2018). *The rust programming language*. No Starch Press. 

Cartas, C. (2019). Rust - the programming language for every industry. Academy of Economic Studies.Economy Informatics, 19(1), 45-51. https://doi.org/10.12948/ei2019.01.05

[^1]: Klabnik, S., & Nichols, C. (2018). *The rust programming language*. No Starch Press. 

# Contribution
Ryan Hedgecock (Sole Member)
