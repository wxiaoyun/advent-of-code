let part_one _ = print_endline "test 2"
let part_two _ = print_endline "test 2"

let () =
  let part = Aoc.Util.parse_args () in
  let input = Aoc.Util.read_input_from_stdin () in
  match part with PartOne -> part_one input | PartTwo -> part_two input
