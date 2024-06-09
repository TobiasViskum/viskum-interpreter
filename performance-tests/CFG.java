import java.io.*;

class GFG {

    // Function to print the fibonacci series
    static int fib(int n)
    {
        // Base Case
        if (n <= 1)
            return n;

        // Recursive call
        return fib(n - 1) + fib(n - 2);
    }

    // Driver Code
    public static void main(String args[])
    {
         int n = 35;

        // Record the start time
        long startTime = System.currentTimeMillis();
        
        // Calculate the Fibonacci series
        int result = fib(n);
        
        // Record the end time
        long endTime = System.currentTimeMillis();

        // Print the Fibonacci series
        System.out.println("Fibonacci series for n=" + n + ": " + result);

        // Calculate and print the elapsed time
        long elapsedTime = endTime - startTime;
        System.out.println("Time elapsed: " + elapsedTime + " milliseconds");
    }
}