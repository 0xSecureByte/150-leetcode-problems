class Solution:
    def isPalindrome(self, x: int) -> bool:
        bool = True
        if x < 0:
            bool = False
        elif str(x) == str(x)[::-1]:
            bool = True
        else:
            bool = False
        return bool

sol = Solution()
print(sol.isPalindrome(10))