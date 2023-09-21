using Helpers;
using System;
using System.Collections.Generic;

namespace CoinCounter
{
    internal class Program
    {
        static void Main(string[] args)
        {
            int number = Helper.GetNumberFromUser();
            var pairs = CoinValueAmountPairs(number);
            var pairsCombined = CoinValueAmountPairsCombined(number);

            WriteDictionaryToConsole(pairs);
            WriteDictionaryToConsole(pairsCombined);

            Console.ReadLine();
        }

        static void WriteDictionaryToConsole(Dictionary<int, int> pairs)
        {
            foreach (var kvp in pairs)
            {
                Console.WriteLine($"Key: {kvp.Key}, Value: {kvp.Value}");
            }
            Console.WriteLine("\n");
        }

        static Dictionary<int, int> CoinValueAmountPairs(int value)
        {
            Dictionary<int, int> keyValuePairs = new Dictionary<int, int>();
            int highestValueCoin;
            while (value > 0)
            {
                highestValueCoin = GetHighestValueCoin(value);
                if (keyValuePairs.ContainsKey(highestValueCoin))
                {
                    keyValuePairs[highestValueCoin]++;
                }
                else
                {
                    keyValuePairs.Add(highestValueCoin, 1);
                }
                value -= highestValueCoin;
            }
            return keyValuePairs;
        }
        static int GetHighestValueCoin(int value)
        {
            if (value - 200 >= 0) return 200;
            if (value - 100 >= 0) return 100;
            if (value - 50 >= 0) return 50;
            if (value - 20 >= 0) return 20;
            if (value - 10 >= 0) return 10;
            if (value - 5 >= 0) return 5;
            if (value - 2 >= 0) return 2;
            if (value - 1 >= 0) return 1;
            return 0;
        }

        static int GetHighestValueCoinSimplified(int value)
        {
            int[] coins = { 200, 100, 50, 20, 10, 5, 2, 1 };

            foreach (int coin in coins)
            {
                if (value >= coin)
                    return coin;
            }

            return 0;
        }

        static Dictionary<int, int> CoinValueAmountPairsCombined(int value)
        {
            Dictionary<int, int> keyValuePairs = new Dictionary<int, int>();
            int[] coins = { 200, 100, 50, 20, 10, 5, 2, 1 };

            foreach (int coin in coins)
            {
                if (value >= coin)
                {
                    int coinCount = value / coin;
                    keyValuePairs.Add(coin, coinCount);
                    value %= coin;
                }
            }

            return keyValuePairs;
        }
    }
}
