# Project description

This is a median and mode project where a vector with a random length (from a minimum of 5 to a maximum of 20) with randomized numbers (with a predeteremined upper and lower 
bound) will be spit out and the user has to correctly answer what the vector's median and mode are. 

The numbers inside the vector will always be random, but the user will be prompted to choose between randomizing the vector length and the upper + lower bound of the numbers (given it's within the predetermined min and max), or choosing those parameters themselves.

The vector that is spit out is unsorted, so correctly answering the median and mode can be non-trivial, especially on vectors with lengths above 15.

# How to run it

Clone the repo and make sure you have Rust installed in your VSCODE. Just type in 'cargo run' and you'll see it on your terminal.
If you use a different I.D.E., provided it can run Rust, then the process is likely similar.

# Problems

1. The user can't give more than one answer for the vector's mode. This can be incorrect as the vector can have more than one mode.
2. There should be a condition where if there's more than one mode, if the user correctly chooses at least one, then the program should recognize that and say that the user is partially correct.
3. Sometimes a vector has no mode. In this case, the program should recognize that. As of right now, if the program encountered that, it always chooses the first element in the vector. The user should be able to answer that the vector has no mode, and the program should recognize that's correct.

# Future fixes/planned changes

1. Fix everything listed in the problems section.
2. Introducing more modularity to the project by wrapping similar functions inside modules and/or separating said modules into files.

# Genesis of the project

I got the idea to make this from the Rust book as one of the three suggested exercises: https://doc.rust-lang.org/stable/book/ch08-03-hash-maps.html. It's at the end of the page.
I wanted to make this because the original vector exercise seemed trivial, but this turned out to be more of a hassle than I expected.