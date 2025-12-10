type rps = Rock | Paper | Scissors

let rps_from_str = function
  | "A" | "X" -> Rock
  | "B" | "Y" -> Paper
  | "C" | "Z" -> Scissors
  | _ -> failwith "bad rps str"

let get_score opp_mov self_mov =
  let mov_score =
    match self_mov with Rock -> 1 | Paper -> 2 | Scissors -> 3
  in

  let win = 6 in
  let draw = 3 in
  let loss = 0 in
  let outcome_score =
    match (opp_mov, self_mov) with
    | a, b when a = b -> draw
    | Rock, Paper -> win
    | Rock, _ -> loss
    | Paper, Scissors -> win
    | Paper, _ -> loss
    | Scissors, Rock -> win
    | Scissors, _ -> loss
  in

  mov_score + outcome_score

let part_one input =
  input |> String.split_on_char '\n'
  |> List.map (fun line ->
      line |> String.split_on_char ' ' |> List.map rps_from_str |> function
      | [ opp; self ] -> get_score opp self
      | _ -> failwith "bad input")
  |> List.fold_left Int.add 0

let part_two _ = failwith "unimplemented"

let () =
  let part = Aoc.Util.parse_args () in
  let input = Aoc.Util.read_input_from_stdin () in

  let result =
    match part with PartOne -> part_one input | PartTwo -> part_two input
  in

  Printf.printf "Result: %d\n" result
