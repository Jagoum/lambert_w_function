use structopt::clap::{Error, ErrorKind};

pub const E: f64 = 2.718_281_828_46;
pub fn lambert_funtion(x: f64) -> Result<f64, Error> {
    let mut wcount;
    let mut l1;
    let mut l2;
    let mut l;
    let mut wx = 0.0;
    if x > -(1.0 / E) {
        wx = 1.0;
        wcount = 0.0;

        while (wx - wcount) > 0.000_000_001 || (wcount - wx) > 0.000_000_001 {
            wcount = wx;
            l = wx * E.powf(wx) - x;
            l1 = E.powf(wx) * (wx + 1.0);
            l2 = E.powf(wx) * (wx + 2.0);
            wx = wx - (l / (l1 - ((l * l2) / (2.0 * l1))));
        }
        return Ok(wx);
    } else {
        return Err(Error::with_description("Value out of range", ErrorKind::InvalidValue));
    }
}
