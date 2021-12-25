use std::collections::HashMap;
use sdl2::pixels::Color;
use sdl2::rect::{Point,Rect};
use sdl2::render::{WindowCanvas, Texture, TextureCreator, BlendMode};
use sdl2::image::{LoadTexture, InitFlag};
use sdl2::ttf::Font;
use sdl2::surface::{Surface};
use crate::world_structs;
#[derive(PartialEq)]
pub enum MapState {
    NORMAL,
    POLITICAL
}
#[derive(PartialEq)]
pub enum MoveDirection {
    UP,
    LEFT,
    DOWN,
    RIGHT,
    ZOOMIN,
    ZOOMOUT
}
#[derive(PartialEq)]
#[derive(Clone)]
pub enum ButtonStatus {
    NEUTRAL,
    HOVERED,
    PRESSED,
    RELEASED
}
pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub zoom: f32,
    pub zoom_speed: f32,
    pub move_speed: f32,
}
impl Camera {
    pub fn zoom(&mut self, dir: MoveDirection, delta: u128) { 
        if dir == MoveDirection::ZOOMIN {
            self.zoom += self.zoom_speed * delta as f32 / 5.0;
        }
        else if dir == MoveDirection::ZOOMOUT {
            self.zoom -= self.zoom_speed * delta as f32 / 5.0;
        }
    }
    pub fn mov(&mut self, dir: MoveDirection, delta: u128) { 
        if dir == MoveDirection::UP {
            self.y -= self.move_speed * delta as f32 / 5.0;
        }
        else if dir == MoveDirection::LEFT {
            self.x -= self.move_speed * delta as f32 / 5.0;
        }

        else if dir == MoveDirection::DOWN {
            self.y += self.move_speed * delta as f32 / 5.0;
        }
        else if dir == MoveDirection::RIGHT {
            self.x += self.move_speed * delta as f32 / 5.0;
        }
    }
}

