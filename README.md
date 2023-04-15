# chess
a cli chess implementation written in rust

showing possible moves is disabled for windows since im too lazy to implement it right now

```
Usage: chess [OPTION]...

--flip will flip the board each move

--numbers will show 1 2 3 4 5 6 7 8 on the bottom instead of a b c d e f g h

--file CSV will load a board from a csv file

--black will make you play as black

--ip IP will connect to a server at IP:port
```

8x8.csv is an example which is just the default board

26x26.csv is just a silly board example with a 26x26 grid

typing EXIT returns all the moves and exits

TODO: implement bots

yellow means the move is possible(does not account for check)

![image](https://user-images.githubusercontent.com/55570525/232214521-d48c291b-6c96-4500-8800-fcf34d1756e8.png)
