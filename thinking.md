# Introduction
Mormon Bridge is trick-taking game played in rounds (called hands). The game is played in 13 hands, starting with 1 card until 7 cards in the hand, and then played back down to 1 card in the hand. Each hand has a trump that is revealed by flipping over the top card from the deck after the hand has been dealt. Before trick taking begins, players make a simultaneous bid from 0 - n, where n is the number of cards in the hand. The bid is recorded. For each trick, the all players are bound to follow the lead suit. If the player has no cards in the lead suit, they are permitted to play any card in their hand. The highest card of the trump suit will take the trick, or if no trumps are played, the highest card in the lead suit wins. Aces are considered the highest card in the suit. After all tricks in the hand have been played, the points for the trick are tallied up. A player wins 10 points for making the bid, or loses 10 points for missing their bid. Additional points are scored or lost based on the formula |bid - tricks taken|. 

# Technical Specs
## Game
A game has 13 hands  
A game has a score for each player  
A game has a dealer order  
A game has 2-7 players  
## Player
A player has a name  
A player has a score  
A player has a hand for the current hand being played  
A player plays cards from their hand  
A player looks at their hand  
A player knows their bid  
A player knows when they play  
A player is dealt cards  
## Hand
A hand has 1-7 tricks  
A hand has a dealer  
A hand has a bid for each player  
A hand has a trump suit  
At the start of a hand, players are dealt cards  
At the start of a hand, after dealing players in, a trump card is shown  
## Trick
A trick has a card from each player  
A trick has a lead card  
A trick determines the winner of the trick  
A trick has a lead player  
## Deck
A deck contains 52 cards, one of each rank-suit pairing  
A deck deals cards to players  
## Card
A card has a rank  
A card has a suit  
A card has a value  
A card is compared to other cards on the basis of rank and suit  
A card's rank ordering is based on either aces high or low  
A card's suit ordering is based on the lead card and the trump suit  
