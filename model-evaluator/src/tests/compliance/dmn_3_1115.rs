/*
 * DMNTK - Decision Model and Notation Toolkit
 *
 * MIT license
 *
 * Copyright (c) 2018-2022 Dariusz Depta Engos Software
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
 * Copyright (c) 2018-2022 Dariusz Depta Engos Software
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
  static ref DEFINITIONS: dmntk_model::model::Definitions = dmntk_model::parse(dmntk_examples::DMN_3_1115).unwrap();
}

#[test]
fn _0001() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "feel-date-function_ErrorCase_001_e9ae035ab9", &ctx, r#"null(date_1 2: null)"#);
}

#[test]
fn _0002() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &DEFINITIONS,
    "feel-date-function_ErrorCase_002_9b9e6085ce",
    &ctx,
    r#"null(expected 1,3 parameters, actual number of parameters is 2)"#,
  );
}

#[test]
fn _0003() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "feel-date-function_ErrorCase_003_e4b7918d8f", &ctx, r#"null(date_3 4)"#);
}

#[test]
fn _0004() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "feel-date-function_ErrorCase_004_f24ed41117", &ctx, r#"null(date_3 3)"#);
}

#[test]
fn _0005() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "feel-date-function_ErrorCase_005_3540a22062", &ctx, r#"null(date_3 2)"#);
}

#[test]
fn _0006() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "feel-date-function_ErrorCase_006_616e24dbb7", &ctx, r#"null(date_3 4)"#);
}

#[test]
fn _0007() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "feel-date-function_ErrorCase_007_cda82a5d01", &ctx, r#"null(date_3 4)"#);
}

#[test]
fn _0008() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "feel-date-function_ErrorCase_008_492649d3d0", &ctx, r#"null(date_3 3)"#);
}

#[test]
fn _0009() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "feel-date-function_ErrorCase_009_9e00bbdad3", &ctx, r#"null(date_3 4)"#);
}

#[test]
fn _0010() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &DEFINITIONS,
    "feel-date-function_ErrorCase_010_6d4d58d23a",
    &ctx,
    r#"null(expected 1,3 parameters, actual number of parameters is 0)"#,
  );
}

#[test]
fn _0011() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "feel-date-function_011_5f0b42b1f8", &ctx, r#"2017-12-31"#);
}

#[test]
fn _0012() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "feel-date-function_012_d9e4b97438", &ctx, r#"2017-01-01"#);
}

#[test]
fn _0013() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "feel-date-function_013_d7e901ee86", &ctx, r#"-2017-12-31"#);
}

#[test]
fn _0014() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "feel-date-function_014_fad7e00633", &ctx, r#"-2017-01-01"#);
}

#[test]
fn _0015() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "feel-date-function_015_1dd66594cf", &ctx, r#""999999999-12-31""#);
}

#[test]
fn _0016() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "feel-date-function_016_31f3fef4a0", &ctx, r#""-999999999-12-31""#);
}

#[test]
fn _0017() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "feel-date-function_017_887dfef005", &ctx, r#"2017-08-14"#);
}

#[test]
fn _0018() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "feel-date-function_018_fc0ef0c8cb", &ctx, r#"2017-08-14"#);
}

#[test]
fn _0019() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "feel-date-function_019_b2b82796ce", &ctx, r#"2017-08-14"#);
}

#[test]
fn _0020() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "feel-date-function_020_7d56b7bf63", &ctx, r#"2017-09-03"#);
}

#[test]
fn _0021() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "feel-date-function_021_95fb3d9984", &ctx, r#"2017-09-06"#);
}

#[test]
fn _0022() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "feel-date-function_022_4063db2d59", &ctx, r#"2012-12-25"#);
}

#[test]
fn _0023() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "feel-date-function_023_4a1f604006", &ctx, r#"2017-08-03"#);
}

#[test]
fn _0024() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "feel-date-function_024_3cb98a2bb8", &ctx, r#"2017-10-11"#);
}

#[test]
fn _0025() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "feel-date-function_025_cf0ad1313c", &ctx, r#"2017-12-31"#);
}

#[test]
fn _0026() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "feel-date-function_026_cedd7e5e5f", &ctx, r#"2017-01-01"#);
}

#[test]
fn _0027() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "feel-date-function_027_987c5be372", &ctx, r#"-2017-12-31"#);
}

#[test]
fn _0028() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "feel-date-function_028_35ca79a6cd", &ctx, r#"-2017-01-01"#);
}

#[test]
fn _0029() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "feel-date-function_029_88f5c7c90f", &ctx, r#""999999999-12-31""#);
}

#[test]
fn _0030() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "feel-date-function_030_9184a7bfc3", &ctx, r#""-999999999-12-31""#);
}

#[test]
fn _0031() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "feel-date-function_ErrorCase_031_4f5ec70669", &ctx, r#"null(date_1 1)"#);
}

#[test]
fn _0032() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "feel-date-function_ErrorCase_032_fc66cc2fec", &ctx, r#"null(date_1 1)"#);
}

#[test]
fn _0033() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "feel-date-function_ErrorCase_033_c3a5600c62", &ctx, r#"null(date_1 1)"#);
}

#[test]
fn _0034() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "feel-date-function_ErrorCase_034_7d2e18a10c", &ctx, r#"null(date_1 1)"#);
}

#[test]
fn _0035() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "feel-date-function_ErrorCase_035_e6c1bb43fd", &ctx, r#"null(date_1 1)"#);
}

#[test]
fn _0036() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "feel-date-function_ErrorCase_036_b826a6b5f9", &ctx, r#"null(date_1 1)"#);
}

#[test]
fn _0037() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "feel-date-function_ErrorCase_037_cfd70896b6", &ctx, r#"null(date_1 1)"#);
}

#[test]
fn _0038() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "feel-date-function_ErrorCase_038_c26782f559", &ctx, r#"null(date_1 1)"#);
}

#[test]
fn _0039() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "feel-date-function_ErrorCase_039_67a6eafa3f", &ctx, r#"null(date_1 1)"#);
}

#[test]
fn _0040() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "feel-date-function_ErrorCase_040_dd2a2ed4a2", &ctx, r#"null(date_1 1)"#);
}

#[test]
fn _0041() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "feel-date-function_ErrorCase_041_9e7e388146", &ctx, r#"null(date_1 1)"#);
}

#[test]
fn _0042() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "feel-date-function_ErrorCase_042_8f5dd97588", &ctx, r#"null(date_3 1)"#);
}

#[test]
fn _0043() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "feel-date-function_ErrorCase_043_8f82301fac", &ctx, r#"null(date_3 1)"#);
}

#[test]
fn _0044() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "feel-date-function_ErrorCase_044_74893220b4", &ctx, r#"null(date_3 1)"#);
}

#[test]
fn _0045() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "feel-date-function_ErrorCase_045_969723fed5", &ctx, r#"null(date_3 1)"#);
}

#[test]
fn _0046() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "feel-date-function_ErrorCase_046_36bf30268a", &ctx, r#"null(date_3 1)"#);
}

#[test]
fn _0047() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "feel-date-function_ErrorCase_047_ba717eb672", &ctx, r#"null(date_3 1)"#);
}

#[test]
fn _0048() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "feel-date-function_ErrorCase_048_25595a6420", &ctx, r#"null(date_1 2: 1)"#);
}

#[test]
fn _0049() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "feel-date-function_ErrorCase_049_a1644ce710", &ctx, r#"null(date_1 2: [])"#);
}

#[test]
fn _0050() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "feel-date-function_050_8f1e299951", &ctx, r#"2012-12-25"#);
}

#[test]
fn _0051() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "feel-date-function_051_ad98079864", &ctx, r#"2017-08-30"#);
}

#[test]
fn _0052() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "feel-date-function_052_63457d78b7", &ctx, r#"2017-08-30"#);
}
