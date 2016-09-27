class Solution(object):
    def firstUniqChar(self, s):
        """
        :type s: str
        :rtype: int
        """
        
        if not s:
            return -1
        
        seen = dict()
        deleted = set()    
            
        for i, char in enumerate(s):
            if char in seen:
                deleted.add(char)
                del seen[char]
            else:
                if char not in deleted:
                    seen[char] = [i, char]
        print(seen.values())
        sorted_uniques = sorted(seen.values(), key=lambda x: x[0])
        
        return sorted_uniques[0][0] if sorted_uniques else -1
            
            
        
