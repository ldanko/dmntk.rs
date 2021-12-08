/*
 * DMNTK - Decision Model and Notation Toolkit
 *
 * MIT license
 *
 * Copyright (c) 2018-2021 Dariusz Depta Engos Software
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 * Apache license, Version 2.0
 *
 * Copyright (c) 2018-2021 Dariusz Depta Engos Software
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! Definitions of built-in functions.

use dmntk_common::DmntkError;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Bif {
  Abs,
  After,
  All,
  Any,
  Append,
  Before,
  Ceiling,
  Coincides,
  Concatenate,
  Contains,
  Count,
  Date,
  DateAndTime,
  DayOfWeek,
  DayOfYear,
  Decimal,
  DistinctValues,
  Duration,
  During,
  EndsWith,
  Even,
  Exp,
  FinishedBy,
  Finishes,
  Flatten,
  Floor,
  GetEntries,
  GetValue,
  Includes,
  IndexOf,
  InsertBefore,
  Is,
  ListContains,
  Log,
  LoweCase,
  Matches,
  Max,
  Mean,
  Median,
  Meets,
  MetBy,
  Min,
  Mode,
  Modulo,
  MonthOfYear,
  Not,
  Number,
  Odd,
  OverlapsAfter,
  OverlapsBefore,
  Product,
  Remove,
  Replace,
  Reverse,
  Sort,
  Split,
  Sqrt,
  StartedBy,
  Starts,
  StartsWith,
  Stddev,
  String,
  StringLength,
  Sublist,
  Substring,
  SubstringAfter,
  SubstringBefore,
  Sum,
  Time,
  Union,
  UpperCase,
  WeekOfYear,
  YearsAndMonthsDuration,
}

impl FromStr for Bif {
  type Err = DmntkError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "abs" => Ok(Self::Abs),
      "after" => Ok(Self::After),
      "all" => Ok(Self::All),
      "any" => Ok(Self::Any),
      "append" => Ok(Self::Append),
      "before" => Ok(Self::Before),
      "ceiling" => Ok(Self::Ceiling),
      "coincides" => Ok(Self::Coincides),
      "concatenate" => Ok(Self::Concatenate),
      "contains" => Ok(Self::Contains),
      "count" => Ok(Self::Count),
      "date" => Ok(Self::Date),
      "date and time" => Ok(Self::DateAndTime),
      "day of week" => Ok(Self::DayOfWeek),
      "day of year" => Ok(Self::DayOfYear),
      "decimal" => Ok(Self::Decimal),
      "distinct values" => Ok(Self::DistinctValues),
      "duration" => Ok(Self::Duration),
      "during" => Ok(Self::During),
      "ends with" => Ok(Self::EndsWith),
      "even" => Ok(Self::Even),
      "exp" => Ok(Self::Exp),
      "finished by" => Ok(Self::FinishedBy),
      "finishes" => Ok(Self::Finishes),
      "flatten" => Ok(Self::Flatten),
      "floor" => Ok(Self::Floor),
      "get entries" => Ok(Self::GetEntries),
      "get value" => Ok(Self::GetValue),
      "includes" => Ok(Self::Includes),
      "index of" => Ok(Self::IndexOf),
      "insert before" => Ok(Self::InsertBefore),
      "is" => Ok(Self::Is),
      "list contains" => Ok(Self::ListContains),
      "log" => Ok(Self::Log),
      "lower case" => Ok(Self::LoweCase),
      "matches" => Ok(Self::Matches),
      "max" => Ok(Self::Max),
      "mean" => Ok(Self::Mean),
      "median" => Ok(Self::Median),
      "meets" => Ok(Self::Meets),
      "met by" => Ok(Self::MetBy),
      "min" => Ok(Self::Min),
      "mode" => Ok(Self::Mode),
      "modulo" => Ok(Self::Modulo),
      "month of year" => Ok(Self::MonthOfYear),
      "not" => Ok(Self::Not),
      "number" => Ok(Self::Number),
      "odd" => Ok(Self::Odd),
      "overlaps after" => Ok(Self::OverlapsAfter),
      "overlaps before" => Ok(Self::OverlapsBefore),
      "product" => Ok(Self::Product),
      "remove" => Ok(Self::Remove),
      "replace" => Ok(Self::Replace),
      "reverse" => Ok(Self::Reverse),
      "sort" => Ok(Self::Sort),
      "split" => Ok(Self::Split),
      "sqrt" => Ok(Self::Sqrt),
      "started by" => Ok(Self::StartedBy),
      "starts" => Ok(Self::Starts),
      "starts with" => Ok(Self::StartsWith),
      "stddev" => Ok(Self::Stddev),
      "string" => Ok(Self::String),
      "string length" => Ok(Self::StringLength),
      "sublist" => Ok(Self::Sublist),
      "substring" => Ok(Self::Substring),
      "substring after" => Ok(Self::SubstringAfter),
      "substring before" => Ok(Self::SubstringBefore),
      "sum" => Ok(Self::Sum),
      "time" => Ok(Self::Time),
      "union" => Ok(Self::Union),
      "upper case" => Ok(Self::UpperCase),
      "week of year" => Ok(Self::WeekOfYear),
      "years and months duration" => Ok(Self::YearsAndMonthsDuration),
      _ => Err(self::errors::unknown_function_name(s)),
    }
  }
}

/// Returns `true` when the specified name is a built-in function name.
/// Built-in functions are described in details in specification: `10.3.4 Built-in functions`.
pub fn is_built_in_function_name(name: &str) -> bool {
  Bif::from_str(name).is_ok()
}

/// Returns `true` when the specified name is a built-in function name for date and time literals:
/// - date
/// - time
/// - date and time
/// - duration
pub fn is_built_in_date_time_function_name(name: &str) -> bool {
  if let Ok(built_in_function) = Bif::from_str(name) {
    matches!(built_in_function, Bif::Date | Bif::Time | Bif::DateAndTime | Bif::Duration)
  } else {
    false
  }
}

/// Definitions of errors raised in built-in functions module.
pub mod errors {
  use dmntk_common::DmntkError;

  /// Built-in functions errors.
  #[derive(Debug, PartialEq)]
  pub enum BifError {
    /// Raised when the specified name is not a built-in function.
    UnknownFunctionName(String),
  }

  impl From<BifError> for DmntkError {
    /// Converts [BifError] into [DmntkError].
    fn from(e: BifError) -> Self {
      DmntkError::new("BifError", &e.to_string())
    }
  }

  impl std::fmt::Display for BifError {
    /// Implements [Display](std::fmt::Display) trait for [BifError]s.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
        BifError::UnknownFunctionName(name) => write!(f, "unknown built-in function name: {}", name),
      }
    }
  }

  /// Creates an instance of `UnknownFunctionName` error.
  pub fn unknown_function_name(name: &str) -> DmntkError {
    BifError::UnknownFunctionName(name.to_string()).into()
  }
}

#[cfg(test)]
mod tests {
  use super::{is_built_in_date_time_function_name, is_built_in_function_name, Bif};
  use std::str::FromStr;

  #[test]
  fn test_built_in_function_from_string() {
    assert_eq!(Bif::Abs, Bif::from_str("abs").unwrap());
    assert_eq!(Bif::After, Bif::from_str("after").unwrap());
    assert_eq!(Bif::All, Bif::from_str("all").unwrap());
    assert_eq!(Bif::Any, Bif::from_str("any").unwrap());
    assert_eq!(Bif::Append, Bif::from_str("append").unwrap());
    assert_eq!(Bif::Before, Bif::from_str("before").unwrap());
    assert_eq!(Bif::Ceiling, Bif::from_str("ceiling").unwrap());
    assert_eq!(Bif::Coincides, Bif::from_str("coincides").unwrap());
    assert_eq!(Bif::Concatenate, Bif::from_str("concatenate").unwrap());
    assert_eq!(Bif::Contains, Bif::from_str("contains").unwrap());
    assert_eq!(Bif::Count, Bif::from_str("count").unwrap());
    assert_eq!(Bif::Date, Bif::from_str("date").unwrap());
    assert_eq!(Bif::DateAndTime, Bif::from_str("date and time").unwrap());
    assert_eq!(Bif::DayOfWeek, Bif::from_str("day of week").unwrap());
    assert_eq!(Bif::DayOfYear, Bif::from_str("day of year").unwrap());
    assert_eq!(Bif::Decimal, Bif::from_str("decimal").unwrap());
    assert_eq!(Bif::DistinctValues, Bif::from_str("distinct values").unwrap());
    assert_eq!(Bif::Duration, Bif::from_str("duration").unwrap());
    assert_eq!(Bif::During, Bif::from_str("during").unwrap());
    assert_eq!(Bif::EndsWith, Bif::from_str("ends with").unwrap());
    assert_eq!(Bif::Even, Bif::from_str("even").unwrap());
    assert_eq!(Bif::Exp, Bif::from_str("exp").unwrap());
    assert_eq!(Bif::FinishedBy, Bif::from_str("finished by").unwrap());
    assert_eq!(Bif::Finishes, Bif::from_str("finishes").unwrap());
    assert_eq!(Bif::Flatten, Bif::from_str("flatten").unwrap());
    assert_eq!(Bif::Floor, Bif::from_str("floor").unwrap());
    assert_eq!(Bif::GetEntries, Bif::from_str("get entries").unwrap());
    assert_eq!(Bif::GetValue, Bif::from_str("get value").unwrap());
    assert_eq!(Bif::Includes, Bif::from_str("includes").unwrap());
    assert_eq!(Bif::IndexOf, Bif::from_str("index of").unwrap());
    assert_eq!(Bif::InsertBefore, Bif::from_str("insert before").unwrap());
    assert_eq!(Bif::Is, Bif::from_str("is").unwrap());
    assert_eq!(Bif::ListContains, Bif::from_str("list contains").unwrap());
    assert_eq!(Bif::Log, Bif::from_str("log").unwrap());
    assert_eq!(Bif::LoweCase, Bif::from_str("lower case").unwrap());
    assert_eq!(Bif::Matches, Bif::from_str("matches").unwrap());
    assert_eq!(Bif::Max, Bif::from_str("max").unwrap());
    assert_eq!(Bif::Mean, Bif::from_str("mean").unwrap());
    assert_eq!(Bif::Median, Bif::from_str("median").unwrap());
    assert_eq!(Bif::Meets, Bif::from_str("meets").unwrap());
    assert_eq!(Bif::MetBy, Bif::from_str("met by").unwrap());
    assert_eq!(Bif::Min, Bif::from_str("min").unwrap());
    assert_eq!(Bif::Mode, Bif::from_str("mode").unwrap());
    assert_eq!(Bif::Modulo, Bif::from_str("modulo").unwrap());
    assert_eq!(Bif::MonthOfYear, Bif::from_str("month of year").unwrap());
    assert_eq!(Bif::Not, Bif::from_str("not").unwrap());
    assert_eq!(Bif::Number, Bif::from_str("number").unwrap());
    assert_eq!(Bif::Odd, Bif::from_str("odd").unwrap());
    assert_eq!(Bif::OverlapsAfter, Bif::from_str("overlaps after").unwrap());
    assert_eq!(Bif::OverlapsBefore, Bif::from_str("overlaps before").unwrap());
    assert_eq!(Bif::Product, Bif::from_str("product").unwrap());
    assert_eq!(Bif::Remove, Bif::from_str("remove").unwrap());
    assert_eq!(Bif::Replace, Bif::from_str("replace").unwrap());
    assert_eq!(Bif::Reverse, Bif::from_str("reverse").unwrap());
    assert_eq!(Bif::Sort, Bif::from_str("sort").unwrap());
    assert_eq!(Bif::Split, Bif::from_str("split").unwrap());
    assert_eq!(Bif::Sqrt, Bif::from_str("sqrt").unwrap());
    assert_eq!(Bif::StartedBy, Bif::from_str("started by").unwrap());
    assert_eq!(Bif::Starts, Bif::from_str("starts").unwrap());
    assert_eq!(Bif::StartsWith, Bif::from_str("starts with").unwrap());
    assert_eq!(Bif::Stddev, Bif::from_str("stddev").unwrap());
    assert_eq!(Bif::String, Bif::from_str("string").unwrap());
    assert_eq!(Bif::StringLength, Bif::from_str("string length").unwrap());
    assert_eq!(Bif::Sublist, Bif::from_str("sublist").unwrap());
    assert_eq!(Bif::Substring, Bif::from_str("substring").unwrap());
    assert_eq!(Bif::SubstringAfter, Bif::from_str("substring after").unwrap());
    assert_eq!(Bif::SubstringBefore, Bif::from_str("substring before").unwrap());
    assert_eq!(Bif::Sum, Bif::from_str("sum").unwrap());
    assert_eq!(Bif::Time, Bif::from_str("time").unwrap());
    assert_eq!(Bif::Union, Bif::from_str("union").unwrap());
    assert_eq!(Bif::WeekOfYear, Bif::from_str("week of year").unwrap());
    assert_eq!(Bif::YearsAndMonthsDuration, Bif::from_str("years and months duration").unwrap());
  }

  #[test]
  fn test_is_built_in_function_name() {
    assert!(is_built_in_function_name("abs"));
    assert!(is_built_in_function_name("after"));
    assert!(is_built_in_function_name("all"));
    assert!(is_built_in_function_name("any"));
    assert!(is_built_in_function_name("append"));
    assert!(is_built_in_function_name("before"));
    assert!(is_built_in_function_name("ceiling"));
    assert!(is_built_in_function_name("coincides"));
    assert!(is_built_in_function_name("concatenate"));
    assert!(is_built_in_function_name("contains"));
    assert!(is_built_in_function_name("count"));
    assert!(is_built_in_function_name("date"));
    assert!(is_built_in_function_name("date and time"));
    assert!(is_built_in_function_name("day of week"));
    assert!(is_built_in_function_name("day of year"));
    assert!(is_built_in_function_name("decimal"));
    assert!(is_built_in_function_name("distinct values"));
    assert!(is_built_in_function_name("duration"));
    assert!(is_built_in_function_name("during"));
    assert!(is_built_in_function_name("ends with"));
    assert!(is_built_in_function_name("even"));
    assert!(is_built_in_function_name("exp"));
    assert!(is_built_in_function_name("finished by"));
    assert!(is_built_in_function_name("finishes"));
    assert!(is_built_in_function_name("flatten"));
    assert!(is_built_in_function_name("floor"));
    assert!(is_built_in_function_name("get entries"));
    assert!(is_built_in_function_name("get value"));
    assert!(is_built_in_function_name("includes"));
    assert!(is_built_in_function_name("index of"));
    assert!(is_built_in_function_name("insert before"));
    assert!(is_built_in_function_name("is"));
    assert!(is_built_in_function_name("list contains"));
    assert!(is_built_in_function_name("log"));
    assert!(is_built_in_function_name("lower case"));
    assert!(is_built_in_function_name("matches"));
    assert!(is_built_in_function_name("max"));
    assert!(is_built_in_function_name("mean"));
    assert!(is_built_in_function_name("median"));
    assert!(is_built_in_function_name("meets"));
    assert!(is_built_in_function_name("met by"));
    assert!(is_built_in_function_name("min"));
    assert!(is_built_in_function_name("mode"));
    assert!(is_built_in_function_name("modulo"));
    assert!(is_built_in_function_name("month of year"));
    assert!(is_built_in_function_name("not"));
    assert!(is_built_in_function_name("number"));
    assert!(is_built_in_function_name("odd"));
    assert!(is_built_in_function_name("overlaps after"));
    assert!(is_built_in_function_name("overlaps before"));
    assert!(is_built_in_function_name("product"));
    assert!(is_built_in_function_name("remove"));
    assert!(is_built_in_function_name("replace"));
    assert!(is_built_in_function_name("reverse"));
    assert!(is_built_in_function_name("sort"));
    assert!(is_built_in_function_name("split"));
    assert!(is_built_in_function_name("sqrt"));
    assert!(is_built_in_function_name("started by"));
    assert!(is_built_in_function_name("starts"));
    assert!(is_built_in_function_name("starts with"));
    assert!(is_built_in_function_name("stddev"));
    assert!(is_built_in_function_name("string"));
    assert!(is_built_in_function_name("string length"));
    assert!(is_built_in_function_name("sublist"));
    assert!(is_built_in_function_name("substring"));
    assert!(is_built_in_function_name("substring after"));
    assert!(is_built_in_function_name("substring before"));
    assert!(is_built_in_function_name("sum"));
    assert!(is_built_in_function_name("time"));
    assert!(is_built_in_function_name("union"));
    assert!(is_built_in_function_name("upper case"));
    assert!(is_built_in_function_name("week of year"));
    assert!(is_built_in_function_name("years and months duration"));
  }

  #[test]
  fn test_is_built_in_date_time_function_name() {
    assert!(is_built_in_date_time_function_name("date"));
    assert!(is_built_in_date_time_function_name("time"));
    assert!(is_built_in_date_time_function_name("date and time"));
    assert!(is_built_in_date_time_function_name("duration"));
  }
}
