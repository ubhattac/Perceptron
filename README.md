# Perceptron Learning Algorithm.

A perceptron is just a neural network unit that repeatedly performs computations in order to glean useful features or business intelligence in the input data.

![](https://miro.medium.com/max/645/0*LJBO8UbtzK_SKMog)

You should ideally [santize](https://realpython.com/python-data-cleaning-numpy-pandas/#tidying-up-fields-in-the-data) your data before using this API. You could probably use `pandas` and not break a sweat.

This is written is being written rust cuz it's [fast](https://kornel.ski/rust-c-speed) and [fun](https://stackoverflow.blog/2020/01/20/what-is-rust-and-why-is-it-so-popular/). I'm pretty busy with school and other things right now so you're heavily encouraged to contribute.

Keep in mind that this is a binary classifier so craft each line in your `.csv` in the format of `<binary class [0 or 1]>, cartesian x, cartesian y`.

After you've done all that (and once this project is complete) it gets damn simple: `cargo run <path_to_.csv>`

If you're interested in seeing it work on a small dataset

```
cargo run examples/small.csv
```

With the addition of a visualizer, the goal is to be able to see the perceptron updates in real-time. 
![](https://demonstrations.wolfram.com/PerceptronAlgorithmInMachineLearning/img/popup_2.png)

### Tech Debts

1. Finish the model trainer. 

2. Create unit tests for the more nuanced datasets which may or may not be linearly separable.

3. Try to keep the entire package under 450 lines of code.