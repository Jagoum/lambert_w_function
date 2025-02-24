# lambert_w_function

- This program is to implement the lambert w function in math
The lambert w function is a mathematical approach that was propose by the mathematicain Lambert .W

This function is also called the the product log . And just like natural log (ln) is the inverse of the funtion (e), The lambert w function is also and inverse to the function f(x) = xe^x

The function helps us to solve complex mathemtics algebra and express it in terms of w(x).

- The domain of the lambert_w function(Product Log) is D:[-1/e, infinity)
And the range of the product log is R:[-1, infinity)

This function also uses the newton rashson's method to compute the value of the W(x) in and iterative loop.

> Examples of of problems that could be solved using the lambert w funtion are :

> 2^x = x^2;
x^x = 2;

 Solving this is actually imposible without graphing it but with lambert w function it makes it easier in that we just need to express our answer as f(x)e^(fx) = constant and then substitude it in the lambert w function

 ## How to use

 - To Contribute to this project you can fork the repository, add you suggestions and then do a pull request
 - To run the app, download it first using 
 ```
 docker pull ghcr.io/jagoum/lambert_w:latest
 ```

 -  Now that it has being downloaded run it using 
 ```
 docker run --rm ghcr.io/jagoum/lambert_w:latest arg
 
 for negative numbers, 

 docker run --rm ghcri.io/jagoum/lambert_w:latest -- arg

 ``` 
