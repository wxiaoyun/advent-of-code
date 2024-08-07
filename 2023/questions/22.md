# [ Day 22: Sand Slabs ](https://adventofcode.com/2023/day/22)

--- Day 22: Sand Slabs ---Enough sand has fallen; it can finally filter water for Snow Island.Well,almost.The sand has been falling as large compactedbricksof sand, piling up to form an impressive stack here near the edge of Island Island. In order to make use of the sand to filter water, some of the bricks will need to be broken apart - nay,disintegrated- back into freely flowing sand.The stack is tall enough that you'll have to be careful about choosing which bricks to disintegrate; if you disintegrate the wrong brick, large portions of the stack could topple, which sounds pretty dangerous.The Elves responsible for water filtering operations took asnapshot of the bricks while they were still falling(your puzzle input) which should let you work out which bricks are safe to disintegrate. For example:1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9Each line of text in the snapshot represents the position of a single brick at the time the snapshot was taken. The position is given as twox,y,zcoordinates - one for each end of the brick - separated by a tilde (~). Each brick is made up of a single straight line of cubes, and the Elves were even careful to choose a time for the snapshot that had all of the free-falling bricks atinteger positions above the ground, so the whole snapshot is aligned to a three-dimensional cube grid.A line like2,2,2~2,2,2means that both ends of the brick are at the same coordinate - in other words, that the brick is a single cube.Lines like0,0,10~1,0,10or0,0,10~0,1,10both represent bricks that aretwo cubesin volume, both oriented horizontally. The first brick extends in thexdirection, while the second brick extends in theydirection.A line like0,0,1~0,0,10represents aten-cube brickwhich is orientedvertically. One end of the brick is the cube located at0,0,1, while the other end of the brick is located directly above it at0,0,10.The ground is atz=0and is perfectly flat; the lowestzvalue a brick can have is therefore1. So,5,5,1~5,6,1and0,2,1~0,2,5are both resting on the ground, but3,3,2~3,3,3was above the ground at the time of the snapshot.Because the snapshot was taken while the bricks were still falling, some bricks willstill be in the air; you'll need to start by figuring out where they will end up. Bricks are magically stabilized, so theynever rotate, even in weird situations like where a long horizontal brick is only supported on one end. Two bricks cannot occupy the same position, so a falling brick will come to rest upon the first other brick it encounters.Here is the same example again, this time with each brick given a letter so it can be marked in diagrams:1,0,1~1,2,1   <- A
0,0,2~2,0,2   <- B
0,2,3~2,2,3   <- C
0,0,4~0,2,4   <- D
2,0,5~2,2,5   <- E
0,1,6~2,1,6   <- F
1,1,8~1,1,9   <- GAt the time of the snapshot, from the side so thexaxis goes left to right, these bricks are arranged like this:x
012
.G. 9
.G. 8
... 7
FFF 6
..E 5 z
D.. 4
CCC 3
BBB 2
.A. 1
--- 0Rotating the perspective 90 degrees so theyaxis now goes left to right, the same bricks are arranged like this:y
012
.G. 9
.G. 8
... 7
.F. 6
EEE 5 z
DDD 4
..C 3
B.. 2
AAA 1
--- 0Once all of the bricks fall downward as far as they can go, the stack looks like this, where?means bricks are hidden behind other bricks at that location:x
012
.G. 6
.G. 5
FFF 4
D.E 3 z
??? 2
.A. 1
--- 0Again from the side:y
012
.G. 6
.G. 5
.F. 4
??? 3 z
B.C 2
AAA 1
--- 0Now that all of the bricks have settled, it becomes easier to tell which bricks are supporting which other bricks:BrickAis the only brick supporting bricksBandC.BrickBis one of two bricks supporting brickDand brickE.BrickCis the other brick supporting brickDand brickE.BrickDsupports brickF.BrickEalso supports brickF.BrickFsupports brickG.BrickGisn't supporting any bricks.Your first task is to figure outwhich bricks are safe to disintegrate. A brick can be safely disintegrated if, after removing it,no other brickswould fall further directly downward. Don't actually disintegrate any bricks - just determine what would happen if, for each brick, only that brick were disintegrated. Bricks can be disintegrated even if they're completely surrounded by other bricks; you can squeeze between bricks if you need to.In this example, the bricks can be disintegrated as follows:BrickAcannot be disintegrated safely; if it were disintegrated, bricksBandCwould both fall.BrickBcanbe disintegrated; the bricks above it (DandE) would still be supported by brickC.BrickCcanbe disintegrated; the bricks above it (DandE) would still be supported by brickB.BrickDcanbe disintegrated; the brick above it (F) would still be supported by brickE.BrickEcanbe disintegrated; the brick above it (F) would still be supported by brickD.BrickFcannot be disintegrated; the brick above it (G) would fall.BrickGcanbe disintegrated; it does not support any other bricks.So, in this example,5bricks can be safely disintegrated.Figure how the blocks will settle based on the snapshot. Once they've settled, consider disintegrating a single brick;how many bricks could be safely chosen as the one to get disintegrated?