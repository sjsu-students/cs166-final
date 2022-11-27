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

`rust` uses a data ownership model. So what does that mean. 

Github Link: [https://github.com/sjsu-students/cs166-final](https://github.com/sjsu-students/cs166-final)
