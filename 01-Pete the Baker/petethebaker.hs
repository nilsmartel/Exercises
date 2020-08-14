module Baker where

type Ingredient = String

type Amount = Int

type Recipe = [(Ingredient, Amount)]

type Storage = [(Ingredient, Amount)]

cakes :: Recipe -> Storage -> Int
cakes recipe storage = error "todo: cakes"
