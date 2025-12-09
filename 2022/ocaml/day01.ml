let part_one input =
  Str.split (Str.regexp "\n\n") input
  |> List.map (fun s ->
      Str.split (Str.regexp "\n") s
      |> List.map int_of_string |> List.fold_left Int.add 0)
  |> List.fold_left Int.max 0

let part_two input =
  let module Element = struct
    type t = int

    let compare a b = a - b
  end in
  let module MinHeap = Pqueue.MakeMin (Element) in
  Str.split (Str.regexp "\n\n") input
  |> List.map (fun s ->
      Str.split (Str.regexp "\n") s
      |> List.map int_of_string |> List.fold_left Int.add 0)
  |> List.fold_left (fun hp ncal ->
         MinHeap.add hp ncal;
         let _ = if MinHeap.length hp > 3 then MinHeap.pop_min hp else None in
         hp)
     @@ MinHeap.create ()
  |> MinHeap.fold_unordered Int.add 0

let () =
  let part = Aoc.Util.parse_args () in
  let input = Aoc.Util.read_input_from_stdin () in

  let result =
    match part with PartOne -> part_one input | PartTwo -> part_two input
  in

  Printf.printf "Result: %d\n" result
