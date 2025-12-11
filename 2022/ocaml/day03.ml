module CharOrd = struct
  type t = char

  let compare a b = Char.compare a b
end

module CharSet = Set.Make (CharOrd)

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
  |> List.map Option.get
  |> List.map (fun chr ->
      match chr with
      | 'A' .. 'Z' -> Char.code chr - Char.code 'A' + 27
      | 'a' .. 'z' -> Char.code chr - Char.code 'a' + 1
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
