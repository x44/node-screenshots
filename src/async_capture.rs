use napi::{
    bindgen_prelude::{Error, Result},
    Env, Task,
};
use xcap::{image::RgbaImage, Monitor as XCapMonitor, Window as XCapWindow};

use crate::Image;

#[derive(Debug, Clone)]
pub enum AsyncCapture {
    Monitor(XCapMonitor),
	MonitorRegion(XCapMonitor, u32, u32, u32, u32),
    Window(XCapWindow),
}

unsafe impl Send for AsyncCapture {}

#[napi]
impl Task for AsyncCapture {
    type Output = RgbaImage;
    type JsValue = Image;

    fn compute(&mut self) -> Result<Self::Output> {
        let capture_image = match self {
            AsyncCapture::Monitor(x_cap_monitor) => x_cap_monitor.capture_image(),
            AsyncCapture::MonitorRegion(x_cap_monitor, x, y, w, h) => x_cap_monitor.capture_region(*x, *y, *w, *h),
            AsyncCapture::Window(x_cap_window) => x_cap_window.capture_image(),
        };

        capture_image.map_err(|err| Error::from_reason(err.to_string()))
    }

    fn resolve(&mut self, _: Env, output: Self::Output) -> Result<Self::JsValue> {
        Ok(Image::from(output))
    }
}
