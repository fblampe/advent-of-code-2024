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
  |> count(fn (levels: List(Int)) { is_safely_increasing(levels) || is_safely_decreasing(levels) })
  |> io.debug()
}

fn map_to_int(strings: List(String)) -> List(Int) {
  let int_results = map(strings, parse(_))
  map(int_results, result.unwrap(_, 0))
}

fn is_safely_increasing(levels: List(Int)) -> Bool {
  window_by_2(levels)
  |> all(fn (pair: #(Int, Int)) -> Bool {
    let diff = pair.1 - pair.0
    diff >= 1 && diff <= 3
  })
}

fn is_safely_decreasing(levels: List(Int)) -> Bool {
window_by_2(levels)
  |> all(fn (pair: #(Int, Int)) -> Bool {
    let diff = pair.0 - pair.1
    diff >= 1 && diff <= 3
  })
}
