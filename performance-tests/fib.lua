
function fib(n)
    if (n < 2) then
        return n
    end
    return fib(n - 2) + fib(n - 1)
end

startTime = os.time()
fib(40)
print(os.time()-startTime.." seconds since start of script execution...")