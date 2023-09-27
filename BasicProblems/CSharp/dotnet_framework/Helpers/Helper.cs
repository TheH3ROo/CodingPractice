using System;

namespace Helpers
{
    public static class Helper
    {
        public static int GetNumberFromUser()
        {
            int number = -1;
            while (number < 0)
            {
                Console.Write("Please give me a number:");
                int.TryParse(Console.ReadLine(), out number);
            }
            return number;
        }
    }
}
