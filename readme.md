# Trulang
So, what is trulang?

Trulang is an interpreted language that is designed to be a simple, easy to learn, and easy to use programming language.


# Syntax
So, how is the syntax of this language?
```
x : [ Int ] | 6 |
. + x 7
```
Don't let this syntax scare you, it is really easy once you understand it.

Here, we are defining a variable `x`. The assignment operator in trulang is `:`. In Trulang, almost everything is a function. Here, we are defining that `x` is a function, who's return type is `Int` (written between the brackets). The function takes no arguments, and returns the value `6`. The last statement of the function is automatically made its return value. So, `x` is a function that returns `6`. Then in the next line, there is a `.`(period), which is equivalent to `print` in other languages. It takes 1 argument and prints it. Here the argument passed is ` + x 7`. `+` is another function, which takes two numbers and adds them. So, `+ x 7` is the same as calling x, and then adding 7 to it. Finally, 13 is printed