pub struct Button {
    pub status: ButtonStatus,
    pub previous_status: ButtonStatus,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32
}
impl Button {
    pub fn hover(&mut self) {
        self.status = ButtonStatus::HOVERED;
    
    }
    pub fn press(&mut self) {
        self.status = ButtonStatus::PRESSED;
    
    }
    pub fn release(&mut self) {
        self.status = ButtonStatus::RELEASED;
    
    }
    pub fn neutralize(&mut self) {
        self.status = ButtonStatus::NEUTRAL;
    
    }
    pub fn check_if_hovered(&mut self, m_x: f32, m_y: f32) {
        if m_x > self.x && m_x < self.x + self.width && m_y > self.y && m_y < self.y + self.height {
                self.hover();
        }
        else {
            self.neutralize();
        }
    } 
    pub fn check_if_pressed(&mut self, m_x: f32, m_y: f32, pressed: bool) {

        if pressed && self.status == ButtonStatus::HOVERED {
            self.press();
        }
    
        else if !pressed && self.previous_status == ButtonStatus::PRESSED {
            self.release();
        }
        self.previous_status = self.status.clone();
    
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

    pub fn render(canvas: &mut WindowCanvas, texture: &Texture, position: Point, sprite: Rect, zoom: f32) -> Result<(), String> {
        let (width, height) = canvas.output_size()?;
        let screen_rect = Rect::new(position.x as i32, position.y as i32,(sprite.width() as f32 * zoom) as u32, (sprite.height() as f32 * zoom) as u32);
        canvas.copy(texture, sprite, screen_rect)?;
        Ok(())
    }

    pub fn render_with_color(canvas: &mut WindowCanvas, texture: &Texture, position: Point, sprite: Rect, color: Color, zoom: f32) -> Result<(), String> {
        let (width, height) = canvas.output_size()?;
        let screen_rect = Rect::new(position.x,position.y, (sprite.width() as f32 * zoom) as u32, (sprite.height() as f32 * zoom) as u32);
        canvas.copy(texture, sprite, screen_rect)?;
        canvas.set_draw_color(color);
        match canvas.fill_rect(Rect::new(position.x as i32,position.y as i32,(sprite.width() as f32 * zoom) as u32,(sprite.height() as f32 * zoom) as u32)) {
                    Ok(_v) => (),
                    Err(_v) => (),
                }
        Ok(())
    }
    pub fn render_rect(canvas: &mut WindowCanvas, position: Point, sprite: Rect, color: Color, zoom: f32) -> Result<(), String> {
        let (width, height) = canvas.output_size()?;
        let screen_rect = Rect::new(position.x,position.y, (sprite.width() as f32 * zoom) as u32, (sprite.height() as f32 * zoom) as u32);
        canvas.set_draw_color(color);
        match canvas.fill_rect(Rect::new(position.x as i32,position.y as i32,(sprite.width() as f32 * zoom) as u32,(sprite.height() as f32 * zoom) as u32)) {
                    Ok(_v) => (),
                    Err(_v) => (),
                }
        Ok(())
    }
pub fn render_text(canvas: &mut WindowCanvas, texture: &Texture, position: Point, sprite: Rect) {

    let screen_rect = Rect::new(position.x,position.y, sprite.width(), sprite.height());
    canvas.copy(texture, None, screen_rect);
}
pub fn tile_graphics() -> HashMap<world_structs::TileType, TileGraphics>{
return HashMap::from([
    (world_structs::TileType::GRASS ,
    TileGraphics {

       sc: Color::RGB(58,132,56),
       tc: Color::RGB(5,85,5)
    }),

    (world_structs::TileType::COLD_LAND ,
    TileGraphics {

       sc: Color::RGB(58,132,56),
       tc: Color::RGB(5,85,5)
    }),
    (world_structs::TileType::WATER ,
    TileGraphics {
       sc: Color::RGB(65,65,195),
       tc: Color::RGB(17,17,87)
    }),

    (world_structs::TileType::ICE ,
    TileGraphics {
       sc: Color::RGB(255,255,255),
       tc: Color::RGB(200,200,250)
    }),

    (world_structs::TileType::PERMAFROST,
    TileGraphics {
       sc: Color::RGB(58,125,50),
       tc: Color::RGB(95,110,95)
    }),

    (world_structs::TileType::COARSE_LAND,
    TileGraphics {
       sc: Color::RGB(150,145,105),
       tc: Color::RGB(90,85,45)
    }),
    (world_structs::TileType::SAVANNAH_LAND,
    TileGraphics {
       sc: Color::RGB(186,165,80),
       tc: Color::RGB(150,150,105)
    }),

    (world_structs::TileType::SAND,
    TileGraphics {
       sc: Color::RGB(255,247,56),
       tc: Color::RGB(170,165,90)
    }),
    (world_structs::TileType::RED_SAND,
    TileGraphics {
       sc: Color::RGB(190,130,80),
       tc: Color::RGB(150,90,35)
    }),
    (world_structs::TileType::MUD_HIVE_WALL,
    TileGraphics {
       sc: Color::RGB(90,90,60),
       tc: Color::RGB(90,90,60)
    }),
    (world_structs::TileType::MUD_HIVE_FLOOR,
    TileGraphics {
       sc: Color::RGB(120,120,75),
       tc: Color::RGB(120,120,75)
    }),

    (world_structs::TileType::STONE_HIVE_WALL,
    TileGraphics {
       sc: Color::RGB(110,110,110),
       tc: Color::RGB(110,110,110)
    }),
    (world_structs::TileType::STONE_HIVE_FLOOR,
    TileGraphics {
       sc: Color::RGB(50,50,50),
       tc: Color::RGB(50,50,50)
    })
]);
}

pub struct Text<'a> {
    pub text_surface: Surface<'a>,
    pub text_texture: Texture<'a>,
    pub text_sprite: Rect
}


pub fn get_text<'a, T>(msg: String, color: Color, font_size: u16, font: &Font,texture_creator: &'a TextureCreator<T>) -> Option<Text<'a>> {

    let text_surface = font
        .render(&msg)
        .blended(color)
        .map_err(|e| e.to_string()).ok()?;
    let text_texture = texture_creator
        .create_texture_from_surface(&text_surface)
        .map_err(|e| e.to_string()).ok()?;
    let text_sprite = Rect::new(0,0,(font_size as f32 / 2.0) as u32 * msg.len() as u32, (font_size as f32  ) as u32);

    let text = Text{text_surface: text_surface, text_texture: text_texture, text_sprite: text_sprite};
    return Some(text);
}
pub fn get_descriptions_for_entities()-> HashMap<world_structs::EntityType, String> {

    let entity_descriptions = HashMap::from([
        (world_structs::EntityType::OAK,
         "Oak".to_string()
         ),
        (world_structs::EntityType::BIRCH,
         "Birch".to_string()
         ),
        (world_structs::EntityType::APPLETREE,
         "Apple tree".to_string()
         ),
        (world_structs::EntityType::PINE,
         "Pine".to_string()
         ),
        (world_structs::EntityType::SPRUCE,
         "Spruce".to_string()
         ),
        (world_structs::EntityType::CACTUS,
         "Cactus".to_string()
         ),
        (world_structs::EntityType::WORKER_ANT,
         "ant worker".to_string()
         ),
        (world_structs::EntityType::SNAIL,
         "Snail".to_string()
         ),
    ]);
    return entity_descriptions;
}
pub fn get_descriptions_for_tiles() -> HashMap<world_structs::TileType, String> {
    
    let tile_descriptions = HashMap::from([
        (world_structs::TileType::GRASS,
         "Grass".to_string()
         ),
        (world_structs::TileType::COLD_LAND,
         "Grass".to_string()
         ),
        (world_structs::TileType::ICE,
         "Ice".to_string()
         ),
        (world_structs::TileType::WATER,
         "Water".to_string()
         ),
        (world_structs::TileType::COARSE_LAND,
         "Coarse grass".to_string()
         ),
        (world_structs::TileType::SAVANNAH_LAND,
         "Savannah grass".to_string()
         ),
        (world_structs::TileType::SAND,
         "Sand".to_string()
         ),
        (world_structs::TileType::RED_SAND,
         "Red sand".to_string()
         ),
        (world_structs::TileType::PERMAFROST,
         "Frozen ground".to_string()
         ),

        (world_structs::TileType::MUD_HIVE_WALL,
         "Mud wall".to_string()
         ),
        (world_structs::TileType::MUD_HIVE_FLOOR,
         "Mud floor".to_string()
         ),
    ]);
    return tile_descriptions;
}
