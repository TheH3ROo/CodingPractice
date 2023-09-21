using Helpers;
using System;
using System.Text;

namespace FizzBuzz
{
    internal class Program
    {
        static void Main(string[] args)
        {
            Console.WriteLine("Welcome to the FizzBuzz word game!\n");

            var input_number = Helper.GetNumberFromUser();
            FizzBuzz(input_number);
            FizzBuzzStringBuilder(input_number);

            Console.WriteLine("Thanks for playing!");
            Console.Read();
        }

        static void FizzBuzz(int number)
        {
            Console.WriteLine(System.Reflection.MethodBase.GetCurrentMethod().Name);
            for (int i = 1; i <= number; i++)
            {
                if (i % 3 == 0 && i % 5 == 0) Console.WriteLine("FizzBuzz");
                else if (i % 3 == 0) Console.WriteLine("Fizz");
                else if (i % 5 == 0) Console.WriteLine("Buzz");
                else Console.WriteLine(i);
            }
            Console.WriteLine("\n");
        }

        static void FizzBuzzStringBuilder(int number)
        {
            Console.WriteLine(System.Reflection.MethodBase.GetCurrentMethod().Name);
            StringBuilder sb = new StringBuilder();
            for (int i = 1; i <= number; i++)
            {
                sb.Clear();
                if (i % 3 == 0) sb.Append("Fizz");
                if (i % 5 == 0) sb.Append("Buzz");
                if(sb.Length == 0) sb.Append(i.ToString());
                Console.WriteLine(sb.ToString());
            }
            Console.WriteLine("\n");
        }
    }
}
