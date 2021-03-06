extern crate cgmath;

use self::cgmath::Vector2;
use self::cgmath::Matrix4;
use self::cgmath::One;
use rectangle::Rectangle;

pub trait ViewportAdapterTrait {
    fn new() -> Self;
    fn with_size(original_width: i32, original_height: i32) -> Self;
    fn with_size_and_virtual(original_width: i32, original_height: i32, virtual_width: i32, virtual_height: i32) -> Self;
    fn get_virtual_width(&self) -> i32;
    fn get_virtual_height(&self) -> i32;
    fn get_viewport_width(&self) -> i32;
    fn get_viewport_height(&self) -> i32;
    fn point_to_virtual_viewport(&self, point: Vector2<f32>) -> Vector2<f32>;
    fn screen_to_virtual_viewport(&self, point: Vector2<f32>) -> Vector2<f32>;
    fn reset(&mut self);
    fn set_viewport(&mut self, viewport: Rectangle);
    fn get_viewport(&self) -> Rectangle;
    fn get_original_viewport(&self) -> Rectangle;
    fn get_scale_matrix(&self) -> Matrix4<f32>;
}

#[derive(Debug, Copy, Clone)]
pub struct ViewportAdapter {
    viewport: Rectangle,
    original_viewport: Rectangle,
    scale_matrix: Matrix4<f32>,
}

impl ViewportAdapterTrait for ViewportAdapter {
    fn new() -> Self {
        ViewportAdapter {
            viewport: Rectangle::new(0.0, 0.0, 0, 0),
            original_viewport: Rectangle::new(0.0, 0.0, 0, 0),
            scale_matrix: Matrix4::one(),
        }
    }

    fn with_size(original_width: i32, original_height: i32) -> Self {
        ViewportAdapter {
            viewport: Rectangle::new(0.0, 0.0, original_width, original_height),
            original_viewport: Rectangle::new(0.0, 0.0, original_width, original_height),
            scale_matrix: Matrix4::one(),
        }
    }

    fn with_size_and_virtual(original_width: i32, original_height: i32, virtual_width: i32, virtual_height: i32) -> Self {
        ViewportAdapter {
            viewport: Rectangle::new(0.0, 0.0, original_width, original_height),
            original_viewport: Rectangle::new(0.0, 0.0, original_width, original_height),
            scale_matrix: Matrix4::one(),
        }
    }

    fn get_virtual_width(&self) -> i32 {
        self.original_viewport.w
    }

    fn get_virtual_height(&self) -> i32 {
        self.original_viewport.h
    }

    fn get_viewport_width(&self) -> i32 {
        self.viewport.w
    }

    fn get_viewport_height(&self) -> i32 {
        self.viewport.h
    }

    fn point_to_virtual_viewport(&self, point: Vector2<f32>) -> Vector2<f32>
    {
        return point;
    }
    
    fn screen_to_virtual_viewport(&self, point: Vector2<f32>) -> Vector2<f32>
    {
        return point;
    }
        
    fn reset(&mut self) {
    }
  
    fn set_viewport(&mut self, viewport: Rectangle) {
        self.viewport = viewport;
        self.original_viewport = viewport;
    }
    
    fn get_viewport(&self) -> Rectangle {
        self.viewport
    }

    fn get_original_viewport(&self) -> Rectangle {
        self.original_viewport
    }

    fn get_scale_matrix(&self) -> Matrix4<f32> {
        self.scale_matrix
    }

}

#[derive(Debug, Copy, Clone)]
pub struct ScalingViewportAdapter {
    base: ViewportAdapter,
    virtual_width: i32,
    virtual_height: i32,
}

impl ViewportAdapterTrait for ScalingViewportAdapter {
    fn new() -> Self {
        ScalingViewportAdapter {
            base: ViewportAdapter::new(),
            virtual_width: 0,
            virtual_height: 0,
        }
    }

    fn with_size(original_width: i32, original_height: i32) -> Self {
        ScalingViewportAdapter {
            base: ViewportAdapter::with_size(original_width, original_height),
            virtual_width: 0,
            virtual_height: 0,
        }
    }

    fn with_size_and_virtual(original_width: i32, original_height: i32, virtual_width: i32, virtual_height: i32) -> Self {
        let mut s = ScalingViewportAdapter {
            base: ViewportAdapter::with_size(original_width, original_height),
            virtual_width: virtual_width,
            virtual_height: virtual_height,
        };
        s.reset();
        s
    }

