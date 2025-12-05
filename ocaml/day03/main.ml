let read_input () =
  let rec read_lines acc =
    try
      let line = input_line stdin in
      read_lines (line :: acc)
    with End_of_file -> List.rev acc
  in
  read_lines []

(* Convert string of digits to int list *)
let digits str =
  str |> String.to_seq
  |> Seq.map (fun c -> Char.code c - Char.code '0')
  |> List.of_seq

(* Compute suffix maximums: for each position, max digit from there to end *)
let suffix_maxes digits =
  List.fold_right
    (fun d acc ->
      match acc with [] -> [d] | max_so_far :: _ -> max d max_so_far :: acc )
    digits []

(* Drop last element from list *)
let drop_last lst =
  match List.rev lst with _ :: rest -> List.rev rest | [] -> []

(* Find max 2-digit joltage from a bank *)
let max_joltage bank =
  let d = digits bank in
  let suffixes = suffix_maxes d in
  match suffixes with
  | _ :: after_maxes ->
      (* d has n elements, after_maxes has n-1; drop last digit *)
      List.combine (drop_last d) after_maxes
      |> List.map (fun (first, second) -> (first * 10) + second)
      |> List.fold_left max 0
  | [] ->
      0

let part1 lines =
  lines |> List.map max_joltage |> List.fold_left ( + ) 0
  |> Printf.printf "Part 1: %d\n"

(* Find (index, value) of max element in range [start_idx, end_idx] *)
let max_in_range lst start_idx end_idx =
  let rec aux idx best_idx best_val = function
    | [] ->
        (best_idx, best_val)
    | _ :: _ when idx > end_idx ->
        (best_idx, best_val)
    | x :: xs when idx >= start_idx && x > best_val ->
        aux (idx + 1) idx x xs
    | _ :: xs ->
        aux (idx + 1) best_idx best_val xs
  in
  aux 0 start_idx (List.nth lst start_idx) lst

(* Greedily pick k digits to form maximum number *)
let pick_max_k digits k =
  let n = List.length digits in
  let rec pick start_pos remaining acc =
    if remaining = 0 then List.rev acc
    else
      let end_pos = n - remaining in
      let best_idx, best_val = max_in_range digits start_pos end_pos in
      pick (best_idx + 1) (remaining - 1) (best_val :: acc)
  in
  pick 0 k []

(* Convert digit list to Int64 (12 digits exceeds int32) *)
let digits_to_int64 digits =
  List.fold_left
    (fun acc d -> Int64.add (Int64.mul acc 10L) (Int64.of_int d))
    0L digits

(* Max joltage from picking 12 batteries *)
let max_joltage_12 bank =
  bank |> digits |> fun d -> pick_max_k d 12 |> digits_to_int64

let part2 lines =
  lines |> List.map max_joltage_12
  |> List.fold_left Int64.add 0L
  |> Printf.printf "Part 2: %Ld\n"

let () =
  let lines = read_input () in
  part1 lines ; part2 lines
