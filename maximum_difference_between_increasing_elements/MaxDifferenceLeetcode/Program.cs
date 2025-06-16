using System;

public class Solution
{

    public static void Main()
    {
        int[] nums1 = [7, 1, 5, 4];     // Output: 4    (1 - 5)
        int[] nums2 = [9, 4, 3, 2];     // Output: -1   (no i < j)
        int[] nums3 = [1, 5, 2, 10];    // Output: 9    (1 - 10)
        Console.WriteLine("Hello World");

        var solution = new Solution();
        Console.WriteLine(solution.MaximumDifference(nums1));
        Console.WriteLine(solution.MaximumDifference(nums2));
        Console.WriteLine(solution.MaximumDifference(nums3));
    }
    public int MaximumDifference(int[] nums)
    {
        int difference = 0;
        int maxDifference = -1;

        foreach (int i in nums)
        {
            foreach (int j in nums)
            {
                if (j < i)
                {
                    continue;
                }

                if (Array.IndexOf(nums, i) > Array.IndexOf(nums, j))
                {
                    continue;
                }
                difference = j - i;

                if (difference > maxDifference)
                {
                    maxDifference = difference;
                }
            }
        }

        if (maxDifference == 0)
        {
            return -1;
        }
        return maxDifference;
    }
}