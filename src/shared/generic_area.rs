use std::num::TryFromIntError;

use crate::error::Error;

/// Generic area for specialization of `Area` to a specific platform backend.
pub struct GenericArea<XY, WH> {
    pub x: XY,
    pub y: XY,
    pub width: WH,
    pub height: WH,
}

impl<XY, WH> TryFrom<Area> for GenericArea<XY, WH>
where
    XY: TryFrom<i64, Error = TryFromIntError>,
    WH: TryFrom<u64, Error = TryFromIntError>,
{
    type Error = crate::error::Error;

    fn try_from(area: Area) -> Result<Self, Self::Error> {
        Ok(GenericArea {
            x: XY::try_from(area.x).map_err(|e| Error::AreaIntConversionError(e))?,
            y: XY::try_from(area.y).map_err(|e| Error::AreaIntConversionError(e))?,
            width: WH::try_from(area.width).map_err(|e| Error::AreaIntConversionError(e))?,
            height: WH::try_from(area.height).map_err(|e| Error::AreaIntConversionError(e))?,
        })
    }
}