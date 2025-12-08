//round f64 to certain decimal places eg round_to(2.345, 2) = 2.35
mod appscript;

macro_rules! round_to {
    ($num:expr, $decimals:expr) => {
        {
            let factor = 10f64.powi($decimals);
            ($num * factor).round() / factor
        }
    };
}


// this calculates the number of sides of a polygon by the total interior angle sum
pub fn calculate_polygon_sides(n: f64) -> f64 {
    // How it works
    // (n-2)*180=i
    // In geometry
    // (n-2)*180 = i        
    //    / 180     / 180                            
    // --------------------
    //      n-2  = i
    //       +2   +2
    // --------------------
    //     n = 

    // Skip the part (n-2)*180, we don't know what n is
    // n/180 + 2 
    return (n / 180.0) + 2.0;
}


/// Finds the measure of a missing angle in a polygon given the number of sides and the known angles.
///
/// # Arguments
///
/// * `num_sides` - A `usize` representing the number of sides of the polygon.
/// * `known_angles` - A `Vec<f64>` containing the measures of the known angles of the polygon.
///
/// # Returns
///
/// * A `f64` representing the measure of the missing angle.
///
/// # Example
///
/// ```
/// let known_angles = vec![108.0, 121.0, 59.0];
/// let num_sides = 4;
/// let missing_angle = find_missing_angle(num_sides, known_angles);
/// println!("The missing angle is: {}", missing_angle); // Output: The missing angle is: 72.0
/// ```
///
/// # How it works
///
/// 1. Calculate the sum of all interior angles of the polygon using the formula `(n - 2) * 180`, where `n` is the number of sides.
/// 2. Sum up all given known angles.
/// 3. Subtract the sum of the known angles from the total interior angle sum to find the measure of the missing angle.
pub fn find_missing_angle(num_sides: usize, known_angles: Vec<f64>) -> f64 {
    // Calculate the sum of all interior angles of the polygon
    let total_sum = (num_sides as f64 - 2.0) * 180.0;
    
    // Sum up all given known angles
    let known_sum: f64 = known_angles.iter().sum();
    
    // Subtract the sum of the known angles from the total interior angle sum
    let missing_angle = total_sum - known_sum;
    
    missing_angle
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_missing_angle() {
        let known_angles = vec![108.0, 121.0, 59.0];
        let num_sides = 4;
        let missing_angle = find_missing_angle(num_sides, known_angles);
        assert_eq!(missing_angle, 72.0);
    }

    #[test]
    fn test_find_missing_angle_with_different_angles() {
        let known_angles = vec![90.0, 90.0, 90.0];
        let num_sides = 4;
        let missing_angle = find_missing_angle(num_sides, known_angles);
        assert_eq!(missing_angle, 90.0);
    }

    #[test]
    fn test_find_missing_angle_with_more_sides() {
        let known_angles = vec![120.0, 110.0, 130.0, 100.0];
        let num_sides = 5;
        let missing_angle = find_missing_angle(num_sides, known_angles);
        assert_eq!(missing_angle, 180.0);
    }

    #[test]
    fn test_calculate_polygon_sides() {
        assert_eq!(calculate_polygon_sides(180.0), 3.0); // Triangle
        assert_eq!(calculate_polygon_sides(360.0), 4.0); // Quadrilateral
        assert_eq!(calculate_polygon_sides(540.0), 5.0); // Pentagon
        assert_eq!(calculate_polygon_sides(720.0), 6.0); // Hexagon
    }
}
