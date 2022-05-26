module Fibonacci
    function fibonacci(n)
        if n <= 2 
            1
        else
            fibonacci(n-1) + fibonacci(n-2)
        end
    end
end
