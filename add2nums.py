class Solution(object):
    def addTwoNumbers(self, l1, l2):
        l1.reverse()
        l2.reverse()
        sum = int("".join(map(str, l1))) + int("".join(map(str, l2)))
        reversed = list(map(int, str(sum)))
        reversed.reverse()
        return reversed

l1, l2 = [2,4,3], [5,6,4]
result = Solution().addTwoNumbers(l1, l2)
print(result)