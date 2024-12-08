import gleam/int.{parse}
import gleam/io
import gleam/result
import gleam/list.{all, count, map, window_by_2}
import gleam/string.{split}
import simplifile.{read}

pub fn main() {
  read(from: "input")
  |> result.unwrap("")
  |> split("\n")
  |> map(split(_, " "))
  |> map(map_to_int(_))
  |> count(is_safe)
  |> io.debug()
}

fn map_to_int(strings) {
  let int_results = map(strings, parse(_))
  map(int_results, result.unwrap(_, 0))
}

fn is_safe(levels) {
  let diffs = window_by_2(levels)
  |> map(fn (pair) { pair.1 - pair.0 })

  all(diffs, is_safe_increase) || all(diffs, is_safe_decrease)
}

fn is_safe_increase(diff) {
  1 <= diff && diff <= 3
}

fn is_safe_decrease(diff) {
  -3 <= diff && diff <= -1
}
