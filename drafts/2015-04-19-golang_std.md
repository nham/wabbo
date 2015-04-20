# strings

This is one topological sort of the exported functions in the `strings` package.

## Index

    func Index(s, sep string) int

Returns the starting index of the first occurrence of `sep` in `s` and `-1` otherwise.

### Examples

    fmt.Println(strings.Index("bananas", "nana"))   // 2
    fmt.Println(strings.Index("bananas", "ban"))    // 0
    fmt.Println(strings.Index("bananas", "na"))     // 2
    fmt.Println(strings.Index("bananas", "ghetti")) // -1

### Code

    Comment says Rabin-Karp search. Didn't want to understand the algorithm for now. Maybe later.


## IndexAny

    func IndexAny(s, chars string) int

Returns the index of the first occurrence of a Unicode code point from `chars in `s, and `-1` otherwise.

### Examples

    fmt.Println(strings.IndexAny("777788899", "123")) // -1
    fmt.Println(strings.IndexAny("761538", "123"))    // 2
    fmt.Println(strings.IndexAny("", "abc"))          // -1
    fmt.Println(strings.IndexAny("foo", ""))          // -1
    fmt.Println(strings.IndexAny("", ""))             // -1

### Code

Pseudocode:

    def IndexAny(s, chars string) int {
        if length of chars > 0 {
            i := 0
            for each c in s {
                for each m in chars {
                    if c == m {
                        return i
                    }
                }
                i += 1
            }
        }
        return -1
    }


## IndexRune

    func IndexRune(s string, r rune) int

Returns the index of the first occurrence of `r` in `s`, or `-1` otherwise.

### Examples

    fmt.Println(strings.IndexRune("777788899", '1')) // -1
    fmt.Println(strings.IndexRune("711511", '5'))    // 3

### Code

Pseudocode:

    def IndexRune(s string, r rune) int {
        i := 0
        for each c in s {
            if c == r {
                return i
            }
            i += 1
        }

        return -1
    }




## Contains

    func Contains(s, substr string) bool

Returns `true` if `substr` is a substring of `s`, and false otherwise.

### Examples

    fmt.Println(strings.Contains("bananas", "nana")) // true
    fmt.Println(strings.Contains("bananas", "nona")) // false
    fmt.Println(strings.Contains("", "foo"))         // false
    fmt.Println(strings.Contains("foo", ""))         // true
    fmt.Println(strings.Contains("", ""))            // true


### Code

    func Contains(s, substr string) bool {
        return Index(s, substr) >= 0
    }


## ContainsAny

    func ContainsAny(s, chars string) bool

Returns `true` if there is any Unicode code point in `chars` that is also in `s`, and `false` otherwise.

### Examples

	fmt.Println(strings.ContainsAny("batman", "abc"))  // true
	fmt.Println(strings.ContainsAny("dzzffzz", "abc")) // false
	fmt.Println(strings.ContainsAny("", "foo"))        // false
	fmt.Println(strings.ContainsAny("foo", ""))        // false
	fmt.Println(strings.ContainsAny("", ""))           // false


### Code

    func ContainsAny(s, chars string) bool {
        return IndexAny(s, chars) >= 0
    }


## ContainsRune

    func ContainsRune(s string, r rune) bool

Returns `true` if `r` is in `s` and `false` otherwise.

### Examples

	fmt.Println(strings.ContainsRune("bamboozle", 'z')) // true
	fmt.Println(strings.ContainsRune("yellow", 'a'))    // false


### Code

    func ContainsRune(s string, r rune) bool {
        return IndexRune(s, r) > = 0
    }





## Count

    func Count(s, sep string) int

Returns the number of times `sep` occurs as a substring of `s`. 

### Examples

	fmt.Println(strings.Count("bananas", "na")) // 2
	fmt.Println(strings.Count("five", ""))      // 5
	fmt.Println(strings.Count("", ""))          // 1
	fmt.Println(strings.Count("", "a"))         // 0


### Code

    Insert some string matching algorithm + counting algorithm here.




## Replace

    func Replace(s, old, new string, n int) string

Returns a copy of `s` with the first `n` non-overlapping instances of `old` replaced by `new`. When `n < 0`, it is unlimited replacements. If `old` is empty, `new` gets inserted before and after every rune in `s`.

### Examples

	fmt.Println(strings.Replace("a22b22", "2", "33", 1))  // a332b22
	fmt.Println(strings.Replace("a22b22", "2", "33", 2))  // a3333b22
	fmt.Println(strings.Replace("a22b22", "2", "33", -1)) // a3333b3333
    fmt.Println(strings.Replace("abcd", "", "#", -1))     // #a#b#c#d#


### Code

If `old == new`, `n == 0`, or there's no instances of `old` in `s`, just return s.

If `n == -1` or `m < n`, we will do `m` replacements. Otherwise we do `n` replacements.

Pseudocode

    def Replace(s, old, new string, n int) string {
        if old == new || n == 0 {
            return s // avoid allocation
        }

        m := Count(s, old); // Compute number of replacements.

        if m == 0 {
            return s // avoid allocation
        }

        var k
        if n < 0 || m < n {
            k = m
        } else {
            k = n
        }

        diff := k * (len(new) - len(old))
        t := make([]byte, len(s) + diff)
        (...)
    }
