#![expect(clippy::needless_doctest_main)]
#![doc = include_str!("../README.md")]
mod path_boolean;
// #[cfg(feature = "parsing")]
mod parsing {
	pub(crate) mod path_command;
	pub(crate) mod path_data;
}
mod util {
	pub(crate) mod aabb;
	pub(crate) mod epsilons;
	pub(crate) mod grid;
	pub(crate) mod math;
	pub(crate) mod quad_tree;
}
mod path;
#[cfg(test)]
mod visual_tests;

#[cfg(feature = "parsing")]
pub(crate) use parsing::*;
pub(crate) use path::*;
pub(crate) use util::*;

pub use intersection_path_segment::path_segment_intersection;
#[cfg(feature = "parsing")]
pub use parsing::path_data::{path_from_path_data, path_to_path_data};
pub use path_boolean::{path_boolean, BooleanError, FillRule, PathBooleanOperation, EPS};
pub use path_segment::PathSegment;

#[cfg(test)]
mod test {
	use crate::path_boolean::{self, FillRule, PathBooleanOperation};
	use crate::path_data::{path_from_path_data, path_to_path_data};
	use path_boolean::path_boolean;

	#[test]
	fn square() {
		let a = path_from_path_data("M 10 10 L 50 10 L 30 40 Z").unwrap();
		let b = path_from_path_data("M 20 30 L 60 30 L 60 50 L 20 50 Z").unwrap();
		let union = path_boolean(
			&a,
			path_boolean::FillRule::NonZero,
			&b,
			path_boolean::FillRule::NonZero,
			path_boolean::PathBooleanOperation::Intersection,
		)
		.unwrap();
		dbg!(path_to_path_data(&union[0], 0.001));
		assert!(!union[0].is_empty());
	}

