This is an implementation of [Dijkstra](https://en.m.wikipedia.org/wiki/Edsger_W._Dijkstra)'s [Two Stack Algorithm](https://switzerb.github.io/imposter/algorithms/2021/01/12/dijkstra-two-stack.html), which is a way to *linearly* execute infixed arithmetic expressions, which have a naturally *recursive* syntax.

Here are the steps to the algorithm:

1. While there are still items to read
    1. Get the next item
    2. If the item is:
        - A number: push it onto the value stack.
        - A left parenthesis: push it onto the operator stack.
        - A right parenthesis:
            1. While the top of the operator stack is not a left parenthesis
                1. Pop the operator from the operator stack.
                2. Pop the value stack twice, getting two operands.
                3. Apply the operator to the operands, in the correct order.
                4. Push the result onto the value stack.
            2. Pop the left parenthesis from the operator stack
        - An operator op:
            1. While the operator stack is not empty, and the top of the operator stack has the same or greater precedence as op,
                1. Pop the operator from the operator stack.
                2. Pop the value stack twice, getting two operands.
                3. Apply the operator to the operands, in the correct order.
                4. Push the result onto the value stack.
            2. Push op onto the operator stack.
2. While the operator stack is not empty,
    1. Pop the operator from the operator stack.
    2. Pop the value stack twice, getting two operands.
    3. Apply the operator to the operands, in the correct order.
    4. Push the result onto the value stack.
3. At this point the operator stack should be empty, and the value stack should have only one value in it, which is the final result.
