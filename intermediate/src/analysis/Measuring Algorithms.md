# Measuring Algorithms

Note that this is a simplification -- and to some an oversimplification -- of asymptotic notation. See [this stackoverflow response](https://cs.stackexchange.com/a/61) for details on the depth of this topic.

<hr>

**Methods for uniformly measuring algorithms.**

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

### O(log(n))

Algorithms can be designed to perform more efficiently depending on the problem they are trying to solve. One example of a somewhat intuitive, more efficient algorithm would be binary search. This algorithm involves splitting the problem into two and evaluating on which side of the split your target exists.

*Using the alphabet again (note the alphabet is sorted), find target_letter.*
```
# given
data = alphabet

# do
halves = alphabet/2
data = half if target_letter in half

# repeat ...
```

This splitting search can be repeated until the target is found. This algorithm is classified as O(log(n)).


## Ω (Big-Omega)

Opposite to Big-O, Ω is used to describe the best-case or lower-bound of an algorithm.

Using binary search, the best case scenario would be Ω(1) if the value you split on is the value you are searching for.

*Find the number 5.*
```
# given
data = [1, 2, 3, 4, 5, 6, 7, 8, 9]
                    ^
```

Where the first position you happen to split on in order to search to the right or left of, you notice that it is the number you are searching for. In this scenario binary search is classified to run in Ω(1) time.


## Θ (Theta)

Θ is used to classify an algorithm that has a Big-O and Ω of the same complexity. Note that Θ is sometimes thought of as the *average-case*, but it depends on the context of how the classification is intended to be used or interpreted. 

*Going back to the special alphabet example...*
```
data = special_alphabet

# do
len = data.len
```

This would be classified as Θ(1) since its upper-bound would be O(1) and its lower-bound would be Ω(1). This algorithm will always run the same regardless of the problem set and its best-case is equal to its worst-case.