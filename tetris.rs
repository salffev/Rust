use ggez::conf::WindowMode;
use ggez::event::{self, MouseButton, MouseState};
use ggez::graphics::{self, Color, DrawParam, Mesh, Rect};
use ggez::{Context, GameResult};
//------------------------------------------------------------------------------------------------------------
const BOARD_WIDTH: i32 = 10;
const BOARD_HEIGHT: i32 = 20;
const CELL_SIZE: i32 = 30;
//------------------------------------------------------------------------------------------------------------
struct Tetris {
    board: [[bool; BOARD_WIDTH as usize]; BOARD_HEIGHT as usize],
    current_piece: [bool; 4 * 4],
    current_position: (i32, i32),
    score: i32,
}
//------------------------------------------------------------------------------------------------------------
impl Tetris {
    fn new() -> Self {
        Self {
            board: [[false; BOARD_WIDTH as usize]; BOARD_HEIGHT as usize],
            current_piece: [false; 4 * 4],
            current_position: (0, 0),
            score: 0,
        }
    }
//------------------------------------------------------------------------------------------------------------
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Arka planı çiz
        graphics::clear(ctx, Color::from_rgb(0, 0, 0));

        // Oyun tahtasını çiz
        for i in 0..BOARD_WIDTH {
            for j in 0..BOARD_HEIGHT {
                if self.board[j as usize][i as usize] {
                    let rect = Rect::new(
                        (i * CELL_SIZE) as f32,
                        (j * CELL_SIZE) as f32,
                        CELL_SIZE as f32,
                        CELL_SIZE as f32,
                    );
                    graphics::draw(
                        ctx,
                        &Mesh::new_rectangle(ctx, rect, Color::from_rgb(255, 255, 255))?,
                        DrawParam::default(),
                    )?;
                }
            }
        }
//------------------------------------------------------------------------------------------------------------
        // Mevcut parçayı çiz
        for i in 0..4 {
            for j in 0..4 {
                if self.current_piece[i * 4 + j] {
                    let rect = Rect::new(
                        (self.current_position.0 + i as i32) * CELL_SIZE as f32,
                        (self.current_position.1 + j as i32) * CELL_SIZE as f32,
                        CELL_SIZE as f32,
                        CELL_SIZE as f32,
                    );
                    graphics::draw(
                        ctx,
                        &Mesh::new_rectangle(ctx, rect, Color::from_rgb(0, 255, 0))?,
                        DrawParam::default(),
                    )?;
                }
            }
        }

        Ok(())
    }

    fn update(&mut self) -> GameResult {
        // Mevcut parçayı aşağı doğru hareket ettir
        self.current_position.1 += 1;

        // Parçanın tahtaya veya başka bir parçaya çarpıp çarpmadığını kontrol
        if self.is_colliding() {
            // Parçayı tahtaya sabitle
            for i in 0..4 {
                for j in 0..4 {
                    if self.current_piece[i * 4 + j] {
                        self.board[(self.current_position.1 + j as i32) as usize]
                            [(self.current_position.0 + i as i32) as usize] = true;
                    }
                }
            }

            // Tamamlanan satırları temizlenmesi
            for j in 0..BOARD_HEIGHT {
                if self.board[j as usize].iter().all(|&x| x) {
                    // Satırı temizlemesi
                    for i in 0..BOARD_WIDTH {
                        self.board[j as usize][i as usize] = false;
                    }

                    // Yukarıdaki tüm satırları bir aşağı kaydırma
                    for k
