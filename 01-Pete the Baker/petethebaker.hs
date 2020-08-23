module Baker where

import Data.Map ((!), fromList, member)

type Ingredient = String

type Amount = Int

type Recipe = [(Ingredient, Amount)]

type Storage = [(Ingredient, Amount)]

cakes :: Recipe -> Storage -> Int
cakes recipe storage =
  let smap = fromList storage
   in foldl
        (\n (ing, amount) ->
           let times =
                 if member ing smap
                   then div (smap ! ing) amount
                   else 0
            in if n > times
                 then times
                 else n)
        0
        recipe
