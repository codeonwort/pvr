use druid::widget::prelude::*;
use druid::widget::FillStrat;
use druid::Widget;
use druid::piet::ImageFormat;
use druid::piet::InterpolationMode;

use crate::AppState;

pub struct DruidViewport {
    pub width: u32,
    pub height: u32
}

impl DruidViewport {
    pub fn new(width: usize, height: usize) -> Self {
        DruidViewport {
            width: width as u32,
            height: height as u32
        }
    }
}

impl Widget<AppState> for DruidViewport {
    fn event(&mut self, _: &mut EventCtx, _: &Event, _: &mut AppState, _: &Env) {
        //
    }

    fn lifecycle(&mut self, _: &mut LifeCycleCtx, _: &LifeCycle, _: &AppState, _: &Env) {
        //
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old: &AppState, new: &AppState, _: &Env) {
        // #todo: Render target update flag in AppState?
        //if !old.same(new) {
            ctx.request_paint();
        //}
    }

    fn layout(&mut self, _ctx: &mut LayoutCtx, bc: &BoxConstraints, _: &AppState, _: &Env) -> Size {
        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, env: &Env) {
        let required_size = (self.width * self.height * 3) as usize;

        let render_result = (data.render_result.lock().unwrap()).clone();
        let valid_result = render_result.len() == required_size;

        let mut temp: Vec<u8> = Vec::new();
        let rawdata: &Vec<u8> = if valid_result {
            &render_result
        } else {
            temp.resize(required_size, 0);
    
            let mut ptr = 0;
            for _y in 0..self.height {
                for _x in 0..self.width {
                    let r: u8 = 0x0;
                    let g: u8 = 0xff;
                    let b: u8 = 0x0;
                    temp[ptr] = r;
                    temp[ptr+1] = g;
                    temp[ptr+2] = b;
                    ptr += 3;
                }
            }

            &temp
        };
    
        let size = Size::new(self.width as f64, self.height as f64);
        let offset_matrix = FillStrat::None.affine_to_fill(ctx.size(), size);

        ctx.with_save(|ctx| {
            ctx.transform(offset_matrix);
            let im = ctx.make_image(self.width as usize, self.height as usize, rawdata, ImageFormat::Rgb).unwrap();

            ctx.draw_image(&im, size.to_rect(), InterpolationMode::NearestNeighbor);
        })
    }
}