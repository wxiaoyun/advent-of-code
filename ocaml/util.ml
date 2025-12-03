type arg = PartOne | PartTwo

let parse_args () =
  let args = Sys.argv in
  if Array.length args < 2 then failwith "Missing positional argument \"part\""
  else
    let arg1 = args.(1) in
    let part = int_of_string arg1 in
    match part with
    | 1 -> PartOne
    | 2 -> PartTwo
    | _ -> failwith "Illegal \"part\" argument. Options: 1, 2"

let read_input_from_stdin () =
  let rec loop builder =
    try
      let line = read_line () in
      loop @@ (line :: builder)
    with End_of_file -> builder |> List.rev |> String.concat "\n"
  in
  loop []
