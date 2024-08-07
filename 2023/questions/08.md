# [ Day 8: Haunted Wasteland ](https://adventofcode.com/2023/day/8)

--- Day 8: Haunted Wasteland ---You're still riding a camel across Desert Island when you spot a sandstorm quickly approaching. When you turn to warn the Elf, she disappears before your eyes! To be fair, she had just finished warning you aboutghostsa few minutes ago.One of the camel's pouches is labeled "maps" - sure enough, it's full of documents (your puzzle input) about how to navigate the desert. At least, you're pretty sure that's what they are; one of the documents contains a list of left/right instructions, and the rest of the documents seem to describe some kind ofnetworkof labeled nodes.It seems like you're meant to use theleft/rightinstructions tonavigate the network. Perhaps if you have the camel follow the same instructions, you can escape the haunted wasteland!After examining the maps for a bit, two nodes stick out:AAAandZZZ. You feel likeAAAis where you are now, and you have to follow the left/right instructions until you reachZZZ.This format defines eachnodeof the network individually. For example:RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)Starting withAAA, you need tolook up the next elementbased on the next left/right instruction in your input. In this example, start withAAAand goright(R) by choosing the right element ofAAA,CCC. Then,Lmeans to choose theleftelement ofCCC,ZZZ. By following the left/right instructions, you reachZZZin2steps.Of course, you might not findZZZright away. If you run out of left/right instructions, repeat the whole sequence of instructions as necessary:RLreally meansRLRLRLRLRLRLRLRL...and so on. For example, here is a situation that takes6steps to reachZZZ:LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)Starting atAAA, follow the left/right instructions.How many steps are required to reachZZZ?