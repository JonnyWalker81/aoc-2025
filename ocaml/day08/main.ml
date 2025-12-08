let read_input () =
  let rec read_lines acc =
    try
      let line = input_line stdin in
      read_lines (line :: acc)
    with End_of_file -> List.rev acc
  in
  read_lines []

let part1 lines =
  ignore lines;
  print_endline "Part 1: TODO"

let part2 lines =
  ignore lines;
  print_endline "Part 2: TODO"

let () =
  let lines = read_input () in
  part1 lines;
  part2 lines
