use std::collections::HashMap;
use sdl2::pixels::Color;
use sdl2::rect::{Point,Rect};
use sdl2::render::{WindowCanvas, Texture};
use sdl2::image::{LoadTexture, InitFlag};
pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub zoom: f32,
    pub zoom_speed: f32,
    pub move_speed: f32,
}
impl Camera {
    pub fn zoom(&mut self, dir: char) { // + is zoom, - is negative zoom
        if dir == '+' {
            self.zoom += self.zoom_speed;
        }
        else if dir == '-' {
            self.zoom -= self.zoom_speed;
        }
    }
    pub fn mov(&mut self, dir: u8) { // 0 = up, 1 = left, 2 = down, 3 = right
        if dir == 0 {
            self.y -= self.move_speed;
        }
        else if dir == 1 {
            self.x -= self.move_speed;
        }

        else if dir == 2 {
            self.y += self.move_speed;
        }
        else if dir == 3 {
            self.x += self.move_speed;
        }
    }
}
pub struct TileGraphics {
    pub sc: Color,
    pub tc: Color
}

pub struct EntityGraphics {
    pub src: String,
    pub color: Color,
    pub position: Point,
    pub sprite: Rect,
}

    pub fn render(canvas: &mut WindowCanvas, texture: &Texture, position: Point, sprite: Rect) -> Result<(), String> {
        let (width, height) = canvas.output_size()?;
        let screen_rect = Rect::from_center(position, sprite.width(), sprite.height());
        canvas.copy(texture, sprite, screen_rect)?;

        
        Ok(())
    }
pub fn tile_graphics() -> HashMap<String, TileGraphics>{
return HashMap::from([
    ("grass".to_string(),
    TileGraphics {

       sc: Color::RGB(58,132,56),
       tc: Color::RGB(5,85,5)
    }),

    ("cold_land".to_string(),
    TileGraphics {

       sc: Color::RGB(58,132,56),
       tc: Color::RGB(5,85,5)
    }),
    ("water".to_string(),
    TileGraphics {
       sc: Color::RGB(65,65,195),
       tc: Color::RGB(17,17,87)
    }),

    ("ice".to_string(),
    TileGraphics {
       sc: Color::RGB(255,255,255),
       tc: Color::RGB(200,200,250)
    }),

    ("permafrost".to_string(),
    TileGraphics {
       sc: Color::RGB(58,125,50),
       tc: Color::RGB(95,110,95)
    }),

    ("coarse_land".to_string(),
    TileGraphics {
       sc: Color::RGB(150,145,105),
       tc: Color::RGB(90,85,45)
    }),
    ("savannah_land".to_string(),
    TileGraphics {
       sc: Color::RGB(186,165,80),
       tc: Color::RGB(150,150,105)
    }),

    ("sand".to_string(),
    TileGraphics {
       sc: Color::RGB(255,247,56),
       tc: Color::RGB(170,165,90)
    }),
    ("red_sand".to_string(),
    TileGraphics {
       sc: Color::RGB(190,130,80),
       tc: Color::RGB(150,90,35)
    }),
    ("mud_hive_wall".to_string(),
    TileGraphics {
       sc: Color::RGB(90,90,60),
       tc: Color::RGB(90,90,60)
    }),
    ("mud_hive_floor".to_string(),
    TileGraphics {
       sc: Color::RGB(120,120,75),
       tc: Color::RGB(120,120,75)
    }),

    ("stone_hive_wall".to_string(),
    TileGraphics {
       sc: Color::RGB(110,110,110),
       tc: Color::RGB(110,110,110)
    }),
    ("stone_hive_floor".to_string(),
    TileGraphics {
       sc: Color::RGB(50,50,50),
       tc: Color::RGB(50,50,50)
    })
]);
}