type rps = Rock | Paper | Scissors

let mov_from_str = function
  | "A" | "X" -> Rock
  | "B" | "Y" -> Paper
  | "C" | "Z" -> Scissors
  | _ -> failwith "bad rps str"

let mov_score = function Rock -> 1 | Paper -> 2 | Scissors -> 3

type outcome = Win | Draw | Loss

let outcome_from_str = function
  | "X" -> Loss
  | "Y" -> Draw
  | "Z" -> Win
  | _ -> failwith "bad rps str"

let outcome_score = function Win -> 6 | Draw -> 3 | Loss -> 0

let get_score opp_mov self_mov =
  let outcome =
    match (opp_mov, self_mov) with
    | a, b when a = b -> Draw
    | Rock, Paper -> Win
    | Rock, _ -> Loss
    | Paper, Scissors -> Win
    | Paper, _ -> Loss
    | Scissors, Rock -> Win
    | Scissors, _ -> Loss
  in

  mov_score self_mov + outcome_score outcome

let part_one input =
  input |> String.split_on_char '\n'
  |> List.map (fun line ->
      line |> String.split_on_char ' ' |> List.map mov_from_str |> function
      | [ opp; self ] -> get_score opp self
      | _ -> failwith "bad input")
  |> List.fold_left Int.add 0

let get_score_from_strategy opp_mov outcome =
  let movs = [ Rock; Paper; Scissors ] in

  let get_winning_mov_against mov =
    let i =
      match List.find_index (fun m -> m = mov) movs with
      | Some i -> i
      | None -> failwith "unreachable"
    in
    List.nth movs (Int.rem (i + 1) 3)
  in

  let get_losing_mov_against mov =
    let i =
      match List.find_index (fun m -> m = mov) movs with
      | Some i -> i
      | None -> failwith "unreachable"
    in
    List.nth movs (Int.rem (3 + i - 1) 3)
  in

  let self_mov =
    match outcome with
    | Draw -> opp_mov
    | Win -> get_winning_mov_against opp_mov
    | Loss -> get_losing_mov_against opp_mov
  in

  mov_score self_mov + outcome_score outcome

let part_two input =
  input |> String.split_on_char '\n'
  |> List.map (fun line ->
      line |> String.split_on_char ' ' |> function
      | [ opp; outcome ] ->
          get_score_from_strategy (mov_from_str opp) (outcome_from_str outcome)
      | _ -> failwith "bad input")
  |> List.fold_left Int.add 0

let () =
  let part = Aoc.Util.parse_args () in
  let input = Aoc.Util.read_input_from_stdin () in

  let result =
    match part with PartOne -> part_one input | PartTwo -> part_two input
  in

  Printf.printf "Result: %d\n" result
