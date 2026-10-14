class Solution:

    def encode(self, strs: List[str]) -> str:
        if not strs:
            return ""
        ans = []
        for s in strs:
            for ch in s:
                if ch == '#':
                    ans.append("##")
                else:
                    ans.append(ch)
            ans.append("#n")
        
        return "".join(ans)
                    
    def decode(self, s: str) -> List[str]:
        if not s:
            return []
        ans = [[]]
        i=0
        while i<len(s):
            if s[i] == '#':
                if s[i+1] == '#':
                    ans[-1].append('#')
                elif s[i+1] == "n":
                    ans.append([])
                i+=2
            else:
                ans[-1].append(s[i])
                i+=1
        
        final_strings = []
        for i in ans:
            string = ''.join(i)
            final_strings.append(string)
        final_strings.pop()
        return final_strings