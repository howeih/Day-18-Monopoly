#Day 18: Monopoly
<br>
Some time ago a friend of mine asked me to help him with this problem.
<br>
Playing Monopoly, what is the probability that you step at position #24 during the first round?
<br>
Law of total probability says that the chance to step on certain position is sum of disjoint events of how we could get there. In this case, we get at #24 if we tossed 1 while standing at #23, or we tossed 2 while standing at #22, … or we tossed 6 while standing at #18.
<br>

It looks quite familiar, doesn’t it? Right, Fibonacci numbers. Strategy to choose? Dynamic programming.
<br>
install gnuplot first.
```
brew reinstall gnuplot --with-qt
```

```
fn main() {
    let z = probability(10);
    println!("{:?}", z);
}

output :
[0.16666666666666666, 0.19444444444444445, 0.22685185185185186, 0.2646604938271605, 0.30877057613168724, 0.36023233882030176, 0.25360439529035206, 0.26809401672763294, 0.28036894544149776, 0.289288461039772]
```
<br>

![Alt text](monopoly.png?raw=true "Monopoly")
