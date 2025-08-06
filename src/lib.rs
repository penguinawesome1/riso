use glam::{ Mat2, Vec2, Vec3, IVec3 };

/// Represents an isometric projection to convert between 3D world grid positions
/// and 2D screen coordinates.
pub struct IsoProjection {
    /// The 2x2 matrix for the XY part of the isometric projection.
    iso_matrix_2d: Mat2,
    
    /// The inverse of the 2x2 matrix.
    inv_iso_matrix_2d: Mat2,
    
    /// Scalar for Z-axis scaling (world_z to screen_z).
    z_scale: f32,
    
    /// Inverse scalar for Z-axis scaling (screen_z to world_z).
    inv_z_scale: f32,
}

impl IsoProjection {
    /// Create a new projection struct to convert between world and screen.
    ///
    /// # Type Parameters
    ///
    /// * `HALF_TW`: Half the width of an isometric tile in screen pixels.
    /// * `HALF_TH`: Half the height of an isometric tile in screen pixels.
    ///
    /// # Examples
    ///
    /// ```
    /// use spriso::IsoProjection;
    ///
    /// let proj: IsoProjection = IsoProjection::new::<14, 14>();
    /// ```
    pub fn new<const HALF_TW: u32, const HALF_TH: u32>() -> Self {
        let iso_matrix_2d = Mat2::from_cols(
            Vec2::new(1.0 * (HALF_TW as f32), 0.5 * (HALF_TH as f32)),
            Vec2::new(-1.0 * (HALF_TW as f32), 0.5 * (HALF_TH as f32))
        );
        let inv_iso_matrix_2d = iso_matrix_2d.inverse();
        let z_scale: f32 = HALF_TH as f32;

        Self {
            iso_matrix_2d,
            inv_iso_matrix_2d,
            z_scale,
            inv_z_scale: 1.0 / z_scale,
        }
    }

    /// Converts 3d grid positions to their corresponding screen position.
    ///
    /// # Examples
    ///
    /// ```
    /// use glam::{ IVec3, Vec3 };
    /// use spriso::IsoProjection;
    ///
    /// let proj: IsoProjection = IsoProjection::new::<14, 14>();
    ///
    /// let pos: IVec3 = IVec3::new(10, 20, 30);
    /// let screen_pos: Vec3 = proj.world_to_screen(pos);
    ///
    /// assert!(screen_pos.x != 0.0);
    /// assert!(screen_pos.y != 0.0);
    /// assert!(screen_pos.z != 0.0);
    /// ```
    pub fn world_to_screen(&self, world_pos: IVec3) -> Vec3 {
        let world_vec_2d: Vec2 = Vec2::new(world_pos.x as f32, world_pos.y as f32);
        let screen_vec_2d: Vec2 = self.iso_matrix_2d * world_vec_2d;

        Vec3::new(screen_vec_2d.x, screen_vec_2d.y, (world_pos.z as f32) * self.z_scale)
    }

    /// Converts screen positions to their corresponding 3d grid positions.
    pub fn screen_to_world(&self, screen_pos: Vec3) -> IVec3 {
        let screen_vec_2d: Vec2 = Vec2::new(screen_pos.x, screen_pos.y);
        let world_vec_2d: Vec2 = self.inv_iso_matrix_2d * screen_vec_2d;

        IVec3::new(
            world_vec_2d.x.round() as i32,
            world_vec_2d.y.round() as i32,
            (screen_pos.z * self.inv_z_scale).round() as i32
        )
    }

    /// Converts 3d grid float position to its corresponding screen position.
    pub fn world_float_to_screen(&self, world_float_pos: Vec3) -> Vec3 {
        let world_vec_2d: Vec2 = Vec2::new(world_float_pos.x, world_float_pos.y);
        let screen_vec_2d: Vec2 = self.iso_matrix_2d * world_vec_2d;

        Vec3::new(screen_vec_2d.x, screen_vec_2d.y, world_float_pos.z * self.z_scale)
    }
}
