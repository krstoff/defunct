# defunct
A new implementation of a dead language.

## Syntax and Semantics
An atomic value (a Float, Nil, or a String) all evaluate to themselves.
A symbol evaluates to the value bound to it in the environment.
A list evaluates to the result of applying the function represented by the symbol
in its car (head) to the arguments represented by its cdr (tail).
Evaluating a function results in a runtime error.

A list is written in the form (values*).

## Built-in Functions
`+` adds two numbers together.

`*` multiplies two numbers together.

`exit` takes no arguments and exits a session.

`-` subtracts one number from another.

`not` takes an argument and returns T if it is Nil.

`<` (and `<=`) take two arguments and returns T if the first is less than (or equal to) the second.

`>` (and `>=`) takes two arguments and returns T if the first is greater than (or equal to) the second.

## Special Forms
Special forms are not evaluated like normal functions.

`(lambda (args*) (expr*)`

Creates a function object that can be applied to arguments.

`(if (test-form) (then-form*) (else-form*)`

If test-form evaluates to NIL, evaluates the then-form. Else, evaluates else-form.

`(define symbol (expr))`
Evaluate expr and bind its value to symbol in the nearest enclosing environment frame.

`(quote form)`
Returns the form, verbatim.

`(do expr*)`
Evaluates any number of expressions, returning the last value. Also known as `progn`.

## FAQ
Q: Why did you call it "defunct"? Are you bashing LISP?  
A: It is tongue-in-cheek pun of "defun", the function defining form in Common Lisp. Curiously, Lisp will never die.

Q: How can I use this?  
A: Just run it and enter valid lisp forms.

Q: What can I do with this?  
A: Just defining basic functions over arithmetic.

Q: Why did you write it in Go?  
A: To learn go.

Q: Will this get better?  
A: I have no plans of improving this. This was my first Go project as a learning exercise and I'm happy it works!
