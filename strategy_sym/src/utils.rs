pub mod conv {
    use crate::defines::*;

    // given pixel position, get the tile it's in
    pub fn pixel_offset_to_grid(pix_cord: PixelOffset) -> GridTile {
        (
            (pix_cord.0 / TILE_SIZE.0) as u16,
            (pix_cord.1 / TILE_SIZE.1) as u16,
        )
    }
    //get upper left corner pixel of given grid tile
    pub fn pixel_offset_of_gridtile(gtile: GridTile) -> PixelOffset {
        (gtile.0 as f32 * TILE_SIZE.0, gtile.1 as f32 * TILE_SIZE.1)
    }

    pub fn zero_floor_sub(a: PixelOffset, b: PixelOffset) -> PixelOffset {
        let mut res = (0.0, 0.0);
        if a.0 > b.0 {
            res.0 = a.0 - b.0;
        }
        if a.1 > b.1 {
            res.1 = a.1 - b.1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_zero_floor_sub() {
        assert_eq!(conv::zero_floor_sub((5.0, 5.0), (3.0, 3.0)), (2.0, 2.0));
        assert_eq!(conv::zero_floor_sub((3.0, 3.0), (5.0, 5.0)), (0.0, 0.0));
        assert_eq!(conv::zero_floor_sub((5.0, 3.0), (3.0, 5.0)), (2.0, 0.0));
        assert_eq!(conv::zero_floor_sub((3.0, 5.0), (5.0, 3.0)), (0.0, 2.0));
    }

    #[test]
    pub fn test_pixel_offset_to_grid() {
        assert_eq!(conv::pixel_offset_to_grid((0.0, 0.0)), (0, 0));
        assert_eq!(conv::pixel_offset_to_grid((39.9, 39.9)), (0, 0));
        assert_eq!(conv::pixel_offset_to_grid((40.0, 40.0)), (1, 1));
        assert_eq!(conv::pixel_offset_to_grid((80.0, 120.0)), (2, 3));
    }
    #[test]
    pub fn test_pixel_offset_of_gridtile() {
        assert_eq!(conv::pixel_offset_of_gridtile((0, 0)), (0.0, 0.0));
        assert_eq!(conv::pixel_offset_of_gridtile((1, 1)), (40.0, 40.0));
        assert_eq!(conv::pixel_offset_of_gridtile((2, 3)), (80.0, 120.0));
    }
}