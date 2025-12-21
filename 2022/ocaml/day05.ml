let parse_input input =
  let fst, snd =
    input |> Str.split (Str.regexp "\n\n") |> function
    | [ fst; snd ] -> (fst, snd)
    | _ -> failwith "bad input"
  in

  let idx, rest =
    fst |> String.split_on_char '\n' |> List.rev |> function
    | idx :: rest ->
        (idx, rest |> List.map (fun l -> l |> String.to_seq |> Array.of_seq))
    | _ -> failwith "bad input"
  in

  let stacks =
    idx |> String.to_seqi
    |> Seq.fold_left
         (fun stacks (i, chr) ->
           if not @@ Char.Ascii.is_digit chr then stacks
           else
             let stack =
               rest
               |> List.fold_left
                    (fun stack arr ->
                      if Char.Ascii.is_alphanum arr.(i) then arr.(i) :: stack
                      else stack)
                    []
             in
             stack :: stacks)
         []
    |> List.rev |> Array.of_list
  in

  let mov_reg = Str.regexp {|move \([0-9]+\) from \([0-9]+\) to \([0-9]+\)|} in

  let moves =
    snd |> String.split_on_char '\n'
    |> List.fold_left
         (fun movs l ->
           let _ = Str.search_forward mov_reg l 0 in

           let mov_tuple =
             List.init 3 Fun.id
             |> List.map (fun i -> Str.matched_group (i + 1) l)
             |> List.map int_of_string
             |> function
             | [ a; b; c ] -> (a, b, c)
             | _ -> failwith "bad input"
           in

           mov_tuple :: movs)
         []
    |> List.rev
  in

  (stacks, moves)

let part_one input =
  let stacks, moves = parse_input input in

  let stacks =
    moves
    |> List.fold_left
         (fun stacks (n, src, dst) ->
           let src, dst = (src - 1, dst - 1) in
           let loop = List.init n Fun.id in

           List.iter
             (fun _ ->
               let src_stack, dst_stack = (stacks.(src), stacks.(dst)) in

               let src_stack, dst_stack =
                 match src_stack with
                 | a :: rest -> (rest, a :: dst_stack)
                 | _ -> failwith "src stack is empty"
               in

               Array.set stacks src src_stack;
               Array.set stacks dst dst_stack)
             loop;

           stacks)
         stacks
  in

  stacks
  |> Array.fold_left
       (fun ls stack ->
         match stack with a :: _ -> a :: ls | _ -> failwith "empty stack")
       []
  |> List.rev |> List.to_seq |> String.of_seq

let part_two input = failwith input

let () =
  let part = Aoc.Util.parse_args () in
  let input = Aoc.Util.read_input_from_stdin () in

  let result =
    match part with PartOne -> part_one input | PartTwo -> part_two input
  in

  Printf.printf "Result: %s\n" result
