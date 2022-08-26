| Category    | Title                                                                                                                                                   | Description                                                                                                            | Solution                                                                                                                                                                     |
| ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Sorting     | [2160. Minimum Sum of Four Digit Number After Splitting Digits](https://leetcode.com/problems/minimum-sum-of-four-digit-number-after-splitting-digits/) | Return the minimum sum of the split of the digits of a 4 digits integer                                                | Extract digits, sort, 2 smallest digits \* 10 ([code](src/leetcode/pb_2160.rs))                                                                                              |
| Binary      | [342. Power of Four](https://leetcode.com/problems/power-of-four/)                                                                                      | Return true if power of 4                                                                                              | Power of 4 is a power of 2 and has its odd bits null ([code](src/leetcode/pb_342.rs))                                                                                        |
| Linked list | [234. Palindrome Linked List](https://leetcode.com/problems/palindrome-linked-list/)                                                                    | Return true if the list is a palindrome                                                                                | Find the middle of the list by traversing the list with a 1 by 1 and a 2 by 2 pointer, reverse the list and compare ([code](src/leetcode/pb_234.rs))                         |
| Array       | [1929. Concatenation of Array](https://leetcode.com/problems/concatenation-of-array/)                                                                   | Concatenate an array with itself                                                                                       | Straight forward ([code](src/leetcode/pb_1929.rs))                                                                                                                           |
| Array       | [1920. Build Array from Permutation](https://leetcode.com/problems/build-array-from-permutation/)                                                       | Return an array with values set as the index pointed by the input array values                                         | Straight forward ([code](src/leetcode/pb_1920.rs))                                                                                                                           |
| Hash maps   | [781. Rabbits in Forest](https://leetcode.com/problems/rabbits-in-forest/)                                                                              | From an array with the amount of rabbits which have the same color as one rabbit, return the minimum number of rabbits | Iterate through the array storing the amount of rabbit with the same color and how many times it occurred as a key value pair in a hash map ([code](src/leetcode/pb_781.rs)) |
| Array       | [1689. Partitioning Into Minimum Number Of Deci-Binary Numbers](https://leetcode.com/problems/partitioning-into-minimum-number-of-deci-binary-numbers/) | Given a string of digits, return the minimum amount of integers of only 0 and 1 digits needed to equal the input       | Just return the largest digit in the string ([code](src/leetcode/pb_1689.rs))                                                                                                |
