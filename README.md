| Category            | Title                                                                                                                                                   | Description                                                                                                            | Solution                                                                                                                                                                                                                             |
| ------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Sorting             | [2160. Minimum Sum of Four Digit Number After Splitting Digits](https://leetcode.com/problems/minimum-sum-of-four-digit-number-after-splitting-digits/) | Return the minimum sum of the split of the digits of a 4 digits integer                                                | Extract digits, sort, 2 smallest digits \* 10 ([code](src/leetcode/pb_2160.rs))                                                                                                                                                      |
| Binary              | [342. Power of Four](https://leetcode.com/problems/power-of-four/)                                                                                      | Return true if power of 4                                                                                              | Power of 4 is a power of 2 and has its odd bits null ([code](src/leetcode/pb_342.rs))                                                                                                                                                |
| Linked list         | [234. Palindrome Linked List](https://leetcode.com/problems/palindrome-linked-list/)                                                                    | Return true if the list is a palindrome                                                                                | Find the middle of the list by traversing the list with a 1 by 1 and a 2 by 2 pointer, reverse the list and compare ([code](src/leetcode/pb_234.rs))                                                                                 |
| Array               | [1929. Concatenation of Array](https://leetcode.com/problems/concatenation-of-array/)                                                                   | Concatenate an array with itself                                                                                       | Straight forward ([code](src/leetcode/pb_1929.rs))                                                                                                                                                                                   |
| Array               | [1920. Build Array from Permutation](https://leetcode.com/problems/build-array-from-permutation/)                                                       | Return an array with values set as the index pointed by the input array values                                         | Straight forward ([code](src/leetcode/pb_1920.rs))                                                                                                                                                                                   |
| Hash maps           | [781. Rabbits in Forest](https://leetcode.com/problems/rabbits-in-forest/)                                                                              | From an array with the amount of rabbits which have the same color as one rabbit, return the minimum number of rabbits | Iterate through the array storing the amount of rabbit with the same color and how many times it occurred as a key value pair in a hash map ([code](src/leetcode/pb_781.rs))                                                         |
| Array               | [1689. Partitioning Into Minimum Number Of Deci-Binary Numbers](https://leetcode.com/problems/partitioning-into-minimum-number-of-deci-binary-numbers/) | Given a string of digits, return the minimum amount of integers of only 0 and 1 digits needed to equal the input       | Just return the largest digit in the string ([code](src/leetcode/pb_1689.rs))                                                                                                                                                        |
| Binary search       | [704. Binary Search](https://leetcode.com/problems/binary-search/)                                                                                      | Execute binary search on a sorted array                                                                                | Right index is initialized as `nums.len()`, if middle is less than target left is set to middle + 1 ([code](src/leetcode/pb_704.rs))                                                                                                 |
| Binary search       | [278. First Bad Version](https://leetcode.com/problems/first-bad-version/)                                                                              | In an array of integers, return the first number returning true calling a getter function                              | Execute binary search until the results switches from false to true                                                                                                                                                                  |
| Binary search       | [35. Search Insert Position](https://leetcode.com/problems/search-insert-position/)                                                                     | Find an integer in a sorted array, if not found return where it would be inserted                                      | Execute binary search and return the left pointer if not found                                                                                                                                                                       |
| Sorting, Pointer    | [977. Squares of a Sorted Array](https://leetcode.com/problems/squares-of-a-sorted-array/)                                                              | From a sorted array return a sorted array where each element is squared                                                | Find the middle and diverge from it, taking the square of each element and pushing the smallest value to the stack ([code](src/leetcode/pb_977.rs))                                                                                  |
| Sorting, Pointer    | [189. Rotate Array](https://leetcode.com/problems/rotate-array/)                                                                                        | Shift array elements by `k` to the right                                                                               | Using recursion: increment the argument `id` until the end of the array is reached, store the value at `id` and the pair index, call recursive function and set the value at pair id to value at id ([code](src/leetcode/pb_189.rs)) |
| Array, Two Pointers | [283. Move Zeros](https://leetcode.com/problems/move-zeroes/)                                                                                           | Given an array of numbers, move all 0's to the end of it while maintaining the relative order of the non-zero elements | One slow pointer on the next element to assign to and one fast pointer iterating through array. If fast pointer element is non zero, swap the 2 values ([code](src/leetcode/pb_283.rs))                                              |
| Array, Two Pointers | [167. Two Sum II - Input Array Is Sorted](https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/)                                              | Given a sorted array, find two numbers such that they add up to a specific `target` number                             | Start low and high pointer at each ends of array, if the sum of the two elements is larger than target, decrement high, if smaller, increment low ([code](src/leetcode/pb_167.rs))                                                   |
