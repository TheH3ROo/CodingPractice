using System;

namespace Helpers
{
    public static class Helper
    {
        public static int GetNumberFromUser()
        {
            int number = 0;
            while (number < 1)
            {
                Console.Write("Please give me a number:");
                int.TryParse(Console.ReadLine(), out number);
            }
            return number;
        }
    }
}
