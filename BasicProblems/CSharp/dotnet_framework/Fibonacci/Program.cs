using System;
using Helpers;

namespace Fibonacci
{
    internal class Program
    {
        static void Main(string[] args)
        {
            Console.WriteLine(NthFibonacci(Helper.GetNumberFromUser()));
            Console.Read();
        }

        static int NthFibonacci(int index)
        {
            if (index < 0) return 0;
            if (index == 1) return 1;
            return NthFibonacci(index - 1) + (NthFibonacci(index - 2));
        }
    }
}
