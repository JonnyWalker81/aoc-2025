let read_input () =
  let rec read_lines acc =
    try
      let line = input_line stdin in
      read_lines (line :: acc)
    with End_of_file -> List.rev acc
  in
  read_lines []

type rotation = Left of int | Right of int

let rotation_of_string = function
  | s when String.length s >= 2 -> (
      let dir = String.get s 0 in
      let num_str = String.sub s 1 (String.length s - 1) in
      let num = int_of_string num_str in
      match dir with
      | 'L' ->
          Left num
      | 'R' ->
          Right num
      | _ ->
          failwith ("Invalid rotation direction: " ^ String.make 1 dir) )
  | _ ->
      failwith "Invalid rotation string"

(** Rotate function
-- Given a direction and current position, compute new position
-- between 0 and 99 (inclusive) 
its a circular range  
      **)
let rotate direction current =
  let degrees = match direction with Left d -> -d | Right d -> d in
  let new_position = (current + degrees) mod 100 in
  if new_position < 0 then new_position + 100 else new_position

let part1 lines =
  let rotations = List.map rotation_of_string lines in
  let _, rots =
    List.fold_left_map
      (fun acc r ->
        let new_pos = rotate r acc in
        (new_pos, new_pos) )
      50 rotations
  in
  List.iter (fun r -> Printf.printf "%d\n" r) rots ;
  let zero_count =
    List.fold_left (fun acc x -> if x = 0 then acc + 1 else acc) 0 rots
  in
  Printf.printf "Final position: %d\n" zero_count ;
  print_endline ""

(** Rotate function
-- Given a direction and current position, compute new position
-- between 0 and 99 (inclusive) 
its a circular range 
need to count how many times it would land on 0 not just end up on 0; 
if current is 50 and rotate is R1000 then it would cross 0 ten times
      **)
let rotate2 direction current =
  let size = 100 in
  match direction with
  | Right steps ->
      let new_pos = (current + steps) mod size in
      let first_zero = if current = 0 then size else size - current in
      let zero_count =
        if steps >= first_zero then 1 + ((steps - first_zero) / size) else 0
      in
      (new_pos, zero_count)
  | Left steps ->
      let new_pos = (((current - steps) mod size) + size) mod size in
      let first_zero = if current = 0 then size else current in
      let zero_count =
        if steps >= first_zero then 1 + ((steps - first_zero) / size) else 0
      in
      (new_pos, zero_count)

let part2 lines =
  let rotations = List.map rotation_of_string lines in
  let _, rots =
    List.fold_left_map
      (fun acc r ->
        let new_pos = rotate2 r acc in
        new_pos )
      50 rotations
  in
  List.iter (fun r -> Printf.printf "%d\n" r) rots ;
  let ones = List.fold_left (fun acc c -> acc + c) 0 rots in
  Printf.printf "Final count of zeros crossed: %d\n" ones ;
  print_endline "" ;
  print_endline "Part 2: TODO"

let () =
  let lines = read_input () in
  part1 lines ; part2 lines