	#[test]
	fn nesting_01() {
		let a = path_from_path_data("M 47,24 A 23,23 0 0 1 24,47 23,23 0 0 1 1,24 23,23 0 0 1 24,1 23,23 0 0 1 47,24 Z").unwrap();
		let b = path_from_path_data(
			"M 37.909023,24 A 13.909023,13.909023 0 0 1 24,37.909023 13.909023,13.909023 0 0 1 10.090978,24 13.909023,13.909023 0 0 1 24,10.090978 13.909023,13.909023 0 0 1 37.909023,24 Z",
		)
		.unwrap();
		let union = path_boolean(&a, path_boolean::FillRule::NonZero, &b, path_boolean::FillRule::NonZero, path_boolean::PathBooleanOperation::Union).unwrap();
		dbg!(path_to_path_data(&union[0], 0.001));
		assert!(!union[0].is_empty());
	}
	#[test]
	fn nesting_02() {
		let a = path_from_path_data("M 0.99999994,31.334457 C 122.61195,71.81859 -79.025816,-5.5803326 47,32.253367 V 46.999996 H 0.99999994 Z").unwrap();
		let b = path_from_path_data("m 25.797222,29.08718 c 0,1.292706 -1.047946,2.340652 -2.340652,2.340652 -1.292707,0 -2.340652,-1.047946 -2.340652,-2.340652 0,-1.292707 1.047945,-2.340652 2.340652,-2.340652 1.292706,0 2.340652,1.047945 2.340652,2.340652 z M 7.5851073,28.332212 c 1e-7,1.292706 -1.0479456,2.340652 -2.3406521,2.340652 -1.2927063,-1e-6 -2.3406518,-1.047946 -2.3406517,-2.340652 -10e-8,-1.292707 1.0479454,-2.340652 2.3406517,-2.340652 1.2927065,-1e-6 2.3406522,1.047945 2.3406521,2.340652 z").unwrap();

		let result = path_boolean(&a, FillRule::NonZero, &b, FillRule::NonZero, PathBooleanOperation::Union).unwrap();

		// Add assertions here based on expected results
		assert_eq!(result.len(), 1, "Expected 1 resulting path for Union operation");
		// Add more specific assertions about the resulting path if needed
		assert!(!result[0].is_empty());
	}
	#[test]
	fn nesting_03() {
		let a = path_from_path_data("m 21.829117,3.5444345 h 4.341766 V 16.502158 H 21.829117 Z M 47,24 A 23,23 0 0 1 24,47 23,23 0 0 1 1,24 23,23 0 0 1 24,1 23,23 0 0 1 47,24 Z").unwrap();
		let b = path_from_path_data("M 24 6.4960938 A 17.504802 17.504802 0 0 0 6.4960938 24 A 17.504802 17.504802 0 0 0 24 41.503906 A 17.504802 17.504802 0 0 0 41.503906 24 A 17.504802 17.504802 0 0 0 24 6.4960938 z M 24 12.193359 A 11.805881 11.805881 0 0 1 35.806641 24 A 11.805881 11.805881 0 0 1 24 35.806641 A 11.805881 11.805881 0 0 1 12.193359 24 A 11.805881 11.805881 0 0 1 24 12.193359 z ").unwrap();

		let result = path_boolean(&a, FillRule::NonZero, &b, FillRule::NonZero, PathBooleanOperation::Union).unwrap();

		// Add assertions here based on expected results
		assert_eq!(result.len(), 1, "Expected 1 resulting path for Union operation");
		// Add more specific assertions about the resulting path if needed
		let path_string = dbg!(path_to_path_data(&result[0], 0.001));
		assert_eq!(path_string.chars().filter(|c| c == &'M').count(), 1, "More than one path returned");
		assert!(!result[0].is_empty());
	}
	#[test]
	fn simple_07() {
		let a = path_from_path_data("M 37.671452,24 C 52.46888,31.142429 42.887716,37.358779 24,37.671452 16.4505,37.796429 10.328548,31.550534 10.328548,24 c 0,-7.550534 6.120918,-13.671452 13.671452,-13.671452 7.550534,0 6.871598,10.389295 13.671452,13.671452 z",
	).unwrap();
		let b = path_from_path_data("M 37.671452,24 C 33.698699,53.634887 29.50935,49.018306 24,37.671452 20.7021,30.879219 10.328548,31.550534 10.328548,24 c 0,-7.550534 6.120918,-13.671452 13.671452,-13.671452 7.550534,0 14.674677,6.187863 13.671452,13.671452 z").unwrap();

		let result = path_boolean(&a, FillRule::NonZero, &b, FillRule::NonZero, PathBooleanOperation::Union).unwrap();

		// Add assertions here based on expected results
		assert_eq!(result.len(), 1, "Expected 1 resulting path for Union operation");
		// Add more specific assertions about the resulting path if needed
		dbg!(path_to_path_data(&result[0], 0.001));
		assert!(!result[0].is_empty());
	}
	#[test]
	fn rect_ellipse() {
		let a = path_from_path_data("M0,0C0,0 100,0 100,0 C100,0 100,100 100,100 C100,100 0,100 0,100 C0,100 0,0 0,0 Z").unwrap();
		let b = path_from_path_data("M50,0C77.589239,0 100,22.410761 100,50 C100,77.589239 77.589239,100 50,100 C22.410761,100 0,77.589239 0,50 C0,22.410761 22.410761,0 50,0 Z").unwrap();

		let result = path_boolean(&a, FillRule::NonZero, &b, FillRule::NonZero, PathBooleanOperation::Union).unwrap();

		// Add assertions here based on expected results
		assert_eq!(result.len(), 1, "Expected 1 resulting path for Union operation");
		dbg!(path_to_path_data(&result[0], 0.001));
		assert!(!result[0].is_empty());
		// Add more specific assertions about the resulting path if needed
	}
	#[test]
	fn red_dress_loop() {
		let a = path_from_path_data("M969.000000,0.000000C969.000000,0.000000 1110.066898,76.934393 1085.000000,181.000000 C1052.000000,318.000000 1199.180581,334.301571 1277.000000,319.000000 C1455.000000,284.000000 1586.999985,81.000000 1418.000000,0.000000 C1418.000000,0.000000 969.000000,0.000000 969.000000,0.000000").unwrap();
		let b = path_from_path_data(
			"M1211.000000,0.000000C1211.000000,0.000000 1255.000000,78.000000 1536.000000,95.000000 C1536.000000,95.000000 1536.000000,0.000000 1536.000000,0.000000 C1536.000000,0.000000 1211.000000,0.000000 1211.000000,0.000000 Z",
		).unwrap();

		let result = path_boolean(&a, FillRule::NonZero, &b, FillRule::NonZero, PathBooleanOperation::Intersection).unwrap();

		// Add assertions here based on expected results
		assert_eq!(result.len(), 1, "Expected 1 resulting path for Union operation");
		dbg!(path_to_path_data(&result[0], 0.001));
		// Add more specific assertions about the resulting path if needed
		assert!(!result[0].is_empty());
	}
	#[test]
	fn painted_dreams_1() {
		let a = path_from_path_data("M969.000000,0.000000C969.000000,0.000000 1110.066898,76.934393 1085.000000,181.000000 C1052.000000,318.000000 1199.180581,334.301571 1277.000000,319.000000 C1455.000000,284.000000 1586.999985,81.000000 1418.000000,0.000000 C1418.000000,0.000000 969.000000,0.000000 969.000000,0.000000 Z").unwrap();
		let b = path_from_path_data(
			"M763.000000,0.000000C763.000000,0.000000 1536.000000,0.000000 1536.000000,0.000000 C1536.000000,0.000000 1536.000000,254.000000 1536.000000,254.000000 C1536.000000,254.000000 1462.000000,93.000000 1271.000000,199.000000 C1149.163056,266.616314 976.413656,188.510842 908.000000,134.000000 C839.586344,79.489158 763.000000,0.000000 763.000000,0.000000 Z",
		).unwrap();

		let result = path_boolean(&a, FillRule::NonZero, &b, FillRule::NonZero, PathBooleanOperation::Intersection).unwrap();

		// Add assertions here based on expected results
		assert_eq!(result.len(), 1, "Expected 1 resulting path for Union operation");
		dbg!(path_to_path_data(&result[0], 0.001));
		// Add more specific assertions about the resulting path if needed
		assert!(!result[0].is_empty());
	}
	#[test]
	fn painted_dreams_2() {
		let a = path_from_path_data("M0,340C161.737914,383.575765 107.564182,490.730587 273,476 C419,463 481.741198,514.692273 481.333333,768 C481.333333,768 -0,768 -0,768 C-0,768 0,340 0,340 Z ")
			.unwrap();
		let b = path_from_path_data(
			"M458.370270,572.165771C428.525848,486.720093 368.618805,467.485992 273,476 C107.564178,490.730591 161.737915,383.575775 0,340 C0,340 0,689 0,689 C56,700 106.513901,779.342590 188,694.666687 C306.607422,571.416260 372.033966,552.205139 458.370270,572.165771 Z",
		).unwrap();

		let result = path_boolean(&a, FillRule::NonZero, &b, FillRule::NonZero, PathBooleanOperation::Union).unwrap();

		// Add assertions here based on expected results
		assert_eq!(result.len(), 1, "Expected 1 resulting path for Union operation");
		dbg!(path_to_path_data(&result[0], 0.001));
		// Add more specific assertions about the resulting path if needed
		assert!(!result[0].is_empty());
	}
	#[test]
	fn painted_dreams_3() {
		let a = path_from_path_data("M889,0C889,0 889,21 898,46 C909.595887,78.210796 872.365858,104.085306 869,147 C865,198 915,237 933,273 C951,309 951.703704,335.407407 923,349 C898.996281,360.366922 881,367 902,394 C923,421 928.592593,431.407407 898,468 C912.888889,472.888889 929.333333,513.333333 896,523 C896,523 876,533.333333 886,572 C896.458810,612.440732 873.333333,657.777778 802.666667,656.444444 C738.670245,655.236965 689,643 655,636 C621,629 604,623 585,666 C566,709 564,768 564,768 C564,768 0,768 0,768 C0,768 0,0 0,0 C0,0 889,0 889,0 Z ").unwrap();
		let b = path_from_path_data(
			"M552,768C552,768 993,768 993,768 C993,768 1068.918039,682.462471 1093,600 C1126,487 1007.352460,357.386071 957,324 C906.647540,290.613929 842,253 740,298 C638,343 491.342038,421.999263 491.342038,506.753005 C491.342038,641.999411 552,768 552,768 Z ",
		).unwrap();

		let result = path_boolean(&a, FillRule::NonZero, &b, FillRule::NonZero, PathBooleanOperation::Difference).unwrap();

		// Add assertions here based on expected results
		assert_eq!(result.len(), 1, "Expected 1 resulting path for Union operation");
		dbg!(path_to_path_data(&result[0], 0.001));
		// Add more specific assertions about the resulting path if needed
		assert!(!result[0].is_empty());
	}
	#[test]
	fn painted_dreams_4() {
		let a = path_from_path_data("M458.370270,572.165771C372.033966,552.205139 306.607422,571.416260 188.000000,694.666687 C106.513901,779.342590 56.000000,700.000000 0.000000,689.000000 C0.000000,689.000000 0.000000,768.000000 0.000000,768.000000 C0.000000,768.000000 481.333344,768.000000 481.333344,768.000000 C481.474091,680.589417 474.095154,617.186768 458.370270,572.165771 Z ").unwrap();
		let b = path_from_path_data(
			"M364.000000,768.000000C272.000000,686.000000 294.333333,468.666667 173.333333,506.666667 C110.156241,526.507407 0.000000,608.000000 0.000000,608.000000 L -0.000000,768.000000 L 364.000000,768.000000 Z",
		).unwrap();

		let result = path_boolean(&a, FillRule::NonZero, &b, FillRule::NonZero, PathBooleanOperation::Difference).unwrap();

		// Add assertions here based on expected results
		assert_eq!(result.len(), 1, "Expected 1 resulting path for Union operation");
		dbg!(path_to_path_data(&result[0], 0.001));
		// Add more specific assertions about the resulting path if needed
		assert!(!result[0].is_empty());
	}
	#[test]
	fn painted_dreams_5() {
		let a = path_from_path_data("M889.000000,0.000000C889.000000,0.000000 889.000000,21.000000 898.000000,46.000000 C909.595887,78.210796 872.365858,104.085306 869.000000,147.000000 C865.000000,198.000000 915.000000,237.000000 933.000000,273.000000 C951.000000,309.000000 951.703704,335.407407 923.000000,349.000000 C898.996281,360.366922 881.000000,367.000000 902.000000,394.000000 C923.000000,421.000000 928.592593,431.407407 898.000000,468.000000 C912.888889,472.888889 929.333333,513.333333 896.000000,523.000000 C896.000000,523.000000 876.000000,533.333333 886.000000,572.000000 C896.458810,612.440732 873.333333,657.777778 802.666667,656.444444 C738.670245,655.236965 689.000000,643.000000 655.000000,636.000000 C621.000000,629.000000 604.000000,623.000000 585.000000,666.000000 C566.000000,709.000000 564.000000,768.000000 564.000000,768.000000 C564.000000,768.000000 0.000000,768.000000 0.000000,768.000000 C0.000000,768.000000 0.000000,0.000000 0.000000,0.000000 C0.000000,0.000000 889.000000,0.000000 889.000000,0.000000 Z"
		).unwrap();
		let b = path_from_path_data(
			"M891.555556,569.382716C891.555556,569.382716 883.555556,577.777778 879.111111,595.851852 C874.666667,613.925926 857.185185,631.407407 830.814815,633.777778 C804.444444,636.148148 765.629630,637.925926 708.148148,616.296296 C650.666667,594.666667 560.666667,568.000000 468.000000,487.333333 C375.333333,406.666667 283.333333,354.666667 283.333333,354.666667 C332.000000,330.666667 373.407788,298.323579 468.479950,219.785706 C495.739209,197.267187 505.084065,165.580817 514.452332,146.721008 C525.711584,124.054345 577.519713,94.951389 589.958848,64.658436 C601.152263,37.399177 601.175694,0.000010 601.175694,0.000000 C601.175694,0.000000 0.000000,0.000000 0.000000,0.000000 C0.000000,0.000000 0.000000,768.000000 0.000000,768.000000 C0.000000,768.000000 891.555556,768.000000 891.555556,768.000000 C891.555556,768.000000 891.555556,569.382716 891.555556,569.382716 Z",
		).unwrap();

		let result = path_boolean(&a, FillRule::NonZero, &b, FillRule::NonZero, PathBooleanOperation::Intersection).unwrap();

		// Add assertions here based on expected results
		assert_eq!(result.len(), 1, "Expected 1 resulting path for Union operation");
		dbg!(path_to_path_data(&result[0], 0.001));
		// Add more specific assertions about the resulting path if needed
		assert!(!result[0].is_empty());
	}
	#[test]
	fn painted_dreams_6() {
		let a = path_from_path_data(
			"M 969.000000000000,0.000000000000 C 969.000000000000,0.000000000000 1110.066900000000,76.934400000000 1085.000000000000,181.000000000000 C 1052.000000000000,318.000000000000 1199.180600000000,334.301600000000 1277.000000000000,319.000000000000 C 1455.000000000000,284.000000000000 1587.000000000000,81.000000000000 1418.000000000000,0.000000000000 C 1418.000000000000,0.000000000000 969.000000000000,0.000000000000 969.000000000000,0.000000000000 L 969.000000000000,0.000000000000"
		).unwrap();
		let b = path_from_path_data(
			"M 763.000000000000,0.000000000000 C 763.000000000000,0.000000000000 1536.000000000000,0.000000000000 1536.000000000000,0.000000000000 C 1536.000000000000,0.000000000000 1536.000000000000,254.000000000000 1536.000000000000,254.000000000000 C 1536.000000000000,254.000000000000 1462.000000000000,93.000000000000 1271.000000000000,199.000000000000 C 1149.163100000000,266.616300000000 976.413700000000,188.510800000000 908.000000000000,134.000000000000 C 839.586300000000,79.489200000000 763.000000000000,0.000000000000 763.000000000000,0.000000000000 L 763.000000000000,0.000000000000",
		).unwrap();

		let result = path_boolean(&a, FillRule::NonZero, &b, FillRule::NonZero, PathBooleanOperation::Intersection).unwrap();

		// Add assertions here based on expected results
		assert_eq!(result.len(), 1, "Expected 1 resulting path for Union operation");
		dbg!(path_to_path_data(&result[0], 0.001));
		// Add more specific assertions about the resulting path if needed
		assert!(!result[0].is_empty());
	}
	#[test]
	fn painted_dreams_7() {
		let a = path_from_path_data(
			"M 989.666700000000,768.000000000000 C 989.666700000000,768.000000000000 1011.111100000000,786.399400000000 1011.111100000000,786.399400000000 C 1011.111100000000,786.399400000000 1299.306500000000,786.399400000000 1299.306500000000,786.399400000000 C 1299.306500000000,786.399400000000 1318.000000000000,768.000000000000 1318.000000000000,768.000000000000 C 1293.666700000000,681.000000000000 1173.363200000000,625.103600000000 1094.162400000000,594.296600000000 C 1094.162400000000,594.296600000000 1058.747200000000,687.805800000000 989.666700000000,768.000000000000"
		).unwrap();
		let b = path_from_path_data(
			"M 983.155000000000,775.589300000000 L 1004.599400000000,793.988700000000 L 1007.409000000000,796.399400000000 L 1011.111100000000,796.399400000000 L 1299.306500000000,796.399400000000 L 1303.402200000000,796.399400000000 L 1306.321200000000,793.526300000000 L 1325.014800000000,775.126900000000 L 1329.236900000000,770.971200000000 L 1327.630400000000,765.306400000000 C 1302.280700000000,675.920800000000 1179.503900000000,617.211200000000 1097.787500000000,584.976800000000 L 1088.418100000000,581.280900000000 L 1084.806400000000,590.765700000000 C 1084.117400000000,592.575300000000 1049.449700000000,683.516200000000 982.090100000000,761.473400000000 L 975.539200000000,769.055000000000 L 983.155000000000,775.589300000000 M 1003.696800000000,766.861600000000 C 1068.901100000000,687.878900000000 1102.806400000000,599.696700000000 1103.497000000000,597.883400000000 L 1090.537200000000,603.616300000000 C 1165.521500000000,632.344400000000 1279.846400000000,683.736400000000 1306.585700000000,765.203400000000 L 1295.210700000000,776.399400000000 L 1014.813100000000,776.399400000000 L 1003.696800000000,766.861600000000",
		).unwrap();

		let result = path_boolean(&a, FillRule::NonZero, &b, FillRule::NonZero, PathBooleanOperation::Difference).unwrap();

		// Add assertions here based on expected results
		assert_eq!(result.len(), 1, "Expected 1 resulting path for Union operation");
		dbg!(path_to_path_data(&result[0], 0.001));
		// Add more specific assertions about the resulting path if needed
		assert!(!result[0].is_empty());
	}
	#[test]
	fn blobs() {
		let a = path_from_path_data(
			"m658.03348 118.4966c7.85928 4.83645 114.84582 7.8304 127.89652 6.52531 20.97932-2.09799 43.06722-24.79623 43.06722-24.79623 0 0-96.43723-26.02101-108.97311-28.54836-20.22849-4.07832-78.95651 36.37872-61.99063 46.81928z
			m658.03348 115.88649c40.45718-30.01653 82.213-45.24662 103.10032-31.32163 7.83037 5.2203-3.58567 22.51547 13.05064 39.152 3.91519 3.9152-129.49099 2.06705-116.15096-7.83037z
			m680.87214 56.0165c2.20775-9.60391 62.6449-29.65403 101.79518-30.01652 17.61846-0.16312 119.39605 40.30737 130.50668 54.8128 5.8045 7.57806-76.88558 29.08762-91.35464 31.32162-15.28899 2.36056-144.20983-41.92525-140.94722-56.1179z"
		).unwrap();
		let b = path_from_path_data("").unwrap();

		let result = path_boolean(&a, FillRule::NonZero, &b, FillRule::NonZero, PathBooleanOperation::Union).unwrap();

		// Add assertions here based on expected results
		assert_eq!(result.len(), 1, "Expected 1 resulting path for Union operation");
		dbg!(path_to_path_data(&result[0], 0.001));
		// Add more specific assertions about the resulting path if needed
		assert!(!result[0].is_empty());
	}
	#[test]
	fn shared_line() {
		let a = path_from_path_data(
			"m 658.03348,118.4966 c 7.85928,4.83645 114.84582,7.8304 127.89652,6.52531 20.97932,-2.09799 43.06722,-24.79623 43.06722,-24.79623 0,0 -96.43723,-26.02101 -108.97311,-28.54836 -20.22849,-4.07832 -78.95651,36.37872 -61.99063,46.81928 Z"
		).unwrap();
		let b = path_from_path_data(
			"m 658.03348,115.88649 c 40.45718,-30.01653 82.213,-45.24662 103.10032,-31.32163 7.83037,5.2203 -3.58567,22.51547 13.05064,39.152 3.91519,3.9152 -129.49099,2.06705 -116.15096,-7.83037 z",
		)
		.unwrap();

		let result = path_boolean(&a, FillRule::NonZero, &b, FillRule::NonZero, PathBooleanOperation::Union).unwrap();

		// Add assertions here based on expected results
		assert_eq!(result.len(), 1, "Expected 1 resulting path for Union operation");
		dbg!(path_to_path_data(&result[0], 0.001));
		// Add more specific assertions about the resulting path if needed
		assert!(!result[0].is_empty());
	}
	#[test]
	fn real_01() {
		let a = path_from_path_data(
			"M 212.67152,105 A 64.171516,64.171516 0 0 1 148.5,169.17152 64.171516,64.171516 0 0 1 84.328484,105 64.171516,64.171516 0 0 1 148.5,40.828484 64.171516,64.171516 0 0 1 212.67152,105 Z",
		)
		.unwrap();
		let b = path_from_path_data(
			"m 83.755387,112.6962 h -7.62 v 37.0332 h 7.62 z m 22.097973,9.144 c -3.4544,0 -5.892801,1.4732 -7.620001,4.5212 v -4.064 H 91.12136 v 38.5064 h 7.111999 v -14.3256 c 1.7272,3.048 4.165601,4.4704 7.620001,4.4704 6.604,0 11.4808,-6.1976 11.4808,-14.5288 0,-8.0772 -4.3688,-14.5796 -11.4808,-14.5796 z m -1.6256,5.9436 c 3.6068,0 5.9944,3.5052 5.9944,8.7376 0,4.9784 -2.4892,8.4836 -5.9944,8.4836 -3.556,0 -5.994401,-3.4544 -5.994401,-8.5852 0,-5.1308 2.438401,-8.636 5.994401,-8.636 z m 23.62201,13.97 h -6.9596 c 0.2032,5.9944 4.6228,9.144 12.954,9.144 9.6012,0 11.9888,-5.4864 11.9888,-9.2964 0,-3.556 -1.778,-5.842 -5.3848,-6.9088 l -8.9916,-2.5908 c -1.9812,-0.6096 -2.4892,-1.016 -2.4892,-2.1336 0,-1.524 1.6256,-2.54 4.1148,-2.54 3.4036,0 5.08,1.2192 5.1308,3.7084 h 6.858 c -0.1016,-5.7912 -4.572,-9.2964 -11.938,-9.2964 -6.9596,0 -11.2776,3.5052 -11.2776,9.144 0,5.3848 4.4196,6.2484 5.9944,6.7564 l 8.4836,2.6416 c 1.778,0.5588 2.3876,1.1176 2.3876,2.2352 0,1.6764 -1.9812,2.6924 -5.2832,2.6924 -4.4704,0 -5.1816,-1.6764 -5.588,-3.556 z m 47.59959,7.9756 v -27.432 h -7.112 v 17.1704 c 0,3.2512 -2.286,5.3848 -5.7404,5.3848 -3.048,0 -4.572,-1.6256 -4.572,-4.9276 v -17.6276 h -7.112 v 19.1008 c 0,6.0452 3.3528,9.4996 9.1948,9.4996 3.7084,0 6.1976,-1.3716 8.2296,-4.4196 v 3.2512 z m 6.60404,-27.432 v 27.432 h 7.112 v -16.4592 c 0,-3.3528 1.8288,-5.3848 4.8768,-5.3848 2.3876,0 3.8608,1.3716 3.8608,3.556 v 18.288 h 7.112 v -16.4592 c 0,-3.3528 1.8288,-5.3848 4.8768,-5.3848 2.3876,0 3.8608,1.3716 3.8608,3.556 v 18.288 h 7.112 v -19.4056 c 0,-5.334 -3.2512,-8.4836 -8.7376,-8.4836 -3.4544,0 -5.8928,1.2192 -8.0264,4.064 -1.3208,-2.5908 -4.064,-4.064 -7.4676,-4.064 -3.1496,0 -5.1816,1.0668 -7.5184,3.8608 v -3.4036 z M 81.012192,49.196201 h -7.62 v 37.0332 h 25.3492 v -6.35 h -17.7292 z m 35.305978,9.144 c -8.382,0 -13.5128,5.5372 -13.5128,14.5288 0,9.0424 5.1308,14.5288 13.5636,14.5288 8.3312,0 13.5636,-5.5372 13.5636,-14.3256 0,-9.2964 -5.0292,-14.732 -13.6144,-14.732 z m 0.0508,5.7404 c 3.9116,0 6.4516,3.5052 6.4516,8.89 0,5.1308 -2.6416,8.6868 -6.4516,8.6868 -3.8608,0 -6.4516,-3.556 -6.4516,-8.7884 0,-5.2324 2.5908,-8.7884 6.4516,-8.7884 z m 18.89761,-5.2832 v 27.432 h 7.112 v -14.5796 c 0,-4.1656 2.0828,-6.2484 6.2484,-6.2484 0.762,0 1.27,0.0508 2.2352,0.2032 v -7.2136 c -0.4064,-0.0508 -0.6604,-0.0508 -0.8636,-0.0508 -3.2512,0 -6.096,2.1336 -7.62,5.842 v -5.3848 z m 31.34362,-0.4572 c -7.874,0 -12.7,5.6896 -12.7,14.8844 0,8.7884 4.7752,14.1732 12.5476,14.1732 6.14679,0 11.12519,-3.5052 12.69999,-8.89 h -7.0104 c -0.8636,2.1844 -2.8448,3.4544 -5.43559,3.4544 -4.7752,0 -5.5372,-3.5052 -5.6896,-7.2136 h 18.38959 c 0.0508,-0.6096 0.0508,-0.8636 0.0508,-1.2192 0,-12.1412 -7.366,-15.1892 -12.85239,-15.1892 z m 5.43559,11.684 H 161.1238 c 0.4572,-4.1656 2.2352,-6.2484 5.3848,-6.2484 3.30199,0 5.23239,2.2352 5.53719,6.2484 z m 12.75081,-11.2268 v 27.432 h 7.112 v -16.4592 c 0,-3.3528 1.8288,-5.3848 4.8768,-5.3848 2.3876,0 3.8608,1.3716 3.8608,3.556 v 18.288 h 7.112 v -16.4592 c 0,-3.3528 1.8288,-5.3848 4.8768,-5.3848 2.3876,0 3.8608,1.3716 3.8608,3.556 v 18.288 h 7.112 v -19.4056 c 0,-5.334 -3.2512,-8.4836 -8.7376,-8.4836 -3.4544,0 -5.8928,1.2192 -8.0264,4.064 -1.3208,-2.5908 -4.064,-4.064 -7.4676,-4.064 -3.1496,0 -5.1816,1.0668 -7.5184,3.8608 v -3.4036 z",
		)
		.unwrap();

		let result = path_boolean(&a, FillRule::NonZero, &b, FillRule::NonZero, PathBooleanOperation::Union).unwrap();

		// Add assertions here based on expected results
		assert_eq!(result.len(), 1, "Expected 1 resulting path for Union operation");
		dbg!(path_to_path_data(&result[0], 0.001));
		// Add more specific assertions about the resulting path if needed
		assert!(!result[0].is_empty());
	}
}
