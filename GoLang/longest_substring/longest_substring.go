package longestsubstring

func Sliding_Window(s string) uint {
	l := 0
	longest := 0
	set := map[byte]bool{}

	for r, _ := range s {
		for {
			if _, exists := set[s[r]]; exists {
				delete(set, s[l])
				l += 1
			} else {
				break
			}
		}

		w := (r - l) + 1

		longest = max(longest, w)
		set[s[r]] = true
	}

	return uint(longest)
}
