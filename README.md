# Ackee-Blockchain wk 2 assignment

The first assignment for the Winter course was to read the Solana Handbook, which you can find [here](https://ackeeblockchain.com/solana-handbook.pdf) assignment was to write a simple calculator in Rust. If you have read The Book, this book would have helped more than enough you along.

Writing a simple calculator falls roughly in the same category for me as a Todo app or a counter you see everywhere in tutorials. My first attempt at writing any code was an instruction from a guy with a backward baseball cap on YouTube in Python.

Rust may get a bit tricky (for me still, even with this simple calculator had me stuck on `String` vs. `str` in the function signature), but hey shouldn´t be too easy, right?

The [code](https://github.com/MWest2020/Ackee-Blockchain/tree/main/calculator) should suffice for this week's homework, but still, I am not pleased with the program as is. I remember that calculator app from years ago in Python, and I could run it everywhere on my laptop as a CLI application.

Lazy is my middle name, so I turned to chatGPT and asked if it could rewrite the program to a CLI application. Lo and behold, it spews out the code within seconds. It also gave me a good spanking on writing more concise code for the calculate function. Having [this code](https://github.com/MWest2020/Ackee-Blockchain/tree/main/calc-cli) on a Linux machine will enable the simple calculator in a terminal window by using the command `calc {n} {operator}.

It made me wonder if ChatGPT can assist me in an attempt to rock some Rust frontend framework for the simple calculator, so thatś what I am going to do next.
