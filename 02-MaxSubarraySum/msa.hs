module MaxSequence where

-- Return the greatest subarray sum within the array of integers passed in.
maxSequence :: [Int] -> Int
maxSequence a =
  foldl
    (\a b ->
       if a > sum b
         then a
         else sum b)
    0
    (subArrays a)

subArrays :: [t] -> [[t]]
subArrays [] = []
subArrays [e] = [[e]]
subArrays a = a : (subArrays (tail a)) ++ (subArrays (init a))
