# Measuring Algorithms

Methods for uniformly measuring algorithms. 

Measuring an algorithm effectively requires doing so in some kind of objective space. In other words, given something like a problem set of increasing size, how efficiently does one algorithm manage the problem set compared to another?

*Visual representation of what you're trying to do when analyzing two algorithms f0 and f1.*
```
# given
problem = [0, ... 1000000]

# how to
f0(problem) vs f1(problem)
```

Where `f0(problem) vs f1(problem)` should express the comparison between the two algorithms in a measurable way.

## Big O

Big O notation is a method for classifying an algorithm and does so to achieve the goal laid out above, but focuses on specifically expressing the upper-bound or worst-cases.

### O(N)

```
# given
data = alphabet

# do
for letter in data
  count
```

N = 26

This algorithm is considered to run in linear time.

### O(1)

The previous example will find the *size* of the problem set (the alphabet) by counting each element (letter) in the set. Now imagine the alphabet in our new function is a special *thing* that has an attribute *len* (length). And, in fact, any problem set we give this new algorithm has a *len* available.

```
data = special_alphabet

# do
len = data.len
```

Our new algorithm will run in O(1) time regardless of the size of the problem set.
