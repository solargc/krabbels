# Design:

Separation of concerns to be able to later develop a GUI.

UI (terminal)-> collects input
Game -> actions logic
Board -> validate moves 

# State management:

I would like to implement a history of the moves, and being able to save state of the game.

# Done:

- Project tree with separated game logic
- Data structs: player, bag, board, rack, etc.
- Basic gaming loop
- Validation of the move

# To do:

- Automate testing
- State management / data serialization
- Checking dictionnary
- Scoring
