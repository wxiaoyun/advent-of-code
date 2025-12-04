let loop_3x3 fn init =
  let loop3 = List.init 3 Fun.id in
  let accumulator i acc j = fn acc j i in
  loop3
  |> List.fold_left
       (fun accum i -> List.fold_left (accumulator i) accum loop3)
       init

let is_invalid_roll mat r c =
  let nrow = Array.length mat in
  let ncol = Array.length mat.(0) in
  Int.min r c < 0 || r >= nrow || c >= ncol || mat.(r).(c) != '@'

let is_accessible mat r c =
  if mat.(r).(c) != '@' then false
  else
    let accumulator roll_cnt i j =
      let rr, cc = (r + i - 1, c + j - 1) in

      if (r, c) = (rr, cc) then roll_cnt
      else if is_invalid_roll mat rr cc then roll_cnt
      else roll_cnt + 1
    in

    loop_3x3 accumulator 0 < 4

let part_one input =
  let mat =
    input |> String.split_on_char '\n'
    |> List.map (fun l -> l |> String.to_seq |> Array.of_seq)
    |> Array.of_list
  in

  let nrow = Array.length mat in
  let ncol = Array.length mat.(0) in
  let loop_rows = List.init nrow Fun.id in
  let loop_cols = List.init ncol Fun.id in

  loop_rows
  |> List.fold_left
       (fun accessible_cnt row ->
         loop_cols
         |> List.fold_left
              (fun accessible_cnt col ->
                accessible_cnt + if is_accessible mat row col then 1 else 0)
              accessible_cnt)
       0

let remove_and_notify_neighbor mat work_queue r c =
  Array.set mat.(r) c '.';

  let accumulator () i j =
    let rr, cc = (r + i - 1, c + j - 1) in
    if is_invalid_roll mat rr cc then () else Queue.add (rr, cc) work_queue
  in

  loop_3x3 accumulator ()

let part_two input =
  let mat =
    input |> String.split_on_char '\n'
    |> List.map (fun l -> l |> String.to_seq |> Array.of_seq)
    |> Array.of_list
  in

  let nrow = Array.length mat in
  let ncol = Array.length mat.(0) in
  let loop_rows = List.init nrow Fun.id in
  let loop_cols = List.init ncol Fun.id in

  let remove_list =
    loop_rows
    |> List.fold_left
         (fun remove_list row ->
           loop_cols
           |> List.fold_left
                (fun remove_list col ->
                  if is_accessible mat row col then (row, col) :: remove_list
                  else remove_list)
                remove_list)
         []
  in

  let work_queue = remove_list |> List.to_seq |> Queue.of_seq in

  let rec loop remove_cnt =
    match Queue.take_opt work_queue with
    | None -> remove_cnt
    | Some (row, col) ->
        if not (is_accessible mat row col) then loop remove_cnt
        else (
          remove_and_notify_neighbor mat work_queue row col;
          loop (remove_cnt + 1))
  in

  loop 0

let () =
  let part = Aoc.Util.parse_args () in
  let input = Aoc.Util.read_input_from_stdin () in

  let result =
    match part with PartOne -> part_one input | PartTwo -> part_two input
  in

  Printf.printf "Result: %d\n" result
