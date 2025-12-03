let part_one input =
  let largest_two_digit_number digit (largest_num, largest_right_digit) =
    let digit = int_of_char digit - int_of_char '0' in
    if largest_right_digit < 0 then (digit, largest_num)
    else
      let largest_num =
        Int.max largest_num @@ ((digit * 10) + largest_right_digit)
      in
      let largest_right_digit = Int.max largest_right_digit digit in
      (largest_right_digit, largest_num)
  in

  input |> String.split_on_char '\n'
  |> List.map (fun line ->
         String.fold_right largest_two_digit_number line (-1, 0))
  |> List.map (fun (_, largest_num) -> largest_num)
  |> List.fold_left Int.add 0

let part_two input =
  let largest_n_digit_number n digits =
    if n <= 0 then failwith "Illegal argument, n must be positive" else ();

    (* Start of the list: Most significant digit *)
    let rec include_digit digits_builder digit =
      match digits_builder with
      | [] -> []
      | msd :: rest ->
          (* if current most significant digit is larger, we cannot build a larger number *)
          if digit < msd then digits_builder
            (* Otherwise, we update the msd and recurse *)
          else digit :: include_digit rest msd
    in

    let build = List.fold_left (fun acc digit -> (acc * 10) + digit) 0 in

    digits |> String.to_seq
    |> Seq.map (fun d -> int_of_char d - int_of_char '0')
    |> List.of_seq |> List.rev
    |> List.fold_left
         (fun (largest_num, builder, builder_len) digit ->
           let new_builder, new_len =
             if builder_len < n then (digit :: builder, builder_len + 1)
             else (include_digit builder digit, builder_len)
           in
           let new_num = build new_builder in
           (Int.max largest_num new_num, new_builder, new_len))
         (-1, [], 0)
    |> fun (largest_num, _, _) -> largest_num
  in

  input |> String.split_on_char '\n'
  |> List.map @@ largest_n_digit_number 12
  |> List.fold_left Int.add 0

let () =
  let part = Aoc.Util.parse_args () in
  let input = Aoc.Util.read_input_from_stdin () in

  let result =
    match part with PartOne -> part_one input | PartTwo -> part_two input
  in

  Printf.printf "Result: %d\n" result
