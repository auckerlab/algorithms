# Recursive Problems

Recursive is way to split the problem into the little problem, until we get a little problem that we can solve easily. This splited basic problem can be solved easily, then we combine the basic little problem into the big problem. Because the problems are similier, as we can reuse the solutions, so recursion involves the self incursion.

A easy example: if you want to calculate the sum of [2, 1, 7, 4, 5], most intuitive method is use a accumulator add every number one by one, like:

```rs
fn nums_sum(nums: Vec<i32>) -> i32 {
    let mut sum = 0;
    for num in nums {
        sum += num
    }
}
```

But what if there is no `for` or `while`? Then how to calculate the sum?

As we can see:

```txt
    2 + 1 + 7 + 4 + 5 = ((((2 + 1) + 7) + 4) + 5)
                  sum = (((3 + 7) + 4) + 5)
                  sum = ((10 + 4) + 5)
                  sum = (14 + 5)
                  sum = 19
                      = (2 + (1 + (7 + (4 + 5))))
                  sum = (2 + (1 + (7 + 9)))
                  sum = (2 + (1 + 16))
                  sum = (2 + 17)
                  sum = 19
```

The math expression is:
$Sum(nums) = First(nums) + Sum(restR(nums))$ or $Sum(nums) = Last(nums) + Sum(restL(nums))$

## 3 basic principle of Recursive

1. recursion must have basic situation
2. recursion must get close to basic situation
3. recursion must invoke self with recursion
