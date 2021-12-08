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

use crate::tests::{assert_decision, context};

lazy_static! {
  static ref DEFINITIONS: dmntk_model::model::Definitions = dmntk_model::parse(dmntk_examples::DMN_3_0056, "file: ///3_0056.dmn").unwrap();
}

#[test]
fn _0001() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "decision001", &ctx, r#"2"#);
}

#[test]
fn _0002() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "decision001_a", &ctx, r#"-2"#);
}

#[test]
fn _0003() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "decision002", &ctx, r#"2"#);
}

#[test]
fn _0004() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "decision003", &ctx, r#"0"#);
}

#[test]
fn _0005() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "decision003_a", &ctx, r#"null(division by zero)"#);
}

#[test]
fn _0006() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &DEFINITIONS,
    "decision004",
    &ctx,
    r#"null(expected 2 parameters, actual number of parameters is 0)"#,
  );
}

#[test]
fn _0007() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &DEFINITIONS,
    "decision005",
    &ctx,
    r#"null(expected 2 parameters, actual number of parameters is 1)"#,
  );
}

#[test]
fn _0008() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &DEFINITIONS,
    "decision005_a",
    &ctx,
    r#"null(expected 2 parameters, actual number of parameters is 3)"#,
  );
}

#[test]
fn _0009() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "decision006", &ctx, r#"2"#);
}

#[test]
fn _0010() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "decision007", &ctx, r#"null(parameter 'divisor' not found)"#);
}

#[test]
fn _0011() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "decision008", &ctx, r#"null(modulo)"#);
}

#[test]
fn _0012() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "decision008_a", &ctx, r#"null(modulo)"#);
}

#[test]
fn _0013() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "decision008_b", &ctx, r#"null(modulo)"#);
}

#[test]
fn _0014() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "decision009", &ctx, r#"null(modulo)"#);
}

#[test]
fn _0015() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "decision010", &ctx, r#"null(modulo)"#);
}

#[test]
fn _0016() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "decision011", &ctx, r#"null(modulo)"#);
}

#[test]
fn _0017() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "decision012", &ctx, r#"null(modulo)"#);
}

#[test]
fn _0018() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "decision013", &ctx, r#"null(modulo)"#);
}

#[test]
fn _0019() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "decision014", &ctx, r#"null(modulo)"#);
}

#[test]
fn _0020() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "decision015", &ctx, r#"null(modulo)"#);
}

#[test]
fn _0021() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "decision016a", &ctx, r#"2"#);
}

#[test]
fn _0022() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "decision016b", &ctx, r#"3"#);
}

#[test]
fn _0023() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "decision016c", &ctx, r#"-3"#);
}

#[test]
fn _0024() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "decision016d", &ctx, r#"-2"#);
}

#[test]
fn _0025() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "decision017a", &ctx, r#"1.1"#);
}

#[test]
fn _0026() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "decision017b", &ctx, r#"3.4"#);
}

#[test]
fn _0027() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "decision017c", &ctx, r#"-3.4"#);
}

#[test]
fn _0028() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "decision017d", &ctx, r#"-1.1"#);
}