    fn get_virtual_width(&self) -> i32 {
        self.base.get_virtual_width()
    }

    fn get_virtual_height(&self) -> i32 {
        self.base.get_viewport_height()
    }

    fn get_viewport_width(&self) -> i32 {
        self.base.get_viewport_width()
    }

    fn get_viewport_height(&self) -> i32 {
        self.base.get_viewport_height()
    }

    fn point_to_virtual_viewport(&self, point: Vector2<f32>) -> Vector2<f32> {
        self.base.point_to_virtual_viewport(point)
    }
    
    fn screen_to_virtual_viewport(&self, point: Vector2<f32>) -> Vector2<f32>
    {
        self.base.screen_to_virtual_viewport(point)
    }
        
    fn reset(&mut self) {
        // Free scaling
        let old_window_size = Vector2::new(self.virtual_width, self.virtual_height);
        let new_window_size = Vector2::new(self.base.original_viewport.w, self.base.original_viewport.h);
        let ratio_x: f32 = new_window_size.x as f32 / old_window_size.x as f32;
        let ratio_y: f32 = new_window_size.y as f32 / old_window_size.y as f32;
        println!("ratio_x: {}", ratio_x);
        println!("ratio_y: {}", ratio_y);
        let original_viewport = self.get_original_viewport();
        println!("original_viewport: {:?}", original_viewport);
        self.base.viewport.x = original_viewport.x * ratio_x as f32;
        self.base.viewport.y = original_viewport.y * ratio_y as f32;
        self.base.viewport.w = (original_viewport.w as f32 * ratio_x) as i32;
        self.base.viewport.h = (original_viewport.h as f32 * ratio_y) as i32;
        let scale_x: f32 = (self.get_viewport_width() / self.virtual_width) as f32;
        let scale_y: f32 = (self.get_viewport_height() / self.virtual_height) as f32;
        println!("scale_x: {}", scale_x);
        println!("scale_y: {}", scale_y);
        // TODO: this seems to be broken
        //self.base.scale_matrix = Matrix4::from_nonuniform_scale(scale_x, scale_y, 1.0);

        // Pixel perfect scaling
        /*
        // minimum multiplier
        let multiplier = 1;
        let scaleX = newWindowSize.x / adapter->virtual_width;
        let scaleY = newWindowSize.y / adapter->virtual_height;

        // find the multiplier that fits both the new width and height
        int maxScale = (int) scaleX < (int) scaleY ? (int) scaleX : (int) scaleY;
        if (maxScale > multiplier) {
            multiplier = maxScale;
        }

        // viewport origin translation
        float diffX = (newWindowSize.x / 2.0f) - ((float) adapter->virtual_width * multiplier / 2.0f);
        float diffY = (newWindowSize.y / 2.0f) - ((float) adapter->virtual_height * multiplier / 2.0f);

        // build the new viewport
        adapter->viewport.min.x = diffX;
        adapter->viewport.min.y = diffY;
        adapter->viewport.max.x = adapter->virtual_width * multiplier;
        adapter->viewport.max.y = adapter->virtual_height * multiplier;

        // compute the scaling matrix
        float matMulX = (adapter->viewport.max.x - adapter->viewport.min.x) / adapter->virtual_width;
        float matMulY = (adapter->viewport.max.y - adapter->viewport.min.y) / adapter->virtual_height;
        kmMat4Identity(&adapter->scale_matrix);
        kmMat4 trans_matrix;
        kmMat4Identity(&trans_matrix);
        kmMat4Translation(&trans_matrix, diffX, diffY, 0.0f);
        kmMat4 sc_matrix;
        kmMat4Identity(&sc_matrix);
        kmMat4Scaling(&sc_matrix, matMulX, matMulY, 1.0f);
        kmMat4Multiply(&adapter->scale_matrix, &trans_matrix, &sc_matrix);
        */
    }
  
    fn set_viewport(&mut self, viewport: Rectangle) {
        println!("{}", "set_viewport");
        self.base.set_viewport(viewport);
    }
    
    fn get_viewport(&self) -> Rectangle {
        self.base.get_viewport()
    }

    fn get_original_viewport(&self) -> Rectangle {
        self.base.get_original_viewport()
    }

    fn get_scale_matrix(&self) -> Matrix4<f32> {
        self.base.get_scale_matrix()
    }
}

