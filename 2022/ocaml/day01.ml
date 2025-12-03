let part_one _ = failwith "part one unimplemented"
let part_two _ = failwith "part two unimplemented"

let () =
  let part = Aoc.Util.parse_args () in
  let input = Aoc.Util.read_input_from_stdin () in
  match part with PartOne -> part_one input | PartTwo -> part_two input
