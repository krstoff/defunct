SYMBOL_CHAR := [a-zA-Z0-9+-*:_!]
SYMBOL := SYMBOL_CHAR+
NUMBER := '-'?[0-9]+('.'[0-9]*)?
LIST := '(' SEXP* ')'
SEXP := NUMBER | SYMBOL | LIST

A form is an sexp that represents a defunct program. To run a program is to evaluate the form.
Numbers and other constants evaluate to themselves. Symbols evaluate to the value that is bound to that symbol within its lexical scope.
Lists evaluate depending on the symbol at the head of the list. If the symbol designates one of the special forms, then it is evaluated according to that special form's rules. Otherwise, the list is interpreted as a function call, each element in the list is evaluated, and the head is applied to the rest of the list.

Special forms:

(if test resultant else-branch)
(let [binding-forms*] exprs*)
(fn [parameters] body)
(cond test1 expr1 test2 expr2 ...)