let read_input () =
  let rec read_lines acc =
    try
      let line = input_line stdin in
      read_lines (line :: acc)
    with End_of_file -> List.rev acc
  in
  read_lines []

type t = {first: int; second: int}

let parseIDs input =
  Printf.printf "Parsing input: %s\n" input ;
  let ranges = String.split_on_char ',' input in
  let ids =
    List.map
      (fun r ->
        Printf.printf "Parsing range: %s\n" r ;
        match String.split_on_char '-' r with
        | [a; b] ->
            {first= int_of_string a; second= int_of_string b}
        | _ ->
            failwith "Invalid range format" )
      ranges
  in
  ids

(** For each number between ts.first and ts.second find the square stings, strings that have an even number of characters and the first half equals the second half, should return all the found square stings **)
let findSquareStrings ts =
  let rec aux n acc =
    if n > ts.second then acc
    else
      let s = string_of_int n in
      let len = String.length s in
      if len mod 2 = 0 then
        let half = len / 2 in
        let first_half = String.sub s 0 half in
        let second_half = String.sub s half half in
        if first_half = second_half then aux (n + 1) (s :: acc)
        else aux (n + 1) acc
      else aux (n + 1) acc
  in
  aux ts.first []

let part1 lines =
  let ids = parseIDs (List.hd lines) in
  let square_strings = List.flatten (List.map findSquareStrings ids) in
  let sum = List.fold_left ( + ) 0 (List.map int_of_string square_strings) in
  Printf.printf "Sum: %d\n" sum ;
  print_endline "Part 1: TODO"

(* Return k >= 2 if s = block repeated k times; otherwise 0. *)
let repeated_count (s : string) : int =
  let len = String.length s in
  if len <= 1 then 0
  else
    (* Try each possible block length b *)
    let rec try_block b =
      if b > len / 2 then 0
      else if len mod b <> 0 then
        (* length not divisible by b: can't be pure repeats *)
        try_block (b + 1)
      else
        let k = len / b in
        if k < 2 then try_block (b + 1)
        else
          (* candidate block is the first b chars *)
          let block = String.sub s 0 b in
          (* check that for each i = 1..k-1, the block at offset i*b matches *)
          let rec check_block i =
            if i = k then true
            else
              let offset = i * b in
              let rec check_chars j =
                if j = b then true
                else if s.[offset + j] <> block.[j] then false
                else check_chars (j + 1)
              in
              check_chars 0 && check_block (i + 1)
          in
          if check_block 1 then k else try_block (b + 1)
    in
    try_block 1

let repeated_count_int (n : int) : int = repeated_count (string_of_int n)

(* For each range, return the list of (id, k) where k is repeat count >= 2. *)
let analyze_ranges (ranges : t list) : (t * (int * int) list) list =
  List.map
    (fun r ->
      let invalids = ref [] in
      for id = r.first to r.second do
        let k = repeated_count_int id in
        if k >= 2 then invalids := (id, k) :: !invalids
      done ;
      (r, List.rev !invalids) )
    ranges

let part2 lines =
  let ids = parseIDs (List.hd lines) in
  let analysis = analyze_ranges ids in
  List.iter
    (fun (r, invalids) ->
      Printf.printf "Range %d-%d:\n" r.first r.second ;
      List.iter
        (fun (id, k) -> Printf.printf "  %d is repeated %d times\n" id k)
        invalids )
    analysis ;
  let sum_nums_repeated =
    List.fold_left
      (fun acc (_, invalids) ->
        acc + List.fold_left (fun acc2 (id, _) -> acc2 + id) 0 invalids )
      0 analysis
  in
  Printf.printf "Total sum of repeats: %d\n" sum_nums_repeated ;
  print_endline "Part 2: TODO"

let () =
  let lines = read_input () in
  part1 lines ; part2 lines
