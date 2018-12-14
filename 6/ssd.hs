
lsq :: [Int] -> Int
lsq [] = 0
lsq (x:xs) = x*x+lsq xs

suml :: [Int] -> Int
suml [] = 0
suml (x:xs) = x + sum xs

f :: Int -> Int
f n = (suml [1..n])*(suml [1..n])-(lsq [1..n])

main = do
 input <- getLine
 print (f (read input))
