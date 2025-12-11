module CharOrd = struct
  type t = char

  let compare = Char.compare
end

module CharSet = Set.Make (CharOrd)

let priority_of_chr chr =
  match chr with
  | 'a' .. 'z' -> Char.code chr - Char.code 'a' + 1
  | 'A' .. 'Z' -> Char.code chr - Char.code 'A' + 27
  | _ -> failwith "priority_of_chr: bad input"

let part_one input =
  input |> String.split_on_char '\n'
  |> List.map (fun l ->
      let len = String.length l in
      let lk_set = l |> String.to_seq |> Seq.take (len / 2) |> CharSet.of_seq in

      l |> String.to_seq
      |> Seq.drop (len / 2)
      |> Seq.fold_left
           (fun dup_opt chr ->
             match dup_opt with
             | Some _ -> dup_opt
             | None -> if CharSet.mem chr lk_set then Some chr else None)
           None)
  |> List.map Option.get |> List.map priority_of_chr |> List.fold_left Int.add 0

let part_two input =
  let rec group_three = function
    | [] -> []
    | a :: b :: c :: rest -> (a, b, c) :: group_three rest
    | _ -> failwith "group_three: bad input"
  in

  let chr_to_int chr = Int64.shift_left 1L @@ (priority_of_chr chr - 1) in
  let int_to_chr i =
    if i < 1L then failwith "int_to_chr: bad input" else ();
    let chrs = "abcdefghijklmnopqrstuvwxyz" |> String.to_seq in
    let chrs_upper = chrs |> Seq.map Char.uppercase_ascii in
    let all_chars = Seq.append chrs chrs_upper |> List.of_seq in

    let rec loop i chrs =
      match (i, chrs) with
      | 1L, chr :: _ -> chr
      | _, _ :: rest -> loop (Int64.shift_right i 1) rest
      | _ -> failwith "loop: bad input"
    in

    loop i all_chars
  in

  input |> String.split_on_char '\n' |> group_three
  |> List.map (fun (a, b, c) ->
      let union a =
        a |> String.to_seq |> Seq.map chr_to_int |> Seq.fold_left Int64.logor 0L
      in

      [ a; b; c ] |> List.map union
      |> List.fold_left Int64.logand Int64.max_int
      |> int_to_chr)
  |> List.map priority_of_chr |> List.fold_left Int.add 0

let () =
  let part = Aoc.Util.parse_args () in
  let input = Aoc.Util.read_input_from_stdin () in

  let result =
    match part with PartOne -> part_one input | PartTwo -> part_two input
  in

  Printf.printf "Result: %d\n" result
