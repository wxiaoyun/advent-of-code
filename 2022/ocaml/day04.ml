let part_one input =
  input |> String.split_on_char '\n'
  |> List.map @@ String.split_on_char ','
  |> List.map (function
    | [] | _ :: [] -> failwith "bad input"
    | lrange :: rrange :: _ ->
        let split_range range =
          range |> String.split_on_char '-' |> List.map int_of_string
        in

        let lrange_num = split_range lrange in
        let rrange_num = split_range rrange in

        let ll, lr = (List.nth lrange_num 0, List.nth lrange_num 1) in
        let rl, rr = (List.nth rrange_num 0, List.nth rrange_num 1) in

        let left_contains = ll <= rl && rr <= lr in
        let right_contains = rl <= ll && lr <= rr in

        if left_contains || right_contains then 1 else 0)
  |> List.fold_left Int.add 0

let part_two input =
  input |> String.split_on_char '\n'
  |> List.map @@ String.split_on_char ','
  |> List.map (function
    | [] | _ :: [] -> failwith "bad input"
    | lrange :: rrange :: _ ->
        let split_range range =
          range |> String.split_on_char '-' |> List.map int_of_string
        in

        let lrange_num = split_range lrange in
        let rrange_num = split_range rrange in

        let ll, lr = (List.nth lrange_num 0, List.nth lrange_num 1) in
        let rl, rr = (List.nth rrange_num 0, List.nth rrange_num 1) in

        let left_no_overlap = lr < rl in
        let right_no_overlap = rr < ll in

        if left_no_overlap || right_no_overlap then 0 else 1)
  |> List.fold_left Int.add 0

let () =
  let part = Aoc.Util.parse_args () in
  let input = Aoc.Util.read_input_from_stdin () in

  let result =
    match part with PartOne -> part_one input | PartTwo -> part_two input
  in

  Printf.printf "Result: %d\n" result
