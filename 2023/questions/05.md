# [ Day 5: If You Give A Seed A Fertilizer ](https://adventofcode.com/2023/day/5)

--- Day 5: If You Give A Seed A Fertilizer ---You take the boat and find the gardener right where you were told he would be: managing a giant "garden" that looks more to you like a farm."A water source? Island Islandisthe water source!" You point out that Snow Island isn't receiving any water."Oh, we had to stop the water because weran out of sandtofilterit with! Can't make snow with dirty water. Don't worry, I'm sure we'll get more sand soon; we only turned off the water a few days... weeks... oh no." His face sinks into a look of horrified realization."I've been so busy making sure everyone here has food that I completely forgot to check why we stopped getting more sand! There's a ferry leaving soon that is headed over in that direction - it's much faster than your boat. Could you please go check it out?"You barely have time to agree to this request when he brings up another. "While you wait for the ferry, maybe you can help us with ourfood production problem. The latest Island IslandAlmanacjust arrived and we're having trouble making sense of it."The almanac (your puzzle input) lists all of the seeds that need to be planted. It also lists what type of soil to use with each kind of seed, what type of fertilizer to use with each kind of soil, what type of water to use with each kind of fertilizer, and so on. Every type of seed, soil, fertilizer and so on is identified with a number, but numbers are reused by each category - that is, soil123and fertilizer123aren't necessarily related to each other.For example:seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4The almanac starts by listing which seeds need to be planted: seeds79,14,55, and13.The rest of the almanac contains a list ofmapswhich describe how to convert numbers from asource categoryinto numbers in adestination category. That is, the section that starts withseed-to-soil map:describes how to convert aseed number(the source) to asoil number(the destination). This lets the gardener and his team know which soil to use with which seeds, which water to use with which fertilizer, and so on.Rather than list every source number and its corresponding destination number one by one, the maps describe entirerangesof numbers that can be converted. Each line within a map containsthree numbers: thedestination range start, thesource range start, and therange length.Consider again the exampleseed-to-soil map:50 98 2
52 50 48The first line has adestination range startof50, asource range startof98, and arange lengthof2. This line means that the source range starts at98and contains two values:98and99. The destination range is the same length, but it starts at50, so its two values are50and51. With this information, you know that seed number98corresponds to soil number50and that seed number99corresponds to soil number51.The second line means that the source range starts at50and contains48values:50,51, ...,96,97. This corresponds to a destination range starting at52and also containing48values:52,53, ...,98,99. So, seed number53corresponds to soil number55.Any source numbers thataren't mappedcorrespond to thesamedestination number. So, seed number10corresponds to soil number10.So, the entire list of seed numbers and their corresponding soil numbers looks like this:seed  soil
0     0
1     1
...   ...
48    48
49    49
50    52
51    53
...   ...
96    98
97    99
98    50
99    51With this map, you can look up the soil number required for each initial seed number:Seed number79corresponds to soil number81.Seed number14corresponds to soil number14.Seed number55corresponds to soil number57.Seed number13corresponds to soil number13.The gardener and his team want to get started as soon as possible, so they'd like to know the closest location that needs a seed. Using these maps, findthe lowest location number that corresponds to any of the initial seeds. To do this, you'll need to convert each seed number through other categories until you can find its correspondinglocation number. In this example, the corresponding types are:Seed79, soil81, fertilizer81, water81, light74, temperature78, humidity78,location82.Seed14, soil14, fertilizer53, water49, light42, temperature42, humidity43,location43.Seed55, soil57, fertilizer57, water53, light46, temperature82, humidity82,location86.Seed13, soil13, fertilizer52, water41, light34, temperature34, humidity35,location35.So, the lowest location number in this example is35.What is the lowest location number that corresponds to any of the initial seed numbers?